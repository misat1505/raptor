use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;
use std::thread_local;

use raptor_lib::common::errors::{ErrorSeverity, IError};
use raptor_lib::frontend::lexer::lazy_stream_reader::LazyStreamReader;
use raptor_lib::frontend::lexer::lexer::{Lexer, LexerOptions};
use raptor_lib::frontend::parser::{IParser, Parser};
use raptor_lib::frontend::tokens::{TokenCategory, TokenValue};
use raptor_lib::semantic::semantic_checker::checker::SemanticChecker;

use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

thread_local! {
    static LEXER_WARNINGS: RefCell<Vec<Box<dyn IError>>> = RefCell::new(Vec::new());
}

fn on_warning(warning: Box<dyn IError>) {
    LEXER_WARNINGS.with(|w| w.borrow_mut().push(warning));
}

struct Backend {
    client: Client,
    documents: Mutex<HashMap<Url, String>>,
}

fn std_function_completions() -> Vec<CompletionItem> {
    let functions = [
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
        "tcp_listen",
        "tcp_read",
        "tcp_write",
        "str_len",
        "sleep_ms",
        "vector_push",
        "vector_size",
        "vector_stringify",
    ];

    functions
        .iter()
        .map(|name| CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        })
        .collect()
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
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        self.documents.lock().await.insert(uri.clone(), text.clone());

        self.validate(uri, text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            let uri = params.text_document.uri;
            let text = change.text;

            self.documents.lock().await.insert(uri.clone(), text.clone());

            self.validate(uri, text).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        if let Some(text) = params.text {
            let uri = params.text_document.uri;

            self.documents.lock().await.insert(uri.clone(), text.clone());

            self.validate(uri, text).await;
        }
    }

    async fn completion(&self, params: CompletionParams) -> LspResult<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;

        let source = self.documents.lock().await.get(&uri).cloned().unwrap_or_default();

        let mut items = keyword_completions();

        items.extend(std_function_completions());
        items.extend(identifier_completions(&source));

        Ok(Some(CompletionResponse::Array(items)))
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

fn keyword_completions() -> Vec<CompletionItem> {
    let keywords = [
        "for", "while", "if", "else", "as", "fn", "true", "false", "return", "switch", "break", "continue", "import", "extern",
    ];
    let types = ["bool", "str", "i64", "f64", "void"];

    let mut items: Vec<CompletionItem> = keywords
        .iter()
        .map(|kw| CompletionItem {
            label: kw.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        })
        .collect();

    items.extend(types.iter().map(|ty| CompletionItem {
        label: ty.to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    }));

    items
}

fn identifier_completions(source: &str) -> Vec<CompletionItem> {
    let cursor = Cursor::new(source.as_bytes().to_vec());
    let reader = LazyStreamReader::new(cursor, Some("current"));
    let lexer_options = LexerOptions {
        max_comment_length: 500,
        max_identifier_length: 100,
    };

    let mut names = std::collections::HashSet::new();

    if let Ok(mut lexer) = Lexer::new(reader, lexer_options, |_| {}) {
        while let Ok(token) = lexer.generate_token() {
            // dopasuj do realnego API lexera
            if token.category == TokenCategory::Identifier {
                if let TokenValue::String(name) = &token.value {
                    names.insert(name.clone());
                }
            }
            if token.category == TokenCategory::ETX {
                break;
            }
        }
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

fn error_to_diagnostic(err: &dyn IError, severity: DiagnosticSeverity) -> Diagnostic {
    let span = err.get_span();

    let range = Range::new(
        Position::new(span.start().line - 1, span.start().column - 1),
        Position::new(span.end().line - 1, span.end().column - 1),
    );

    Diagnostic {
        range,
        severity: Some(severity),
        source: Some("raptor".to_string()),
        message: err.message(),
        ..Default::default()
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: Mutex::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
