use crate::{
    common::errors::IError,
    frontend::{
        ast::{Expression, Node},
        lexer::lexer::ILexer,
        parser::{core::try_consume_token, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_vector_literal(&mut self) -> Result<Option<Node<Expression>>, Box<dyn IError>> {
        // vector_literal = "[", [ expression, { ",", expression } ], "]";
        let open_bracket_token = try_consume_token!(self, TokenCategory::BracketOpen);

        let mut expressions: Vec<Box<Node<Expression>>> = vec![];
        if let Ok(Some(expr)) = self.parse_expression() {
            expressions.push(Box::new(expr));

            while self.current_token().category == TokenCategory::Comma {
                let _ = self.consume_must_be(TokenCategory::Comma)?;
                let expression = self
                    .parse_expression()?
                    .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing vector literal.")))?;

                expressions.push(Box::new(expression));
            }
        }
        let close_bracket_token = self.consume_must_be(TokenCategory::BracketClose)?;

        let node = Node {
            value: Expression::Vector(expressions),
            span: Span::new(open_bracket_token.span.start(), close_bracket_token.span.end()),
        };
        Ok(Some(node))
    }
}
