use crate::{
    common::errors::IError,
    frontend::{
        ast::{Literal, Node},
        lexer::lexer::ILexer,
        parser::Parser,
        tokens::{TokenCategory, TokenValue},
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_literal(&mut self) -> Result<Option<Node<Literal>>, Box<dyn IError>> {
        let token = self.current_token();

        let literal = match (token.category, token.value) {
            (TokenCategory::True, _) => Literal::True,
            (TokenCategory::False, _) => Literal::False,
            (TokenCategory::I64Value, TokenValue::I64(int)) => Literal::I64(int),
            (TokenCategory::F64Value, TokenValue::F64(float)) => Literal::F64(float),
            (TokenCategory::StringValue, TokenValue::String(string)) => Literal::String(string),
            (TokenCategory::CharValue, TokenValue::Char(c)) => Literal::Char(c),
            _ => return Ok(None),
        };

        let _ = self.next_token();

        let node = Node {
            value: literal,
            span: token.span,
        };
        Ok(Some(node))
    }
}
