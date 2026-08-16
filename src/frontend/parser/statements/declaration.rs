
use crate::{
    common::errors::IError,
    frontend::{
        ast::{Node, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub fn parse_declaration(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // declaration = type, identifier, [ "=", expression ];
        let declaration_type = try_consume!(self, parse_type);

        let position = declaration_type.position;
        let identifier = self
            .parse_identifier()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create identifier while parsing variable declaration.")))?;

        let value = match self.consume_if_matches(TokenCategory::Assign)? {
            Some(_) => self.parse_expression()?,
            None => None,
        };
        let node = Node {
            value: Statement::Declaration {
                var_type: declaration_type,
                identifier,
                value,
            },
            position,
        };
        Ok(Some(node))
    }

    pub fn parse_variable_declaration(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        let decl = try_consume!(self, parse_declaration);

        self.consume_must_be(TokenCategory::Semicolon)?;
        Ok(Some(decl))
    }
}
