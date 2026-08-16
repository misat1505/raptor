use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::frontend::{
        ast::{Argument, Expression, Literal, PassedBy},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    };

#[test]
fn parse_arguments_comma_end() {
    let tokens = vec![
        // 1,
        create_token(TokenCategory::I64Value, TokenValue::I64(1)),
        create_token(TokenCategory::Comma, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    assert!(parser.parse_arguments().is_err());
}

#[test]
fn parse_arguments() {
    let token_series = [
        vec![
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // 1
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // 1, 2
            create_token(TokenCategory::Reference, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::Comma, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(2)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        vec![],
        vec![test_node!(Argument {
            value: test_node!(Expression::Literal(Literal::I64(1))),
            passed_by: PassedBy::Value
        })],
        vec![
            test_node!(Argument {
                value: test_node!(Expression::Literal(Literal::I64(1))),
                passed_by: PassedBy::Reference
            }),
            test_node!(Argument {
                value: test_node!(Expression::Literal(Literal::I64(2))),
                passed_by: PassedBy::Value
            }),
        ],
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let vector = parser.parse_arguments().unwrap();
        assert_eq!(vector, expected[idx]);
    }
}

#[test]
fn parse_argument() {
    let token_series = [
        vec![
            // 1
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // &x
            create_token(TokenCategory::Reference, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        Argument {
            value: test_node!(Expression::Literal(Literal::I64(1))),
            passed_by: PassedBy::Value,
        },
        Argument {
            value: test_node!(Expression::Variable(String::from("x"))),
            passed_by: PassedBy::Reference,
        },
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_argument().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}
