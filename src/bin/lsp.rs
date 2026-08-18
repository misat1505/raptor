use std::cell::RefCell;
use std::io::Cursor;
use std::thread_local;

use raptor_lib::common::errors::{ErrorSeverity, IError};
use raptor_lib::frontend::lexer::lazy_stream_reader::LazyStreamReader;
use raptor_lib::frontend::lexer::lexer::{Lexer, LexerOptions};
use raptor_lib::frontend::parser::{IParser, Parser};
use raptor_lib::semantic::semantic_checker::checker::SemanticChecker;

use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use regex::Regex;

thread_local! {
    static LEXER_WARNINGS: RefCell<Vec<Box<dyn IError>>> = RefCell::new(Vec::new());
}

fn on_warning(warning: Box<dyn IError>) {
    LEXER_WARNINGS.with(|w| w.borrow_mut().push(warning));
}

struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> LspResult<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "raptor-lsp initialized").await;
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
}

impl Backend {
    async fn validate(&self, uri: Url, text: String) {
        let diagnostics = analyze(&text);
        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

fn analyze(source: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Czyścimy bufor warningów przed każdym przebiegiem
    LEXER_WARNINGS.with(|w| w.borrow_mut().clear());

    // --- LEXER ---
    let cursor = Cursor::new(source.as_bytes().to_vec());
    let reader = LazyStreamReader::new(cursor, Some("current"));

    let lexer_options = LexerOptions {
        max_comment_length: 500,
        max_identifier_length: 100,
    };

    let lexer = match Lexer::new(reader, lexer_options, on_warning) {
        Ok(lexer) => lexer,
        Err(err) => {
            diagnostics.push(error_to_diagnostic(err.as_ref(), DiagnosticSeverity::ERROR));
            return diagnostics;
        }
    };

    LEXER_WARNINGS.with(|w| {
        for warning in w.borrow().iter() {
            diagnostics.push(error_to_diagnostic(warning.as_ref(), DiagnosticSeverity::WARNING));
        }
    });

    // --- PARSER ---
    let mut parser = Parser::new(lexer);

    let program = match parser.parse() {
        Ok(program) => program,
        Err(err) => {
            diagnostics.push(error_to_diagnostic(err.as_ref(), DiagnosticSeverity::ERROR));
            return diagnostics;
        }
    };

    // --- SEMANTIC CHECKER ---
    let mut semantic_checker = match SemanticChecker::new(&program) {
        Ok(checker) => checker,
        Err(err) => {
            diagnostics.push(error_to_diagnostic(err.as_ref(), DiagnosticSeverity::ERROR));
            return diagnostics;
        }
    };

    semantic_checker.check();

    for error in &semantic_checker.errors {
        let severity = match error.get_severity() {
            ErrorSeverity::HIGH => DiagnosticSeverity::ERROR,
            ErrorSeverity::LOW => DiagnosticSeverity::WARNING,
        };
        diagnostics.push(error_to_diagnostic(error.as_ref(), severity));
    }

    diagnostics
}

fn parse_position_from_message(message: &str) -> (Option<(u32, u32)>, String) {
    let ansi_re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    let clean = ansi_re.replace_all(message, "").to_string();

    let pos_re = Regex::new(r"-->.*?:(\d+):(\d+)").unwrap();
    let position = pos_re.captures(&clean).map(|caps| {
        let line: u32 = caps[1].parse().unwrap_or(1);
        let col: u32 = caps[2].parse().unwrap_or(1);
        (line, col)
    });

    (position, clean)
}

fn error_to_diagnostic(err: &dyn IError, severity: DiagnosticSeverity) -> Diagnostic {
    let (pos, clean_message) = parse_position_from_message(&err.message());

    let range = match pos {
        Some((line, col)) => {
            let l = line.saturating_sub(1);
            let c = col.saturating_sub(1);
            Range::new(Position::new(l, c), Position::new(l, c + 1))
        }
        None => Range::new(Position::new(0, 0), Position::new(0, 1)),
    };

    Diagnostic {
        range,
        severity: Some(severity),
        source: Some("raptor".to_string()),
        message: clean_message,
        ..Default::default()
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
