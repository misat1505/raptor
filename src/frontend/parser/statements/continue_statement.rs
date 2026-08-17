use crate::{
    common::errors::IError,
    frontend::{
        ast::{Node, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume_token, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_continue_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // continue_statement = "continue", ";";
        let token = try_consume_token!(self, TokenCategory::Continue);

        let _ = self.consume_must_be(TokenCategory::Semicolon)?;
        let node = Node {
            value: Statement::Continue,
            position: token.position,
        };
        Ok(Some(node))
    }
}
