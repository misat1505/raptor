use std::{collections::HashMap, rc::Rc};

use crate::{
    backend::std_functions::std_functions::get_std_functions,
    common::errors::{ErrorSeverity, IError, ParserError},
    frontend::{
        ast::{ExternFunctionDeclaration, FunctionDeclaration, Node, Program, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume_token, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub fn parse_while_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // while_statement = "while", "(", expression, ")", statement_block;
        let if_token = try_consume_token!(self, TokenCategory::While);

        let _ = self.consume_must_be(TokenCategory::ParenOpen)?;
        let condition = self
            .parse_expression()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing while statement.")))?;

        let _ = self.consume_must_be(TokenCategory::ParenClose)?;
        let block = self
            .parse_statement_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement block while parsing while statement.")))?;

        let node = Node {
            value: Statement::WhileLoop { condition, block },
            position: if_token.position,
        };
        Ok(Some(node))
    }
}
