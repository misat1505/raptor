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
    pub(in crate::frontend::parser) fn parse_relation_term(&mut self) -> Result<Option<Node<Expression>>, Box<dyn IError>> {
        // relation_term = additive_term, [ relation_operands, additive_term ];
        let left_side = try_consume!(self, parse_additive_term);

        let operands = [
            TokenCategory::Equal,
            TokenCategory::NotEqual,
            TokenCategory::Greater,
            TokenCategory::GreaterOrEqual,
            TokenCategory::Less,
            TokenCategory::LessOrEqual,
        ];

        let current_token = self.current_token();
        if !operands.contains(&current_token.category) {
            return Ok(Some(left_side));
        }

        let _ = self.next_token()?;
        let right_side = self
            .parse_additive_term()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create additive term while parsing relation term.")))?;

        let box_l = Box::new(left_side.clone());
        let box_r = Box::new(right_side);

        let expr = match current_token.category {
            TokenCategory::Equal => Expression::Equal(box_l, box_r),
            TokenCategory::NotEqual => Expression::NotEqual(box_l, box_r),
            TokenCategory::Greater => Expression::Greater(box_l, box_r),
            TokenCategory::GreaterOrEqual => Expression::GreaterEqual(box_l, box_r),
            TokenCategory::Less => Expression::Less(box_l, box_r),
            TokenCategory::LessOrEqual => Expression::LessEqual(box_l, box_r),
            _ => return Err(self.create_parser_error(String::from("Couldn't create additive term while parsing relation term."))),
        };

        let node = Node {
            value: expr,
            position: left_side.position,
        };
        Ok(Some(node))
    }
}
