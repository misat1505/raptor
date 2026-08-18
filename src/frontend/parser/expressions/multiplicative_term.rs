use crate::{
    common::errors::IError,
    frontend::{
        ast::{Expression, Node},
        lexer::lexer::ILexer,
        parser::{core::try_consume, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_multiplicative_term(&mut self) -> Result<Option<Node<Expression>>, Box<dyn IError>> {
        // multiplicative_term = casted_term, { ("*" | "/" | "%"), casted_term };
        let mut left_side = try_consume!(self, parse_casted_term);

        let mut current_token = self.current_token();
        while current_token.category == TokenCategory::Multiply
            || current_token.category == TokenCategory::Divide
            || current_token.category == TokenCategory::Modulo
        {
            let _ = self.next_token()?;
            let right_side = self
                .parse_casted_term()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create casted term while parsing multiplicative term.")))?;

            let expression_type = match current_token.category {
                TokenCategory::Multiply => Expression::Multiplication(Box::new(left_side), Box::new(right_side)),
                TokenCategory::Divide => Expression::Division(Box::new(left_side), Box::new(right_side)),
                TokenCategory::Modulo => Expression::Modulo(Box::new(left_side), Box::new(right_side)),
                _ => unreachable!(),
            };

            left_side = Node {
                value: expression_type,
                span: Span::new(left_side.span.start(), right_side.span.end()),
            };
            current_token = self.current_token();
        }
        Ok(Some(left_side))
    }
}
