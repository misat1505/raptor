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
    pub(in crate::frontend::parser) fn parse_break_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // break_statement = "break", ";";
        let token = try_consume_token!(self, TokenCategory::Break);

        let _ = self.consume_must_be(TokenCategory::Semicolon)?;
        let node = Node {
            value: Statement::Break,
            position: token.position,
        };
        Ok(Some(node))
    }
}
