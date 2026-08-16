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
    pub fn parse_extern_function_declaration(&mut self) -> Result<Option<Node<ExternFunctionDeclaration>>, Box<dyn IError>> {
        // function_declaration = "extern", "fn", identifier, "(", parameters, ")", ":", type | "void", [ "as", identifier ] ";";
        let extern_token = try_consume_token!(self, TokenCategory::Extern);
        let _ = self.consume_must_be(TokenCategory::Fn)?;

        let identifier = self
            .parse_identifier()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create identifier while parsing extern function declaration.")))?;

        let _ = self.consume_must_be(TokenCategory::ParenOpen)?;
        let parameters = self.parse_parameters()?;
        let _ = self.consume_must_be(TokenCategory::ParenClose)?;
        let _ = self.consume_must_be(TokenCategory::Colon)?;
        let return_type = match self.parse_type() {
            Ok(Some(t)) => t,
            _ => self.void_type_or_error()?,
        };

        let alias = if self.consume_if_matches(TokenCategory::As)?.is_some() {
            Some(
                self.parse_identifier()?
                    .ok_or_else(|| self.create_parser_error(String::from("Expected identifier after 'as' in extern function declaration.")))?,
            )
        } else {
            None
        };

        let _ = self.consume_must_be(TokenCategory::Semicolon)?;

        let node = Node {
            value: ExternFunctionDeclaration {
                identifier,
                parameters,
                return_type,
                alias,
            },
            position: extern_token.position,
        };

        Ok(Some(node))
    }
}
