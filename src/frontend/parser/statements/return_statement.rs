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
    pub fn parse_return_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // return_statement = "return", [ expression ], ";";
        let token = try_consume_token!(self, TokenCategory::Return);

        let returned_value = self.parse_expression()?;
        self.consume_must_be(TokenCategory::Semicolon)?;
        let node = Node {
            value: Statement::Return(returned_value),
            position: token.position,
        };
        Ok(Some(node))
    }
}
