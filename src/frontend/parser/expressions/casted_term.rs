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
    pub(in crate::frontend::parser) fn parse_casted_term(&mut self) -> Result<Option<Node<Expression>>, Box<dyn IError>> {
        // casted_term = unary_term, [ "as", type ];
        let unary_term = try_consume!(self, parse_unary_term);

        let position = unary_term.position.clone();
        match self.consume_if_matches(TokenCategory::As)? {
            Some(_) => {
                let type_parsed = self
                    .parse_type()?
                    .ok_or_else(|| self.create_parser_error(String::from("Couldn't parse type.")))?;

                Ok(Some(Node {
                    value: Expression::Casting {
                        value: Box::new(unary_term),
                        to_type: type_parsed,
                    },
                    span: Span::new(unary_term.span.start(), type_parsed.span.end()),
                }))
            }
            None => Ok(Some(unary_term)),
        }
    }
}
