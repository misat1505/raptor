use crate::{
    common::{errors::IError, span::Span},
    frontend::{
        ast::{Block, Node, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume_token, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_statement_block(&mut self) -> Result<Option<Node<Block>>, Box<dyn IError>> {
        // statement_block = ("{", {statement}, "}") | statement;

        if let Some(stmt) = self.parse_statement()? {
            return Ok(Some(Node {
                value: Block(vec![stmt.clone()]),
                span: stmt.span,
            }));
        }

        let token = try_consume_token!(self, TokenCategory::BraceOpen);

        let mut statements: Vec<Node<Statement>> = vec![];

        let closing_brace = loop {
            if let Some(token) = self.consume_if_matches(TokenCategory::BraceClose)? {
                break token;
            }

            let statement = self
                .parse_statement()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement while parsing statement block.")))?;

            statements.push(statement);
        };

        Ok(Some(Node {
            value: Block(statements),
            span: Span::new(token.span.start(), closing_brace.span.end()),
        }))
    }
}
