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
    pub(in crate::frontend::parser) fn parse_identifier_or_call(&mut self) -> Result<Option<Node<Expression>>, Box<dyn IError>> {
        // identifier_or_call = identifier, [ "(", arguments, ")" ], { "[", expression, "]" }
        let identifier = try_consume!(self, parse_identifier);

        let position = identifier.position;

        let mut result = match self.consume_if_matches(TokenCategory::ParenOpen)? {
            Some(_) => {
                let args = self.parse_arguments()?.into_iter().map(Box::new).collect();
                let _ = self.consume_must_be(TokenCategory::ParenClose)?;
                Expression::FunctionCall { identifier, arguments: args }
            }
            None => Expression::Variable(identifier.value),
        };

        while self.current_token().category == TokenCategory::BracketOpen {
            let _ = self.consume_must_be(TokenCategory::BracketOpen);
            let index_expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Expected an expression inside '[]' index.")))?;
            let _ = self.consume_must_be(TokenCategory::BracketClose)?;

            result = Expression::Index {
                collection: Box::new(Node { value: result, position }),
                index: Box::new(index_expr),
            };
        }

        Ok(Some(Node { value: result, position }))
    }
}
