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
fn parse_continue_statement_fail() {
    let series = vec![
        // continue
        create_token(TokenCategory::Continue, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(series);
    let mut parser = Parser::new(mock_lexer);

    assert!(parser.parse_continue_statement().is_err());
}

#[test]
fn parse_continue_statement() {
    let tokens = vec![
        // continue;
        create_token(TokenCategory::Continue, TokenValue::Null),
        create_token(TokenCategory::Semicolon, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let node = parser.parse_continue_statement().unwrap().unwrap();
    assert_eq!(node.value, Statement::Continue);
}
