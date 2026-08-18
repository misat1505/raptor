use crate::{
    common::errors::IError,
    frontend::{
        ast::{Expression, Node},
        lexer::lexer::ILexer,
        parser::Parser,
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_unary_term(&mut self) -> Result<Option<Node<Expression>>, Box<dyn IError>> {
        // unary_term = [ ("-", "!") ], factor;
        let start_pos = self.current_token().span.start();

        if let Some(token) = self.consume_if_matches(TokenCategory::Negate)? {
            let factor = self.parse_unary_term_factor()?;
            return Ok(Some(Node {
                value: Expression::BooleanNegation(Box::new(factor)),
                span: Span::new(start_pos, token.span.end()),
            }));
        }

        if let Some(token) = self.consume_if_matches(TokenCategory::Minus)? {
            let factor = self.parse_unary_term_factor()?;
            return Ok(Some(Node {
                value: Expression::ArithmeticNegation(Box::new(factor)),
                span: Span::new(start_pos, factor.span.end()),
            }));
        }

        let factor = self.parse_factor()?;
        Ok(factor)
    }

    fn parse_unary_term_factor(&mut self) -> Result<Node<Expression>, Box<dyn IError>> {
        match self.parse_factor()? {
            Some(t) => Ok(t),
            None => return Err(self.create_parser_error(String::from("Couldn't create factor while parsing unary term."))),
        }
    }
}
