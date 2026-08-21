use crate::{
    common::{errors::IError, span::Span},
    frontend::{
        ast::{Node, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume_token, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_for_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // for_statement = "for", "(", [ declaration | let_declaration ], ";", expression, ";", [ identifier, "=", expression ], ")", statement_block;

        let for_token = try_consume_token!(self, TokenCategory::For);

        let _ = self.consume_must_be(TokenCategory::ParenOpen)?;

        let declaration = if self.current_token().category == TokenCategory::Let {
            self.parse_let_declaration()?
        } else {
            self.parse_declaration()?
        }
        .map(Box::new);

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
        }

        self.consume_must_be(TokenCategory::ParenClose)?;

        let block = self
            .parse_statement_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement block while parsing for statement.")))?;

        let node = Node {
            value: Statement::ForLoop {
                declaration,
                condition,
                assignment,
                block: block.clone(),
            },
            span: Span::new(for_token.span.start(), block.span.end()),
        };

        Ok(Some(node))
    }
}
