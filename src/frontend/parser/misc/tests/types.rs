use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::{
    common::types::Type,
    frontend::{
        parser::{IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    },
};

#[test]
fn parse_type() {
    let token_series = [
        vec![
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            create_token(TokenCategory::F64, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            create_token(TokenCategory::String, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            create_token(TokenCategory::Bool, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected_types = [Type::I64, Type::F64, Type::Str, Type::Bool];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_type().unwrap().unwrap();
        assert_eq!(node.value, expected_types[idx]);
    }
}

#[test]
fn parse_type_fail() {
    let token_series = [
        vec![
            create_token(TokenCategory::Void, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            create_token(TokenCategory::Comma, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    for series in token_series {
        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_type().is_ok());
        assert!(parser.parse_type().unwrap().is_none());
    }
}

#[test]
fn parse_type_vector() {
    let token_series = [
        vec![
            // i64[]
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::BracketOpen, TokenValue::Null),
            create_token(TokenCategory::BracketClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // i64[][]
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::BracketOpen, TokenValue::Null),
            create_token(TokenCategory::BracketClose, TokenValue::Null),
            create_token(TokenCategory::BracketOpen, TokenValue::Null),
            create_token(TokenCategory::BracketClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        Type::Vector(Box::new(Type::I64)),
        Type::Vector(Box::new(Type::Vector(Box::new(Type::I64)))),
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_type().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}

#[test]
fn parse_type_all_scalars() {
    let cases = [
        (TokenCategory::I8, Type::I8),
        (TokenCategory::I16, Type::I16),
        (TokenCategory::I32, Type::I32),
        (TokenCategory::I64, Type::I64),
        (TokenCategory::U8, Type::U8),
        (TokenCategory::U16, Type::U16),
        (TokenCategory::U32, Type::U32),
        (TokenCategory::U64, Type::U64),
        (TokenCategory::F64, Type::F64),
        (TokenCategory::Bool, Type::Bool),
        (TokenCategory::Char, Type::Char),
        (TokenCategory::String, Type::Str),
    ];
    for (cat, expected) in cases {
        let series = vec![create_token(cat, TokenValue::Null), create_token(TokenCategory::ETX, TokenValue::Null)];
        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);
        let node = parser.parse_type().unwrap().unwrap();
        assert_eq!(node.value, expected);
    }
}

#[test]
fn parse_type_vector_of_each_scalar() {
    let scalars = [
        (TokenCategory::Bool, Type::Bool),
        (TokenCategory::Char, Type::Char),
        (TokenCategory::U8, Type::U8),
        (TokenCategory::F64, Type::F64),
    ];
    for (cat, inner) in scalars {
        let series = vec![
            create_token(cat, TokenValue::Null),
            create_token(TokenCategory::BracketOpen, TokenValue::Null),
            create_token(TokenCategory::BracketClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];
        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);
        let node = parser.parse_type().unwrap().unwrap();
        assert_eq!(node.value, Type::Vector(Box::new(inner)));
    }
}

#[test]
fn parse_type_triple_nested_vector() {
    // i64[][][]
    let series = vec![
        create_token(TokenCategory::I64, TokenValue::Null),
        create_token(TokenCategory::BracketOpen, TokenValue::Null),
        create_token(TokenCategory::BracketClose, TokenValue::Null),
        create_token(TokenCategory::BracketOpen, TokenValue::Null),
        create_token(TokenCategory::BracketClose, TokenValue::Null),
        create_token(TokenCategory::BracketOpen, TokenValue::Null),
        create_token(TokenCategory::BracketClose, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];
    let mock_lexer = LexerMock::new(series);
    let mut parser = Parser::new(mock_lexer);
    let node = parser.parse_type().unwrap().unwrap();
    assert_eq!(
        node.value,
        Type::Vector(Box::new(Type::Vector(Box::new(Type::Vector(Box::new(Type::I64))))))
    );
}

#[test]
fn parse_type_missing_closing_bracket_fails() {
    // i64[
    let series = vec![
        create_token(TokenCategory::I64, TokenValue::Null),
        create_token(TokenCategory::BracketOpen, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];
    let mock_lexer = LexerMock::new(series);
    let mut parser = Parser::new(mock_lexer);
    assert!(parser.parse_type().is_err());
}

#[test]
fn void_type_or_error_success() {
    let series = vec![
        create_token(TokenCategory::Void, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];
    let mock_lexer = LexerMock::new(series);
    let mut parser = Parser::new(mock_lexer);
    let node = parser.void_type_or_error().unwrap();
    assert_eq!(node.value, Type::Void);
}

#[test]
fn void_type_or_error_fails_on_other_token() {
    let series = vec![
        create_token(TokenCategory::I64, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];
    let mock_lexer = LexerMock::new(series);
    let mut parser = Parser::new(mock_lexer);
    assert!(parser.void_type_or_error().is_err());
}

#[test]
fn parse_type_returns_none_for_non_type_tokens() {
    let non_types = [
        TokenCategory::Identifier,
        TokenCategory::Plus,
        TokenCategory::Semicolon,
        TokenCategory::Fn,
        TokenCategory::ETX,
    ];
    for cat in non_types {
        let series = vec![create_token(cat, TokenValue::Null), create_token(TokenCategory::ETX, TokenValue::Null)];
        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);
        let result = parser.parse_type().unwrap();
        assert!(result.is_none());
    }
}
