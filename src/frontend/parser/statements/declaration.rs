use crate::{
    common::{errors::IError, span::Span},
    frontend::{
        ast::{Node, Statement, VariableDeclarationKind},
        lexer::lexer::ILexer,
        parser::{
            core::{try_consume, try_consume_token},
            Parser,
        },
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
                identifier,
                kind: VariableDeclarationKind::TYPE {
                    value,
                    var_type: declaration_type.clone(),
                },
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

    pub(in crate::frontend::parser) fn parse_let_declaration(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // let_declaration = let, identifier, [ ":", type ], "=", expression;

        let let_token = try_consume_token!(self, TokenCategory::Let);

        let identifier = self
            .parse_identifier()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create identifier while parsing variable declaration.")))?;

        let var_type = if self.current_token().category == TokenCategory::Colon {
            self.consume_must_be(TokenCategory::Colon)?;

            Some(
                self.parse_type()?
                    .ok_or_else(|| self.create_parser_error(String::from("Couldn't create type while parsing let declaration.")))?,
            )
        } else {
            None
        };

        self.consume_must_be(TokenCategory::Assign)?;

        let value = self
            .parse_expression()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't parse expression while parsing variable declaration.")))?;

        let end = value.span.end();

        let node = Node {
            value: Statement::Declaration {
                identifier,
                kind: VariableDeclarationKind::LET { var_type, value },
            },
            span: Span::new(let_token.span.start(), end),
        };

        Ok(Some(node))
    }

    pub(in crate::frontend::parser) fn parse_let_variable_declaration(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // let_variable_declaration = let, identifier, [ ":", type ], "=", expression, ";";

        let mut decl = try_consume!(self, parse_let_declaration);

        self.consume_must_be(TokenCategory::Semicolon)?;

        decl.span = Span::new(decl.span.start(), self.current_token().span.end());

        Ok(Some(decl))
    }
}
