//! Raptor Language Server (tower-lsp).
//!
//! Analysis is delegated to the self-hosted `lsp-backend` binary, which runs the
//! Raptor frontend up to the semantic checker and prints a JSON document on
//! stdout (see `tools/lsp-backend.rp`).
//!
//! Configure the backend path with `RAPTOR_LSP_BACKEND`. If unset, the server
//! looks for `lsp-backend` on `PATH`, then `./build/lsp-backend`.

use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::process::Command;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::OnceLock;

/// Optional absolute path from `--backend` (set once in main before the server starts).
static BACKEND_PATH: OnceLock<PathBuf> = OnceLock::new();

/// Set when the client sent Windows drive URIs (`file:///c:/...`) while we run in WSL.
/// Then every path we send back must be converted from `/mnt/c/...` to `c:/...`.
static WSL_DRIVE_URIS: AtomicBool = AtomicBool::new(false);

use serde::Deserialize;
use tempfile::NamedTempFile;
use tokio::sync::{watch, Mutex, Semaphore};
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

// ---------------------------------------------------------------------------
// JSON payload from tools/lsp-backend.rp
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
struct BackendSpan {
    file: String,
    start_line: i64,
    start_column: i64,
    end_line: i64,
    end_column: i64,
    #[serde(default)]
    start_offset: i64,
    #[serde(default)]
    end_offset: i64,
}

#[derive(Debug, Clone, Deserialize)]
struct BackendDiagnostic {
    severity: String,
    message: String,
    #[serde(default)]
    phase: String,
    span: BackendSpan,
}

#[derive(Debug, Clone, Deserialize)]
struct BackendHover {
    span: BackendSpan,
    contents: String,
}

#[derive(Debug, Clone, Deserialize)]
struct BackendDefinition {
    #[serde(rename = "use")]
    use_span: BackendSpan,
    def: BackendSpan,
}

#[derive(Debug, Clone, Deserialize)]
struct BackendPayload {
    #[serde(default)]
    ok: bool,
    #[serde(default)]
    diagnostics: Vec<BackendDiagnostic>,
    #[serde(default)]
    definitions: Vec<BackendDefinition>,
    #[serde(default)]
    hovers: Vec<BackendHover>,
}

#[derive(Debug, Clone)]
struct HoverInfo {
    span: BackendSpan,
    contents: String,
}

#[derive(Debug, Clone)]
struct DefinitionInfo {
    use_span: BackendSpan,
    def_span: BackendSpan,
}

struct DocumentState {
    text: String,
    /// Hash of the text as last seen on disk (open / save). A save with the same hash
    /// changed nothing on disk, so nothing else needs to be re-analyzed.
    saved_hash: Option<u64>,
    analyzed_path: String,
    hovers: Vec<HoverInfo>,
    definitions: Vec<DefinitionInfo>,
}

/// Reverse import graph: file -> files that import it (built by scanning `.rp` sources).
struct ImportGraph {
    built_at: Option<Instant>,
    roots: Vec<String>,
    importers: Arc<HashMap<String, Vec<String>>>,
}

struct Backend {
    client: Client,
    documents: Mutex<HashMap<Url, DocumentState>>,
    /// Newest analysis request id per document; older results are discarded. It is a watch
    /// channel so that a newer edit also *wakes* the older, still running analysis, which then
    /// kills its backend process instead of burning CPU for a result nobody will read.
    latest: Mutex<HashMap<Url, watch::Sender<u64>>>,
    seq: AtomicU64,
    /// Caps concurrently running backend processes (each one is a full compiler frontend run).
    limiter: Semaphore,
    /// Duration of the last finished analysis (ms); drives the adaptive debounce.
    last_analysis_ms: AtomicU64,
    /// Open file -> entrypoint (an importer, higher in the import tree) that analyzed it best.
    context_cache: Mutex<HashMap<String, String>>,
    graph: Mutex<ImportGraph>,
    workspace_roots: Mutex<Vec<String>>,
}

/// Owns the shared server state. Analyses run in spawned tasks that need `Arc<Backend>`,
/// so the LSP handlers must return quickly and never sit on a request slot for seconds.
struct Handle(Arc<Backend>);

impl Deref for Handle {
    type Target = Backend;
    fn deref(&self) -> &Backend {
        &self.0
    }
}

fn hash_text(text: &str) -> u64 {
    let mut h = DefaultHasher::new();
    text.hash(&mut h);
    h.finish()
}

fn fix_wsl_drive_prefix(path: String) -> String {
    #[cfg(not(windows))]
    {
        let bytes = path.as_bytes();
        let looks_like_drive_letter = bytes.len() >= 4
            && bytes[0] == b'/'
            && bytes[1].is_ascii_alphabetic()
            && bytes[2] == b':'
            && (bytes[3] == b'/' || bytes[3] == b'\\');
        if looks_like_drive_letter {
            WSL_DRIVE_URIS.store(true, Ordering::Relaxed);
            let drive = (bytes[1] as char).to_ascii_lowercase();
            return format!("/mnt/{}{}", drive, &path[3..]);
        }
    }
    path
}

fn path_for_uri(uri: &Url) -> String {
    let raw = uri
        .to_file_path()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| uri.path().to_string());
    fix_wsl_drive_prefix(raw)
}

fn uri_for_path(path: &str) -> Option<Url> {
    // Inverse of `fix_wsl_drive_prefix`: `/mnt/c/Users/x` -> `file:///c:/Users/x`,
    // otherwise VS Code (Windows) would try to open `\mnt\c\Users\x`.
    #[cfg(not(windows))]
    {
        if WSL_DRIVE_URIS.load(Ordering::Relaxed) {
            if let Some(rest) = path.strip_prefix("/mnt/") {
                let mut it = rest.splitn(2, '/');
                let drive = it.next().unwrap_or("");
                let tail = it.next().unwrap_or("");
                if drive.len() == 1 && drive.chars().all(|c| c.is_ascii_alphabetic()) {
                    let d = drive.to_ascii_lowercase();
                    let mut url = Url::parse(&format!("file:///{d}:/")).ok()?;
                    url.set_path(&format!("/{d}:/{tail}"));
                    return Some(url);
                }
            }
        }
    }
    Url::from_file_path(path).ok()
}

fn backend_binary() -> PathBuf {
    // 1) Explicit path from CLI (--backend), stored at startup
    if let Some(p) = BACKEND_PATH.get() {
        return p.clone();
    }
    // 2) Environment (works when not going through broken WSL env forwarding)
    if let Ok(p) = std::env::var("RAPTOR_LSP_BACKEND") {
        return PathBuf::from(p);
    }
    if let Ok(path) = which("lsp-backend") {
        return path;
    }
    // 3) Search near the server binary and parent build/ dirs
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("lsp-backend"));
            candidates.push(dir.join("../../../build/lsp-backend"));
            candidates.push(dir.join("../../../../build/lsp-backend"));
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd.join("build/lsp-backend"));
        candidates.push(cwd.join("lsp-backend"));
        let mut parent = cwd;
        for _ in 0..8 {
            candidates.push(parent.join("build/lsp-backend"));
            if !parent.pop() {
                break;
            }
        }
    }
    for c in &candidates {
        if c.exists() {
            return c.clone();
        }
    }
    PathBuf::from("./build/lsp-backend")
}

fn which(name: &str) -> Result<PathBuf, ()> {
    let path_var = std::env::var_os("PATH").ok_or(())?;
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Ok(candidate);
        }
        #[cfg(windows)]
        {
            let with_exe = dir.join(format!("{name}.exe"));
            if with_exe.is_file() {
                return Ok(with_exe);
            }
        }
    }
    Err(())
}

fn span_to_range(span: &BackendSpan) -> Range {
    Range::new(
        Position::new(
            span.start_line.saturating_sub(1) as u32,
            span.start_column.saturating_sub(1) as u32,
        ),
        Position::new(
            span.end_line.saturating_sub(1) as u32,
            span.end_column.saturating_sub(1) as u32,
        ),
    )
}

fn span_contains_position(span: &BackendSpan, pos: Position) -> bool {
    let range = span_to_range(span);
    (pos.line > range.start.line
        || (pos.line == range.start.line && pos.character >= range.start.character))
        && (pos.line < range.end.line
            || (pos.line == range.end.line && pos.character <= range.end.character))
}

fn span_len(span: &BackendSpan) -> (u32, u32) {
    (
        (span.end_line.saturating_sub(span.start_line)) as u32,
        (span.end_column.saturating_sub(span.start_column)) as u32,
    )
}

fn severity_from_str(s: &str) -> DiagnosticSeverity {
    match s {
        "error" => DiagnosticSeverity::ERROR,
        "warning" => DiagnosticSeverity::WARNING,
        _ => DiagnosticSeverity::INFORMATION,
    }
}

/// Debounce before spawning the backend (ms) and hard limit for one analysis (s).
/// The debounce adapts: it is a third of the last analysis time, between these bounds, so a
/// 3 s analysis of a big program is not restarted on every short pause between keystrokes.
const DEBOUNCE_MS: u64 = 150;
const MAX_DEBOUNCE_MS: u64 = 1000;
const BACKEND_TIMEOUT_SECS: u64 = 15;
/// Max backend runs per analysis: the file itself + this many minus one importers.
const MAX_CONTEXT_ATTEMPTS: usize = 5;

const SUPERSEDED: &str = "superseded by a newer edit";

/// Completes once request `seq` is no longer the newest one for its document (or the document
/// was closed).
async fn superseded(rx: &mut watch::Receiver<u64>, seq: u64) {
    loop {
        if *rx.borrow_and_update() != seq {
            return;
        }
        if rx.changed().await.is_err() {
            return;
        }
    }
}

/// Runs the backend with `entry` as entrypoint. `overlays` are (path, text) of every open
/// editor buffer: the backend reads those instead of the (possibly older) files on disk.
/// `focus` is the open file: the backend then only emits hovers / definitions located in it
/// (for a big program that is a 10-30x smaller JSON). The run is killed as soon as a newer
/// edit of the same document arrives. Returns the payload and everything the backend wrote to
/// stderr (your `eprintln`s).
async fn run_backend(
    entry: &str,
    focus: &str,
    overlays: &[(String, String)],
    limiter: &Semaphore,
    mut cancel: watch::Receiver<u64>,
    seq: u64,
) -> Result<(BackendPayload, String), String> {
    let bin = backend_binary();
    if !bin.exists() {
        return Err(format!(
            "lsp-backend not found at '{}' (set RAPTOR_LSP_BACKEND)",
            bin.display()
        ));
    }

    let work = async {
        let _permit = limiter.acquire().await.map_err(|e| e.to_string())?;

        let mut cmd = Command::new(&bin);
        let mut tmp_files: Vec<NamedTempFile> = Vec::new();
        for (path, text) in overlays {
            let mut tmp = NamedTempFile::with_suffix(".rp").map_err(|e| e.to_string())?;
            tmp.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
            tmp.flush().map_err(|e| e.to_string())?;
            cmd.arg("--overlay").arg(path).arg(tmp.path());
            tmp_files.push(tmp);
        }
        cmd.arg("--focus").arg(focus);
        // kill_on_drop + timeout: a hanging backend can no longer block a file forever.
        cmd.arg(entry).kill_on_drop(true);

        let output = match tokio::time::timeout(
            Duration::from_secs(BACKEND_TIMEOUT_SECS),
            cmd.output(),
        )
        .await
        {
            Ok(r) => r.map_err(|e| format!("failed to spawn {}: {e}", bin.display()))?,
            Err(_) => {
                return Err(format!(
                    "lsp-backend timed out after {BACKEND_TIMEOUT_SECS}s and was killed"
                ))
            }
        };
        drop(tmp_files);
        Ok::<_, String>(output)
    };

    // Dropping `work` here drops the child process future, which kills the backend.
    let output = tokio::select! {
        r = work => r?,
        _ = superseded(&mut cancel, seq) => return Err(SUPERSEDED.to_string()),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let json_line = stdout
        .lines()
        .rev()
        .find(|l| {
            let t = l.trim();
            t.starts_with('{') && t.ends_with('}')
        })
        .unwrap_or("")
        .trim();

    if json_line.is_empty() {
        return Err(format!(
            "lsp-backend produced no JSON ({}). stderr:\n{stderr}\nstdout:\n{stdout}",
            output.status
        ));
    }

    let payload: BackendPayload = serde_json::from_str(json_line)
        .map_err(|e| format!("invalid backend JSON: {e}\n{json_line}"))?;
    Ok((payload, stderr))
}

// ---------------------------------------------------------------------------
// Paths and the reverse import graph (used to find a better entrypoint)
// ---------------------------------------------------------------------------

/// Lexical normalization, the same rules as `import_resolver_normalize_path` in the backend.
fn normalize_path(path: &str) -> String {
    let is_abs = path.starts_with('/');
    let mut stack: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                stack.pop();
            }
            p => stack.push(p),
        }
    }
    let joined = stack.join("/");
    if is_abs {
        format!("/{joined}")
    } else if joined.is_empty() {
        ".".to_string()
    } else {
        joined
    }
}

fn parent_dir(path: &str) -> String {
    match path.rfind('/') {
        Some(0) => "/".to_string(),
        Some(idx) => path[..idx].to_string(),
        None => ".".to_string(),
    }
}

fn resolve_import(from_file: &str, import: &str) -> String {
    if import.starts_with('/') {
        return normalize_path(import);
    }
    let dir = parent_dir(from_file);
    normalize_path(&format!("{dir}/{import}"))
}

/// Finds `import "x.rp";` as well as the future `import { a, b } from "x.rp";`
/// (the first quoted string on a line that starts with `import`).
fn parse_imports(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in source.lines() {
        let t = line.trim_start();
        let Some(rest) = t.strip_prefix("import") else {
            continue;
        };
        if !rest.starts_with(|c: char| c.is_whitespace() || c == '{' || c == '"') {
            continue;
        }
        let Some(start) = rest.find('"') else {
            continue;
        };
        let after = &rest[start + 1..];
        let Some(end) = after.find('"') else {
            continue;
        };
        out.push(after[..end].to_string());
    }
    out
}

fn scan_rp_files(dir: &Path, depth: usize, out: &mut Vec<String>) {
    if depth > 16 || out.len() > 20000 {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            if name.starts_with('.') || name == "node_modules" || name == "target" {
                continue;
            }
            scan_rp_files(&path, depth + 1, out);
        } else if path.extension().map(|e| e == "rp").unwrap_or(false) {
            out.push(normalize_path(&path.to_string_lossy()));
        }
    }
}

/// `overrides` holds the text of open editor buffers (newer than disk).
fn build_import_graph(
    roots: &[String],
    overrides: &HashMap<String, String>,
) -> HashMap<String, Vec<String>> {
    let mut set: HashSet<String> = HashSet::new();
    for root in roots {
        let mut found = Vec::new();
        scan_rp_files(Path::new(root), 0, &mut found);
        set.extend(found);
    }
    set.extend(overrides.keys().cloned());
    let mut files: Vec<String> = set.into_iter().collect();
    files.sort();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for file in &files {
        let text = match overrides.get(file) {
            Some(t) => t.clone(),
            None => match std::fs::read_to_string(file) {
                Ok(t) => t,
                Err(_) => continue,
            },
        };
        for import in parse_imports(&text) {
            let target = resolve_import(file, &import);
            graph.entry(target).or_default().push(file.clone());
        }
    }
    for importers in graph.values_mut() {
        importers.sort();
        importers.dedup();
    }
    graph
}

fn fallback_root(path: &str) -> String {
    let start = parent_dir(path);
    let mut dir = start.clone();
    loop {
        if Path::new(&dir).join(".git").exists() {
            return dir;
        }
        let parent = parent_dir(&dir);
        if parent == dir || dir == "/" || dir == "." {
            return start;
        }
        dir = parent;
    }
}

/// Files that (transitively) import `start`, level by level (BFS): direct importers first,
/// then importers of those, and so on. Open files go first within a level.
fn context_candidates(
    graph: &HashMap<String, Vec<String>>,
    start: &str,
    open: &HashSet<String>,
    max: usize,
) -> Vec<(String, usize)> {
    let mut result: Vec<(String, usize)> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.to_string());
    let mut frontier = vec![start.to_string()];
    let mut level = 0;
    while !frontier.is_empty() && result.len() < max {
        level += 1;
        let mut next: Vec<String> = Vec::new();
        for file in &frontier {
            if let Some(importers) = graph.get(file) {
                for importer in importers {
                    if visited.insert(importer.clone()) {
                        next.push(importer.clone());
                    }
                }
            }
        }
        next.sort_by(|a, b| open.contains(b).cmp(&open.contains(a)).then_with(|| a.cmp(b)));
        for n in &next {
            if result.len() >= max {
                break;
            }
            result.push((n.clone(), level));
        }
        frontier = next;
    }
    result
}

// ---------------------------------------------------------------------------
// One backend run, rated from the point of view of the open file
// ---------------------------------------------------------------------------

struct Attempt {
    entry: String,
    level: usize,
    result: Result<(BackendPayload, String), String>,
    elapsed_ms: u128,
    /// Parser / import / macro / io error somewhere: the analysis did not reach (or finish) the checker.
    fatal: bool,
    /// Errors located in the open file.
    own_errors: usize,
    /// parser / import / io error in the open file: a different entrypoint cannot fix that.
    hard_own: bool,
    /// Fatal macro error located in the open file. Standalone it may only mean "defined by
    /// an importer", but under an entrypoint that is already known to work it is a typo.
    own_macro: bool,
}

impl Attempt {
    fn key(&self) -> (u8, usize, usize) {
        (self.fatal as u8, self.own_errors, self.level)
    }

    fn clean(&self) -> bool {
        !self.fatal && self.own_errors == 0
    }
}

fn rate_attempt(
    entry: &str,
    level: usize,
    open_path: &str,
    result: Result<(BackendPayload, String), String>,
    elapsed_ms: u128,
) -> Attempt {
    let (fatal, own_errors, hard_own, own_macro) = match &result {
        Ok((payload, _)) => {
            let mut fatal = false;
            let mut own_errors = 0;
            let mut hard_own = false;
            let mut own_macro = false;
            for d in &payload.diagnostics {
                if d.severity != "error" {
                    continue;
                }
                let own = span_in_file(&d.span, open_path);
                let hard_phase = matches!(d.phase.as_str(), "io" | "parser" | "import");
                if hard_phase || d.phase == "macro" {
                    fatal = true;
                }
                if own {
                    own_errors += 1;
                    if hard_phase {
                        hard_own = true;
                    }
                    if d.phase == "macro" {
                        own_macro = true;
                    }
                }
            }
            (fatal, own_errors, hard_own, own_macro)
        }
        Err(_) => (true, 1_000_000, false, false),
    };
    Attempt {
        entry: entry.to_string(),
        level,
        result,
        elapsed_ms,
        fatal,
        own_errors,
        hard_own,
        own_macro,
    }
}

fn paths_equal(a: &str, b: &str) -> bool {
    normalize_path(a) == normalize_path(b)
}

/// True when the span belongs to `current` (an empty file name means "the analyzed file").
fn span_in_file(span: &BackendSpan, current: &str) -> bool {
    span.file.is_empty() || paths_equal(&span.file, current)
}

/// Diagnostics located in `own_path`, as LSP diagnostics. Errors that live in other files
/// (the analysis may cover many modules) are not shown here: each file is shown when it is
/// opened. If such an error stopped the analysis, one note says so.
fn own_diagnostics(payload: &BackendPayload, own_path: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let mut blocked_by: Option<&BackendDiagnostic> = None;
    for d in &payload.diagnostics {
        if span_in_file(&d.span, own_path) {
            let mut diagnostic = Diagnostic {
                range: span_to_range(&d.span),
                severity: Some(severity_from_str(&d.severity)),
                source: Some("raptor".to_string()),
                message: d.message.clone(),
                ..Default::default()
            };
            if !d.phase.is_empty() {
                diagnostic.message = format!("[{}] {}", d.phase, d.message);
            }
            out.push(diagnostic);
        } else if blocked_by.is_none()
            && d.severity == "error"
            && matches!(d.phase.as_str(), "io" | "parser" | "import" | "macro")
        {
            blocked_by = Some(d);
        }
    }
    if let Some(d) = blocked_by {
        out.push(Diagnostic {
            range: Range::new(Position::new(0, 0), Position::new(0, 1)),
            severity: Some(DiagnosticSeverity::WARNING),
            source: Some("raptor".to_string()),
            message: format!(
                "Analysis incomplete: error in {}:{}: {}",
                d.span.file, d.span.start_line, d.message
            ),
            ..Default::default()
        });
    }
    out
}

fn keyword_completions() -> Vec<CompletionItem> {
    const KEYWORDS: &[&str] = &[
        "fn", "let", "if", "else", "for", "while", "return", "break", "continue",
        "match", "rest", "struct", "enum", "import", "extern", "as", "true", "false",
        "void", "bool", "str", "char", "i8", "i16", "i32", "i64", "u8", "u16", "u32",
        "u64", "f64", "derives", "switch",
    ];
    KEYWORDS
        .iter()
        .map(|name| CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        })
        .collect()
}

fn std_function_completions() -> Vec<CompletionItem> {
    const FUNCTIONS: &[&str] = &[
        "print",
        "println",
        "input",
        "read_file",
        "write_file",
        "append_file",
        "delete_file",
        "exists_file",
        "tcp_accept",
        "tcp_close",
        "tcp_connect",
        "tcp_listen",
        "tcp_read",
        "tcp_write",
        "str_len",
        "sleep_ms",
        "vector_push",
        "vector_size",
        "vector_stringify",
        "cli_args",
    ];
    FUNCTIONS
        .iter()
        .map(|name| CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        })
        .collect()
}

fn identifier_completions(source: &str) -> Vec<CompletionItem> {
    let mut names = HashSet::new();
    let mut current = String::new();
    for ch in source.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            current.push(ch);
        } else {
            if current.len() > 1
                && current
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_alphabetic() || c == '_')
                    .unwrap_or(false)
            {
                names.insert(std::mem::take(&mut current));
            } else {
                current.clear();
            }
        }
    }
    if current.len() > 1 {
        names.insert(current);
    }
    names
        .into_iter()
        .map(|name| CompletionItem {
            label: name,
            kind: Some(CompletionItemKind::VARIABLE),
            ..Default::default()
        })
        .collect()
}

#[tower_lsp::async_trait]
impl LanguageServer for Handle {
    async fn initialize(&self, params: InitializeParams) -> LspResult<InitializeResult> {
        let mut roots: Vec<String> = Vec::new();
        if let Some(folders) = &params.workspace_folders {
            for folder in folders {
                roots.push(normalize_path(&path_for_uri(&folder.uri)));
            }
        }
        if roots.is_empty() {
            if let Some(root) = &params.root_uri {
                roots.push(normalize_path(&path_for_uri(root)));
            }
        }
        *self.workspace_roots.lock().await = roots;
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "raptor-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        let bin = backend_binary();
        self.client
            .log_message(
                MessageType::INFO,
                format!("raptor-lsp initialized (backend: {})", bin.display()),
            )
            .await;
    }

    async fn shutdown(&self) -> LspResult<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        // What the editor opened is what is on disk right now.
        let hash = hash_text(&text);
        self.0.validate(uri.clone(), text).await;
        if let Some(d) = self.documents.lock().await.get_mut(&uri) {
            d.saved_hash = Some(hash);
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            self.0.validate(params.text_document.uri, change.text).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        let (current, saved_hash) = {
            let docs = self.documents.lock().await;
            match docs.get(&uri) {
                Some(d) => (Some(d.text.clone()), d.saved_hash),
                None => (None, None),
            }
        };
        let text = match params.text {
            Some(t) => t,
            None => match current.clone() {
                Some(t) => t,
                None => return,
            },
        };
        let hash = hash_text(&text);

        // `did_change` already analyzed exactly this text: running the backend again would
        // only repeat that work (the old code did it on every single save).
        if current.as_deref() != Some(text.as_str()) {
            self.0.validate(uri.clone(), text).await;
        }

        // Disk content did not change (e.g. a repeated Ctrl+S): other files and the import
        // graph are exactly as they were.
        if saved_hash == Some(hash) {
            return;
        }
        if let Some(d) = self.documents.lock().await.get_mut(&uri) {
            d.saved_hash = Some(hash);
        }
        // Imports may have changed on disk: rebuild the import graph and re-analyze the
        // other open documents too.
        self.graph.lock().await.built_at = None;
        self.0.revalidate_others(&uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.lock().await.remove(&uri);
        // Dropping the sender wakes (and so cancels) an analysis that is still running.
        self.latest.lock().await.remove(&uri);
        self.context_cache
            .lock()
            .await
            .remove(&normalize_path(&path_for_uri(&uri)));
    }

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> LspResult<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let source = self
            .documents
            .lock()
            .await
            .get(&uri)
            .map(|doc| doc.text.clone())
            .unwrap_or_default();
        let mut items = keyword_completions();
        items.extend(std_function_completions());
        items.extend(identifier_completions(&source));
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> LspResult<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let documents = self.documents.lock().await;
        let Some(doc) = documents.get(&uri) else {
            return Ok(None);
        };
        let best = doc
            .hovers
            .iter()
            .filter(|h| span_contains_position(&h.span, position))
            .min_by_key(|h| span_len(&h.span));
        Ok(best.map(|h| Hover {
            contents: HoverContents::Scalar(MarkedString::String(h.contents.clone())),
            range: Some(span_to_range(&h.span)),
        }))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> LspResult<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let documents = self.documents.lock().await;
        let Some(doc) = documents.get(&uri) else {
            return Ok(None);
        };
        let best = doc
            .definitions
            .iter()
            .filter(|d| span_contains_position(&d.use_span, position))
            .min_by_key(|d| span_len(&d.use_span));
        let Some(def) = best else {
            return Ok(None);
        };
        let def_path = if def.def_span.file.is_empty() {
            path_for_uri(&uri)
        } else {
            def.def_span.file.clone()
        };
        // Same file as the one being edited: reuse the client's own URI untouched.
        let target_uri = if paths_equal(&def_path, &doc.analyzed_path) {
            uri.clone()
        } else {
            uri_for_path(&def_path).unwrap_or_else(|| uri.clone())
        };
        Ok(Some(GotoDefinitionResponse::Scalar(Location {
            uri: target_uri,
            range: span_to_range(&def.def_span),
        })))
    }
}

impl Backend {
    async fn revalidate_others(self: &Arc<Self>, except: &Url) {
        let others: Vec<(Url, String)> = self
            .documents
            .lock()
            .await
            .iter()
            .filter(|(u, _)| *u != except)
            .map(|(u, d)| (u.clone(), d.text.clone()))
            .collect();
        for (u, text) in others {
            self.validate(u, text).await;
        }
    }

    async fn is_latest(&self, uri: &Url, seq: u64) -> bool {
        self.latest
            .lock()
            .await
            .get(uri)
            .map(|tx| *tx.borrow() == seq)
            .unwrap_or(false)
    }

    /// Receiver that observes newer requests for `uri` (None when the document is closed).
    async fn subscribe(&self, uri: &Url) -> Option<watch::Receiver<u64>> {
        self.latest.lock().await.get(uri).map(|tx| tx.subscribe())
    }

    fn debounce(&self) -> Duration {
        let last = self.last_analysis_ms.load(Ordering::Relaxed);
        Duration::from_millis((last / 3).clamp(DEBOUNCE_MS, MAX_DEBOUNCE_MS))
    }

    /// (path, text) of every open editor buffer.
    async fn open_overlays(&self) -> Vec<(String, String)> {
        self.documents
            .lock()
            .await
            .values()
            .map(|d| (d.analyzed_path.clone(), d.text.clone()))
            .collect()
    }

    /// Reverse import graph, rebuilt when older than 3 s (or after a save).
    async fn importers_snapshot(
        &self,
        focus: &str,
        overlays: &[(String, String)],
    ) -> Arc<HashMap<String, Vec<String>>> {
        let mut roots = self.workspace_roots.lock().await.clone();
        let inside = roots
            .iter()
            .any(|r| focus.starts_with(&format!("{}/", r.trim_end_matches('/'))));
        if !inside {
            // single file / file outside the workspace folders
            roots.push(fallback_root(focus));
        }
        roots.sort();
        roots.dedup();

        let mut graph = self.graph.lock().await;
        let fresh = graph
            .built_at
            .map(|t| t.elapsed() < Duration::from_secs(3))
            .unwrap_or(false)
            && graph.roots == roots;
        if !fresh {
            let overrides: HashMap<String, String> = overlays.iter().cloned().collect();
            let scan_roots = roots.clone();
            let built =
                tokio::task::spawn_blocking(move || build_import_graph(&scan_roots, &overrides))
                    .await
                    .unwrap_or_default();
            graph.importers = Arc::new(built);
            graph.roots = roots;
            graph.built_at = Some(Instant::now());
        }
        graph.importers.clone()
    }

    async fn attempt(
        &self,
        entry: &str,
        level: usize,
        open_path: &str,
        overlays: &[(String, String)],
        cancel: &watch::Receiver<u64>,
        seq: u64,
    ) -> Attempt {
        let started = Instant::now();
        let result = run_backend(
            entry,
            open_path,
            overlays,
            &self.limiter,
            cancel.clone(),
            seq,
        )
        .await;
        rate_attempt(entry, level, open_path, result, started.elapsed().as_millis())
    }

    /// Analyze `path`. Standalone first; when that leaves errors in the file (typically
    /// symbols that are only defined by whoever imports it), walk up the import tree:
    /// importers, importers of importers, ... and keep the entrypoint with the fewest errors.
    /// Returns None when a newer edit made this analysis stale.
    async fn analyze_with_context(
        &self,
        uri: &Url,
        path: &str,
        seq: u64,
        overlays: &[(String, String)],
    ) -> Option<(Attempt, usize)> {
        let cancel = self.subscribe(uri).await?;

        // 1) entrypoint that worked last time
        let cached = self.context_cache.lock().await.get(path).cloned();
        if let Some(entry) = cached {
            let usable = entry != path
                && (overlays.iter().any(|(p, _)| paths_equal(p, &entry))
                    || Path::new(&entry).exists());
            if usable {
                let a = self.attempt(&entry, 1, path, overlays, &cancel, seq).await;
                if !self.is_latest(uri, seq).await {
                    return None;
                }
                // This entrypoint is known to work, so an error in the open file itself
                // (a half-typed derive, a syntax error) is just an error in the file. Only
                // a failure caused elsewhere sends us searching for another entrypoint:
                // re-searching on every keystroke used to cost up to 5 full analyses.
                if !a.fatal || a.hard_own || a.own_macro {
                    return Some((a, 1));
                }
            }
            self.context_cache.lock().await.remove(path);
        }

        // 2) the file on its own
        let first = self.attempt(path, 0, path, overlays, &cancel, seq).await;
        if !self.is_latest(uri, seq).await {
            return None;
        }
        // A syntax / import / io error in the file itself cannot be fixed by a different entrypoint.
        if first.clean() || first.hard_own {
            return Some((first, 1));
        }

        // 3) walk up the import tree
        let graph = self.importers_snapshot(path, overlays).await;
        let open: HashSet<String> = overlays.iter().map(|(p, _)| p.clone()).collect();
        let candidates = context_candidates(&graph, path, &open, MAX_CONTEXT_ATTEMPTS - 1);

        let mut best = first;
        let mut tried = 1;
        for (entry, level) in candidates {
            if !self.is_latest(uri, seq).await {
                return None;
            }
            let a = self.attempt(&entry, level, path, overlays, &cancel, seq).await;
            tried += 1;
            let clean = a.clean();
            if a.key() < best.key() {
                best = a;
            }
            if clean {
                break;
            }
        }

        let mut cache = self.context_cache.lock().await;
        if paths_equal(&best.entry, path) {
            cache.remove(path);
        } else {
            cache.insert(path.to_string(), best.entry.clone());
        }
        Some((best, tried))
    }

    /// Registers the newest text of `uri` and starts the analysis in a background task, so the
    /// LSP handler returns at once. (It used to await the whole analysis, which kept one of
    /// the server's few request slots busy for seconds and delayed hover / completion.)
    async fn validate(self: &Arc<Self>, uri: Url, text: String) {
        let display_path = normalize_path(&path_for_uri(&uri));

        // Register this request and store the newest text right away, so completion
        // never sees older text and older analyses can be recognized as stale (and killed).
        let seq = self.seq.fetch_add(1, Ordering::SeqCst) + 1;
        {
            let mut latest = self.latest.lock().await;
            match latest.get(&uri) {
                Some(tx) => {
                    tx.send_replace(seq);
                }
                None => {
                    let (tx, _rx) = watch::channel(seq);
                    latest.insert(uri.clone(), tx);
                }
            }
        }
        {
            let mut docs = self.documents.lock().await;
            match docs.get_mut(&uri) {
                Some(d) => {
                    d.text = text;
                    d.analyzed_path = display_path.clone();
                }
                None => {
                    docs.insert(
                        uri.clone(),
                        DocumentState {
                            text,
                            saved_hash: None,
                            analyzed_path: display_path.clone(),
                            hovers: Vec::new(),
                            definitions: Vec::new(),
                        },
                    );
                }
            }
        }

        let this = Arc::clone(self);
        tokio::spawn(async move { this.analyze_and_publish(uri, display_path, seq).await });
    }

    async fn analyze_and_publish(&self, uri: Url, display_path: String, seq: u64) {
        // Debounce: while the user types, only the last request runs the backend.
        tokio::time::sleep(self.debounce()).await;
        if !self.is_latest(&uri, seq).await {
            return;
        }

        let overlays = self.open_overlays().await;
        let Some((best, tried)) = self
            .analyze_with_context(&uri, &display_path, seq, &overlays)
            .await
        else {
            return;
        };
        if !self.is_latest(&uri, seq).await {
            return;
        }
        if best.result.is_ok() {
            self.last_analysis_ms
                .store(best.elapsed_ms.min(u64::MAX as u128) as u64, Ordering::Relaxed);
        }

        let (payload, stderr) = match best.result {
            Ok(r) => r,
            Err(e) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("analysis failed for {display_path} (entry {}): {e}", best.entry),
                    )
                    .await;
                let diag = Diagnostic {
                    range: Range::new(Position::new(0, 0), Position::new(0, 1)),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("raptor".to_string()),
                    message: e,
                    ..Default::default()
                };
                self.client
                    .publish_diagnostics(uri, vec![diag], None)
                    .await;
                return;
            }
        };

        if !stderr.trim().is_empty() {
            self.client
                .log_message(
                    MessageType::LOG,
                    format!("lsp-backend stderr for {display_path}:\n{stderr}"),
                )
                .await;
        }

        let own = own_diagnostics(&payload, &display_path);
        let (raw_ok, raw_diags, raw_h, raw_d) = (
            payload.ok,
            payload.diagnostics.len(),
            payload.hovers.len(),
            payload.definitions.len(),
        );

        // The analysis may cover many modules: keep only what is located in the open file.
        let hovers: Vec<HoverInfo> = payload
            .hovers
            .into_iter()
            .filter(|h| span_in_file(&h.span, &display_path))
            .map(|h| HoverInfo {
                span: h.span,
                contents: h.contents,
            })
            .collect();
        let definitions: Vec<DefinitionInfo> = payload
            .definitions
            .into_iter()
            .filter(|d| span_in_file(&d.use_span, &display_path))
            .map(|d| DefinitionInfo {
                use_span: d.use_span,
                def_span: d.def,
            })
            .collect();

        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "analysis {display_path}: entry {} (level {}, {tried} attempt(s)), {} ms, \
                     ok={raw_ok}, diagnostics {}/{raw_diags} own, hovers {}/{raw_h}, definitions {}/{raw_d}",
                    best.entry,
                    best.level,
                    best.elapsed_ms,
                    own.len(),
                    hovers.len(),
                    definitions.len(),
                ),
            )
            .await;

        {
            let mut docs = self.documents.lock().await;
            match docs.get_mut(&uri) {
                Some(d) => {
                    d.hovers = hovers;
                    d.definitions = definitions;
                }
                None => return, // document was closed meanwhile
            }
        }

        self.client.publish_diagnostics(uri, own, None).await;
    }
}

fn parse_backend_arg() {
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        if a == "--backend" {
            if let Some(path) = args.next() {
                let _ = BACKEND_PATH.set(PathBuf::from(path));
            }
        } else if let Some(path) = a.strip_prefix("--backend=") {
            let _ = BACKEND_PATH.set(PathBuf::from(path));
        }
    }
}

#[tokio::main]
async fn main() {
    parse_backend_arg();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    // Each backend run is single-threaded and CPU-bound: leave a core for the editor.
    let permits = std::thread::available_parallelism()
        .map(|n| n.get().saturating_sub(1))
        .unwrap_or(1)
        .clamp(1, 4);
    let (service, socket) = LspService::new(move |client| Handle(Arc::new(Backend {
        client,
        documents: Mutex::new(HashMap::new()),
        latest: Mutex::new(HashMap::new()),
        seq: AtomicU64::new(0),
        limiter: Semaphore::new(permits),
        last_analysis_ms: AtomicU64::new(0),
        context_cache: Mutex::new(HashMap::new()),
        graph: Mutex::new(ImportGraph {
            built_at: None,
            roots: Vec::new(),
            importers: Arc::new(HashMap::new()),
        }),
        workspace_roots: Mutex::new(Vec::new()),
    })));
    Server::new(stdin, stdout, socket).serve(service).await;
}