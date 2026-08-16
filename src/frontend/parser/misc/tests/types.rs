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
