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
    pub(in crate::frontend::parser) fn parse_if_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // if_statement = "if", "(", expression, ")", statement_block, [ "else", statement_block ];

        let if_token = try_consume_token!(self, TokenCategory::If);

        let _ = self.consume_must_be(TokenCategory::ParenOpen)?;

        let condition = self
            .parse_expression()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing if statement.")))?;

        let _ = self.consume_must_be(TokenCategory::ParenClose)?;

        let true_block = self
            .parse_statement_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement block while parsing if statement.")))?;

        let false_block = match self.consume_if_matches(TokenCategory::Else)? {
            Some(_) => self.parse_statement_block()?,
            None => None,
        };

        let end = false_block
            .as_ref()
            .map(|block| block.span.end())
            .unwrap_or_else(|| true_block.span.end());

        let node = Node {
            value: Statement::Conditional {
                condition,
                if_block: true_block,
                else_block: false_block,
            },
            span: Span::new(if_token.span.start(), end),
        };

        Ok(Some(node))
    }
}
