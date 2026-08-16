use std::vec;

use crate::{
    common::errors::IError,
    frontend::{
        ast::{Block, Node, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume_token, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub fn parse_statement_block(&mut self) -> Result<Option<Node<Block>>, Box<dyn IError>> {
        // statement_block = ("{", {statement}, "}") | statement;
        if let Some(stmt) = self.parse_statement()? {
            return Ok(Some(Node {
                value: Block(vec![stmt.clone()]),
                position: stmt.position,
            }));
        }

        let token = try_consume_token!(self, TokenCategory::BraceOpen);

        let mut statements: Vec<Node<Statement>> = vec![];
        while self.consume_if_matches(TokenCategory::BraceClose)?.is_none() {
            let statement = self
                .parse_statement()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement while parsing statement block.")))?;

            statements.push(statement);
        }
        Ok(Some(Node {
            value: Block(statements),
            position: token.position,
        }))
    }
}
