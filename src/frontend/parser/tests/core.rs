use super::{create_token, LexerMock};
use crate::frontend::{
    parser::{IParser, Parser},
    tokens::{TokenCategory, TokenValue},
};

#[test]
fn consume_must_be() {
    let tokens = vec![
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);
    assert_eq!(parser.current_token().clone().category, TokenCategory::ParenOpen);
    let _ = parser.consume_must_be(TokenCategory::ParenOpen).unwrap();

    assert_eq!(parser.current_token().clone().category, TokenCategory::ETX);
}

#[test]
fn consume_must_be_fail() {
    let tokens = vec![
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);
    assert_eq!(parser.current_token().clone().category, TokenCategory::ParenOpen);
    let result = parser.consume_must_be(TokenCategory::Semicolon);

    assert!(result.is_err());
    assert_eq!(parser.current_token().clone().category, TokenCategory::ParenOpen);
}

#[test]
fn consume_if_matches() {
    let tokens = vec![
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);
    assert_eq!(parser.current_token().clone().category, TokenCategory::ParenOpen);
    let _ = parser.consume_if_matches(TokenCategory::ParenOpen).unwrap();

    assert_eq!(parser.current_token().clone().category, TokenCategory::ETX);
}

#[test]
fn consume_if_matches_fail() {
    let tokens = vec![
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);
    assert_eq!(parser.current_token().clone().category, TokenCategory::ParenOpen);
    let result = parser.consume_if_matches(TokenCategory::Semicolon);

    assert!(result.unwrap().is_none());
    assert_eq!(parser.current_token().clone().category, TokenCategory::ParenOpen);
}
