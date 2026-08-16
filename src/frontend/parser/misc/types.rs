
use crate::{
    common::{
        errors::{ErrorSeverity, IError, ParserError},
        types::Type,
    },
    frontend::{
        ast::Node,
        lexer::lexer::ILexer,
        parser::Parser,
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub fn parse_type(&mut self) -> Result<Option<Node<Type>>, Box<dyn IError>> {
        let token = self.current_token();

        let mut result = match token.category {
            TokenCategory::Bool => Type::Bool,
            TokenCategory::String => Type::Str,
            TokenCategory::I64 => Type::I64,
            TokenCategory::F64 => Type::F64,
            _ => return Ok(None),
        };

        let _ = self.next_token()?;
        while self.current_token().category == TokenCategory::BracketOpen {
            self.consume_must_be(TokenCategory::BracketOpen)?;
            self.consume_must_be(TokenCategory::BracketClose)?;

            result = Type::Vector(Box::new(result));
        }

        Ok(Some(Node {
            value: result,
            position: token.position,
        }))
    }

    pub fn void_type_or_error(&mut self) -> Result<Node<Type>, Box<dyn IError>> {
        match self.consume_if_matches(TokenCategory::Void)? {
            Some(token) => Ok(Node {
                value: Type::Void,
                position: token.position,
            }),
            None => {
                return Err(Box::new(ParserError::expected_found(
                    ErrorSeverity::HIGH,
                    "Bad return type".to_string(),
                    "'i64', 'f64', 'bool', 'str', or 'void'".to_string(),
                    format!("{:?}", self.current_token().category),
                    self.current_token().position,
                )));
            }
        }
    }
}
