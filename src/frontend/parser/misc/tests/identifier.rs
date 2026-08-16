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
fn parse_identifier() {
    let tokens = vec![
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let node = parser.parse_identifier().unwrap().unwrap();
    assert_eq!(node.value, String::from("print"));
}

#[test]
fn parse_identifier_bad_value_type() {
    let tokens = vec![
        // 5 is not string
        create_token(TokenCategory::Identifier, TokenValue::I64(5)),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let result = parser.parse_identifier();
    assert!(result.is_err());
}
