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
    pub fn parse_additive_term(&mut self) -> Result<Option<Node<Expression>>, Box<dyn IError>> {
        // additive_term = multiplicative_term , { ("+" | "-"), multiplicative_term };
        let mut left_side = try_consume!(self, parse_multiplicative_term);

        let mut current_token = self.current_token();
        while current_token.category == TokenCategory::Plus || current_token.category == TokenCategory::Minus {
            let _ = self.next_token()?;
            let right_side = self
                .parse_multiplicative_term()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create multiplicative term while parsing additive term.")))?;

            let mut expression_type = Expression::Addition(Box::new(left_side.clone()), Box::new(right_side.clone()));
            if current_token.category == TokenCategory::Minus {
                expression_type = Expression::Subtraction(Box::new(left_side), Box::new(right_side))
            }
            left_side = Node {
                value: expression_type,
                position: current_token.position,
            };
            current_token = self.current_token();
        }
        Ok(Some(left_side))
    }
}
