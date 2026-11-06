use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use serde::Deserialize;
use tempfile::NamedTempFile;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

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
    /// Absolute path of the temp (or real) file last analyzed for this buffer.
    analyzed_path: String,
    hovers: Vec<HoverInfo>,
    definitions: Vec<DefinitionInfo>,
}

struct Backend {
    client: Client,
    documents: Mutex<HashMap<Url, DocumentState>>,
    /// Files that received diagnostics for a given entry URI (so we can clear
    /// stale ones on the next validation).
    published_files: Mutex<HashMap<Url, HashSet<String>>>,
}

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

fn fix_wsl_drive_prefix(path: String) -> String {
    #[cfg(not(windows))]
    {
        let bytes = path.as_bytes();
        let looks_like_drive_letter =
            bytes.len() >= 4 && bytes[0] == b'/' && bytes[1].is_ascii_alphabetic() && bytes[2] == b':' && (bytes[3] == b'/' || bytes[3] == b'\\');
        if looks_like_drive_letter {
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
    Url::from_file_path(path).ok()
}

fn backend_binary() -> PathBuf {
    static CACHED: OnceLock<PathBuf> = OnceLock::new();
    CACHED
        .get_or_init(|| {
            if let Ok(p) = std::env::var("RAPTOR_LSP_BACKEND") {
                return PathBuf::from(p);
            }
            if let Ok(path) = which("lsp-backend") {
                return path;
            }
            PathBuf::from("./build/lsp-backend")
        })
        .clone()
}

/// Minimal `which` without an extra crate.
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

// ---------------------------------------------------------------------------
// Span ↔ LSP
// ---------------------------------------------------------------------------

fn span_to_range(span: &BackendSpan) -> Range {
    Range::new(
        Position::new(span.start_line.saturating_sub(1) as u32, span.start_column.saturating_sub(1) as u32),
        Position::new(span.end_line.saturating_sub(1) as u32, span.end_column.saturating_sub(1) as u32),
    )
}

fn span_contains_position(span: &BackendSpan, pos: Position) -> bool {
    let range = span_to_range(span);
    (pos.line > range.start.line || (pos.line == range.start.line && pos.character >= range.start.character))
        && (pos.line < range.end.line || (pos.line == range.end.line && pos.character <= range.end.character))
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

// ---------------------------------------------------------------------------
// Invoke lsp-backend
// ---------------------------------------------------------------------------

fn run_backend(source: &str, display_path: &str) -> Result<BackendPayload, String> {
    let mut tmp = NamedTempFile::with_suffix(".rp").map_err(|e| e.to_string())?;
    tmp.write_all(source.as_bytes()).map_err(|e| e.to_string())?;
    tmp.flush().map_err(|e| e.to_string())?;
    let tmp_path = tmp.path().to_path_buf();

    let bin = backend_binary();
    if !bin.exists() {
        return Err(format!("lsp-backend not found at '{}' (set RAPTOR_LSP_BACKEND)", bin.display()));
    }

    let output = Command::new(&bin)
        .arg(tmp_path.as_os_str())
        .output()
        .map_err(|e| format!("failed to spawn {}: {e}", bin.display()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Backend prints one JSON object (possibly among other lines). Take the
    // last line that looks like a JSON object.
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
            "lsp-backend produced no JSON (exit {}). stderr:\n{stderr}\nstdout:\n{stdout}",
            output.status
        ));
    }

    let mut payload: BackendPayload = serde_json::from_str(json_line).map_err(|e| format!("invalid backend JSON: {e}\n{json_line}"))?;

    // Remap temp-file paths in spans back to the editor's path so diagnostics
    // and go-to-def land on the open document.
    let tmp_str = tmp_path.to_string_lossy().to_string();
    let remap = |span: &mut BackendSpan| {
        if span.file.is_empty() || span.file == tmp_str || paths_equal(&span.file, &tmp_str) {
            span.file = display_path.to_string();
        }
    };
    for d in &mut payload.diagnostics {
        remap(&mut d.span);
    }
    for h in &mut payload.hovers {
        remap(&mut h.span);
    }
    for d in &mut payload.definitions {
        remap(&mut d.use_span);
        remap(&mut d.def);
    }

    // Keep tempfile alive until here (dropped at end of scope).
    drop(tmp);
    Ok(payload)
}

fn paths_equal(a: &str, b: &str) -> bool {
    Path::new(a) == Path::new(b)
}

fn payload_to_diagnostics(payload: &BackendPayload) -> HashMap<String, Vec<Diagnostic>> {
    let mut map: HashMap<String, Vec<Diagnostic>> = HashMap::new();
    for d in &payload.diagnostics {
        let file = if d.span.file.is_empty() { String::new() } else { d.span.file.clone() };
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
        map.entry(file).or_default().push(diagnostic);
    }
    map
}

// ---------------------------------------------------------------------------
// Completions (local — no backend round-trip)
// ---------------------------------------------------------------------------

fn keyword_completions() -> Vec<CompletionItem> {
    const KEYWORDS: &[&str] = &[
        "fn", "let", "if", "else", "for", "while", "return", "break", "continue", "match", "rest", "struct", "enum", "import", "extern", "as",
        "true", "false", "void", "bool", "str", "char", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f64", "derives", "switch",
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
            if current.len() > 1 && current.chars().next().map(|c| c.is_ascii_alphabetic() || c == '_').unwrap_or(false) {
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

// ---------------------------------------------------------------------------
// LanguageServer
// ---------------------------------------------------------------------------

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> LspResult<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
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
            .log_message(MessageType::INFO, format!("raptor-lsp initialized (backend: {})", bin.display()))
            .await;
    }

    async fn shutdown(&self) -> LspResult<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.validate(params.text_document.uri, params.text_document.text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            self.validate(params.text_document.uri, change.text).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        if let Some(text) = params.text {
            self.validate(params.text_document.uri, text).await;
        }
    }

    async fn completion(&self, params: CompletionParams) -> LspResult<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let source = self.documents.lock().await.get(&uri).map(|doc| doc.text.clone()).unwrap_or_default();
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

    async fn goto_definition(&self, params: GotoDefinitionParams) -> LspResult<Option<GotoDefinitionResponse>> {
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
        let target_uri = uri_for_path(&def_path).unwrap_or_else(|| uri.clone());
        Ok(Some(GotoDefinitionResponse::Scalar(Location {
            uri: target_uri,
            range: span_to_range(&def.def_span),
        })))
    }
}

impl Backend {
    async fn validate(&self, uri: Url, text: String) {
        let display_path = path_for_uri(&uri);

        let result = tokio::task::spawn_blocking({
            let text = text.clone();
            let display_path = display_path.clone();
            move || run_backend(&text, &display_path)
        })
        .await;

        let payload = match result {
            Ok(Ok(p)) => p,
            Ok(Err(e)) => {
                self.client.log_message(MessageType::ERROR, format!("analysis failed: {e}")).await;
                let diag = Diagnostic {
                    range: Range::new(Position::new(0, 0), Position::new(0, 1)),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("raptor".to_string()),
                    message: e,
                    ..Default::default()
                };
                self.client.publish_diagnostics(uri, vec![diag], None).await;
                return;
            }
            Err(e) => {
                self.client.log_message(MessageType::ERROR, format!("analysis task panicked: {e}")).await;
                return;
            }
        };

        let diagnostics_by_file = payload_to_diagnostics(&payload);
        let hovers: Vec<HoverInfo> = payload
            .hovers
            .into_iter()
            .map(|h| HoverInfo {
                span: h.span,
                contents: h.contents,
            })
            .collect();
        let definitions: Vec<DefinitionInfo> = payload
            .definitions
            .into_iter()
            .map(|d| DefinitionInfo {
                use_span: d.use_span,
                def_span: d.def,
            })
            .collect();

        self.documents.lock().await.insert(
            uri.clone(),
            DocumentState {
                text,
                analyzed_path: display_path.clone(),
                hovers,
                definitions,
            },
        );

        let mut current_files: HashSet<String> = HashSet::new();

        // Entry document — always publish (clears previous diagnostics).
        let own = diagnostics_by_file
            .get(&display_path)
            .cloned()
            .or_else(|| diagnostics_by_file.get("").cloned())
            .unwrap_or_default();
        self.client.publish_diagnostics(uri.clone(), own, None).await;
        current_files.insert(display_path.clone());

        for (file, diags) in &diagnostics_by_file {
            if file.is_empty() || paths_equal(file, &display_path) {
                continue;
            }
            if let Some(other_uri) = uri_for_path(file) {
                self.client.publish_diagnostics(other_uri, diags.clone(), None).await;
                current_files.insert(file.clone());
            }
        }

        // Clear diagnostics on files that no longer report any.
        let mut published = self.published_files.lock().await;
        if let Some(previous) = published.get(&uri) {
            for stale in previous {
                if !current_files.contains(stale) {
                    if let Some(stale_uri) = uri_for_path(stale) {
                        self.client.publish_diagnostics(stale_uri, vec![], None).await;
                    }
                }
            }
        }
        published.insert(uri, current_files);
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: Mutex::new(HashMap::new()),
        published_files: Mutex::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
