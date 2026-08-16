use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::{
    common::types::Type,
    frontend::{
        ast::{Block, Expression, Literal, Parameter, PassedBy, Statement, SwitchCase, SwitchExpression},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    },
};

#[test]
fn parse_parameters_fail() {
    let tokens = vec![
        // i64 x,
        create_token(TokenCategory::I64, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
        create_token(TokenCategory::Comma, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    assert!(parser.parse_parameters().is_err());
}

#[test]
fn parse_parameters() {
    let token_series = [
        vec![
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // i64 x
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // i64 x, i64 y
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Comma, TokenValue::Null),
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("y"))),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        vec![],
        vec![test_node!(Parameter {
            passed_by: PassedBy::Value,
            parameter_type: test_node!(Type::I64),
            identifier: test_node!(String::from("x")),
        })],
        vec![
            test_node!(Parameter {
                passed_by: PassedBy::Value,
                parameter_type: test_node!(Type::I64),
                identifier: test_node!(String::from("x")),
            }),
            test_node!(Parameter {
                passed_by: PassedBy::Value,
                parameter_type: test_node!(Type::I64),
                identifier: test_node!(String::from("y")),
            }),
        ],
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let vector = parser.parse_parameters().unwrap();
        assert_eq!(vector, expected[idx]);
    }
}

#[test]
fn parse_parameter() {
    let token_series = [
        vec![
            // &i64 x = 0
            create_token(TokenCategory::Reference, TokenValue::Null),
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Assign, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(0)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // i64 x
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        Parameter {
            passed_by: PassedBy::Reference,
            parameter_type: test_node!(Type::I64),
            identifier: test_node!(String::from("x")),
        },
        Parameter {
            passed_by: PassedBy::Value,
            parameter_type: test_node!(Type::I64),
            identifier: test_node!(String::from("x")),
        },
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_parameter().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}
