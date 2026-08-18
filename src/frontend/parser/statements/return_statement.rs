use crate::{
    common::{errors::IError, span::Span},
    frontend::{
        ast::{Node, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume_token, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_return_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // return_statement = "return", [ expression ], ";";

        let token = try_consume_token!(self, TokenCategory::Return);

        let returned_value = self.parse_expression()?;

        let semicolon = self.consume_must_be(TokenCategory::Semicolon)?;

        let end = returned_value
            .as_ref()
            .map(|value| value.span.end())
            .unwrap_or_else(|| semicolon.span.end());

        let node = Node {
            value: Statement::Return(returned_value),
            span: Span::new(token.span.start(), end),
        };

        Ok(Some(node))
    }
}
