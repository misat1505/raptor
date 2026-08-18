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
    pub(in crate::frontend::parser) fn parse_while_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // while_statement = "while", "(", expression, ")", statement_block;

        let while_token = try_consume_token!(self, TokenCategory::While);

        let _ = self.consume_must_be(TokenCategory::ParenOpen)?;

        let condition = self
            .parse_expression()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing while statement.")))?;

        let _ = self.consume_must_be(TokenCategory::ParenClose)?;

        let block = self
            .parse_statement_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement block while parsing while statement.")))?;

        let block_end = block.span.end();

        let node = Node {
            value: Statement::WhileLoop { condition, block },
            span: Span::new(while_token.span.start(), block_end),
        };

        Ok(Some(node))
    }
}
