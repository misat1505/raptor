use crate::{
    common::{errors::IError, span::Span},
    frontend::{
        ast::{Node, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_declaration(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // declaration = type, identifier, [ "=", expression ];

        let declaration_type = try_consume!(self, parse_type);

        let identifier = self
            .parse_identifier()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create identifier while parsing variable declaration.")))?;

        let value = match self.consume_if_matches(TokenCategory::Assign)? {
            Some(_) => self.parse_expression()?,
            None => None,
        };

        let end = value.as_ref().map(|value| value.span.end()).unwrap_or_else(|| identifier.span.end());

        let node = Node {
            value: Statement::Declaration {
                var_type: declaration_type.clone(),
                identifier,
                value,
            },
            span: Span::new(declaration_type.span.start(), end),
        };

        Ok(Some(node))
    }

    pub(in crate::frontend::parser) fn parse_variable_declaration(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        let mut decl = try_consume!(self, parse_declaration);

        self.consume_must_be(TokenCategory::Semicolon)?;

        decl.span = Span::new(decl.span.start(), self.current_token().span.end());

        Ok(Some(decl))
    }
}
