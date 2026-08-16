
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
    pub fn parse_for_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // for_statement = "for", "(", [ declaration ], ";", expression, ";", [ identifier, "=", expression ], ")", statement_block;
        let for_token = try_consume_token!(self, TokenCategory::For);

        let _ = self.consume_must_be(TokenCategory::ParenOpen)?;
        let declaration = self
            .parse_declaration()
            .map_err(|_| self.create_parser_error(String::from("Couldn't create declaration while parsing for statement.")))?
            .map(|t| {
                let position = t.position;
                let node = Node { value: t.value, position };
                Box::new(node)
            });

        self.consume_must_be(TokenCategory::Semicolon)?;
        let condition = self
            .parse_expression()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing for statement.")))?;

        self.consume_must_be(TokenCategory::Semicolon)?;
        let mut assignment: Option<Box<Node<Statement>>> = None;
        if self.current_token().category == TokenCategory::Identifier {
            assignment = Some(Box::new(self.parse_assign_or_call_without_semicolon()?.ok_or_else(|| {
                self.create_parser_error(String::from("Couldn't create assignment or call while parsing for statement."))
            })?));
        };

        self.consume_must_be(TokenCategory::ParenClose)?;
        let block = self
            .parse_statement_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement block while parsing for statement.")))?;

        let node = Node {
            value: Statement::ForLoop {
                declaration,
                condition,
                assignment,
                block,
            },
            position: for_token.position,
        };
        Ok(Some(node))
    }
}
