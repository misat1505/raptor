use crate::{
    common::errors::IError,
    frontend::{ast::Node, lexer::lexer::ILexer, parser::Parser, tokens::TokenCategory},
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_derive_declaration(&mut self) -> Result<Vec<Node<String>>, Box<dyn IError>> {
        // derives_declaration = derives, identifier, { ",", identifier };
        let _ = match self.consume_if_matches(TokenCategory::Derives)? {
            Some(token) => token,
            None => return Ok(vec![]),
        };

        let mut derived_value = self
            .parse_identifier()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create identifier while parsing derive declaration.")))?;

        let mut derives = vec![derived_value];

        while self.consume_if_matches(TokenCategory::Comma)?.is_some() {
            derived_value = self
                .parse_identifier()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create identifier while parsing derive declaration.")))?;

            derives.push(derived_value);
        }

        Ok(derives)
    }
}
