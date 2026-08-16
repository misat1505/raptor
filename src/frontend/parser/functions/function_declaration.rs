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
    pub fn parse_function_declaration(&mut self) -> Result<Option<Node<FunctionDeclaration>>, Box<dyn IError>> {
        // function_declaration = "fn", identifier, "(", parameters, ")", ":", type | "void", statement_block;
        let fn_token = try_consume_token!(self, TokenCategory::Fn);

        let identifier = self
            .parse_identifier()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create identifier while parsing function declaration.")))?;

        let _ = self.consume_must_be(TokenCategory::ParenOpen)?;
        let parameters = self.parse_parameters()?;
        let _ = self.consume_must_be(TokenCategory::ParenClose)?;
        let _ = self.consume_must_be(TokenCategory::Colon)?;
        let return_type = match self.parse_type() {
            Ok(Some(t)) => t,
            _ => self.void_type_or_error()?,
        };
        let block = self
            .parse_statement_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement block while parsing function declaration.")))?;

        let node = Node {
            value: FunctionDeclaration {
                identifier,
                parameters,
                return_type,
                block,
            },
            position: fn_token.position,
        };

        Ok(Some(node))
    }
}
