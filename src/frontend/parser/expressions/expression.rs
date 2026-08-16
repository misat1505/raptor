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
    pub(in crate::frontend::parser) fn parse_expression(&mut self) -> Result<Option<Node<Expression>>, Box<dyn IError>> {
        // expression = concatenation_term { "||", concatenation_term };
        let mut left_side = try_consume!(self, parse_concatenation_term);

        let mut current_token = self.current_token();
        while current_token.category == TokenCategory::Or {
            let _ = self.next_token()?;
            let right_side = self
                .parse_concatenation_term()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create concatenation term while parsing expression.")))?;

            let expression_type = Expression::Alternative(Box::new(left_side.clone()), Box::new(right_side.clone()));
            left_side = Node {
                value: expression_type,
                position: current_token.position,
            };
            current_token = self.current_token();
        }
        Ok(Some(left_side))
    }
}
