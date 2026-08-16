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
fn parse_break_statement_fail() {
    let series = vec![
        // break
        create_token(TokenCategory::Break, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(series);
    let mut parser = Parser::new(mock_lexer);

    assert!(parser.parse_break_statement().is_err());
}

#[test]
fn parse_break_statement() {
    let tokens = vec![
        // break;
        create_token(TokenCategory::Break, TokenValue::Null),
        create_token(TokenCategory::Semicolon, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let node = parser.parse_break_statement().unwrap().unwrap();
    assert_eq!(node.value, Statement::Break);
}
