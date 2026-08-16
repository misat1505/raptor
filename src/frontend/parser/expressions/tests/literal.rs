use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::{
    common::types::Type,
    frontend::{
        ast::{Block, Expression, Literal, Statement, SwitchCase, SwitchExpression},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    },
};

#[test]
fn parse_literals() {
    let tokens = vec![
        create_token(TokenCategory::True, TokenValue::Null),
        create_token(TokenCategory::False, TokenValue::Null),
        create_token(TokenCategory::StringValue, TokenValue::String(String::from("a"))),
        create_token(TokenCategory::I64Value, TokenValue::I64(5)),
        create_token(TokenCategory::F64Value, TokenValue::F64(5.0)),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let mut literal = parser.parse_literal().unwrap().unwrap();
    assert_eq!(literal.value, Literal::True);

    literal = parser.parse_literal().unwrap().unwrap();
    assert_eq!(literal.value, Literal::False);

    literal = parser.parse_literal().unwrap().unwrap();
    assert_eq!(literal.value, Literal::String(String::from("a")));

    literal = parser.parse_literal().unwrap().unwrap();
    assert_eq!(literal.value, Literal::I64(5));

    literal = parser.parse_literal().unwrap().unwrap();
    assert_eq!(literal.value, Literal::F64(5.0));
}
