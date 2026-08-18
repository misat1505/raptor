use raptor_lib::frontend::lexer::lexer::Lexer;
use raptor_lib::frontend::parser::Parser;
use raptor_lib::semantic::semantic_checker::checker::SemanticChecker;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
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
        self.client.log_message(MessageType::INFO, "lsp initialized").await;
    }

    async fn shutdown(&self) -> Result<()> {
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
}

impl Backend {
    async fn validate(&self, uri: Url, text: String) {
        let mut diagnostics = vec![];

        // TODO: podłącz tu realny lexer/parser/semantic checker
        // i mapuj błędy Position -> lsp_types::Range
        match parse_and_check(&text) {
            Ok(_) => {}
            Err(e) => {
                diagnostics.push(Diagnostic {
                    range: Range::new(Position::new(0, 0), Position::new(0, 1)),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: e.to_string(),
                    ..Default::default()
                });
            }
        }

        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

fn parse_and_check(source: &str) -> std::result::Result<(), String> {
    // np.:
    // let tokens = Lexer::new(source).tokenize().map_err(|e| e.to_string())?;
    // let ast = Parser::new(tokens).parse_program().map_err(|e| e.to_string())?;
    // Checker::new().check(&ast).map_err(|e| e.to_string())?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
