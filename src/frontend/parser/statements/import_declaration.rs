use crate::{
    common::{errors::IError, span::Span},
    frontend::{
        ast::{Literal, Node, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume_token, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_import_declaration(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // import_declaration = "import", literal, ";";

        let import_token = try_consume_token!(self, TokenCategory::Import);

        let import_literal = self
            .parse_literal()?
            .ok_or_else(|| self.create_parser_error("Couldn't create literal while parsing import declaration.".to_string()))?;

        let Literal::String(import_path) = import_literal.value else {
            return Err(self.create_parser_error("Import path must be a string literal.".to_string()));
        };

        let semicolon_token = self.consume_must_be(TokenCategory::Semicolon)?;

        let node = Node {
            value: Statement::Import {
                path: Node {
                    value: import_path,
                    span: import_literal.span,
                },
            },
            span: Span::new(import_token.span.start(), semicolon_token.span.end()),
        };

        Ok(Some(node))
    }
}
