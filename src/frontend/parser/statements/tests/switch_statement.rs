use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::frontend::{
        ast::{Block, Expression, Literal, Statement, SwitchCase, SwitchExpression},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    };

#[test]
fn parse_switch_statement() {
    let series = vec![
        // switch(x) {
        //      (true) -> {}
        // }
        create_token(TokenCategory::Switch, TokenValue::Null),
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
        create_token(TokenCategory::ParenClose, TokenValue::Null),
        create_token(TokenCategory::BraceOpen, TokenValue::Null),
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::True, TokenValue::Null),
        create_token(TokenCategory::ParenClose, TokenValue::Null),
        create_token(TokenCategory::Arrow, TokenValue::Null),
        create_token(TokenCategory::BraceOpen, TokenValue::Null),
        create_token(TokenCategory::BraceClose, TokenValue::Null),
        create_token(TokenCategory::BraceClose, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let expected = Statement::Switch {
        expressions: vec![test_node!(SwitchExpression {
            expression: test_node!(Expression::Variable(String::from("x"))),
            alias: None,
        })],
        cases: vec![test_node!(SwitchCase {
            condition: test_node!(Expression::Literal(Literal::True)),
            block: test_node!(Block(vec![])),
        })],
    };

    let mock_lexer = LexerMock::new(series);
    let mut parser = Parser::new(mock_lexer);

    let node = parser.parse_switch_statement().unwrap().unwrap();
    assert_eq!(node.value, expected);
}

#[test]
fn parse_switch_expressions_fail() {
    let series = vec![
        // x: temp,
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
        create_token(TokenCategory::Colon, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("temp"))),
        create_token(TokenCategory::Comma, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(series);
    let mut parser = Parser::new(mock_lexer);

    assert!(parser.parse_switch_expressions().is_err());
}

#[test]
fn parse_switch_expressions() {
    let token_series = [
        vec![
            // x: temp, y
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("temp"))),
            create_token(TokenCategory::Comma, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("y"))),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // x
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected_types = [
        vec![
            test_node!(SwitchExpression {
                expression: test_node!(Expression::Variable(String::from("x"))),
                alias: Some(test_node!(String::from("temp"))),
            }),
            test_node!(SwitchExpression {
                expression: test_node!(Expression::Variable(String::from("y"))),
                alias: None,
            }),
        ],
        vec![test_node!(SwitchExpression {
            expression: test_node!(Expression::Variable(String::from("x"))),
            alias: None,
        })],
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let vector = parser.parse_switch_expressions().unwrap();
        assert_eq!(vector, expected_types[idx]);
    }
}

#[test]
fn parse_switch_expression() {
    let token_series = [
        vec![
            // x: temp
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("temp"))),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // x
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected_types = [
        SwitchExpression {
            expression: test_node!(Expression::Variable(String::from("x"))),
            alias: Some(test_node!(String::from("temp"))),
        },
        SwitchExpression {
            expression: test_node!(Expression::Variable(String::from("x"))),
            alias: None,
        },
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_switch_expression().unwrap().unwrap();
        assert_eq!(node.value, expected_types[idx]);
    }
}

#[test]
fn parse_switch_case() {
    let series = vec![
        // (true) -> {}
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::True, TokenValue::Null),
        create_token(TokenCategory::ParenClose, TokenValue::Null),
        create_token(TokenCategory::Arrow, TokenValue::Null),
        create_token(TokenCategory::BraceOpen, TokenValue::Null),
        create_token(TokenCategory::BraceClose, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let expected = SwitchCase {
        condition: test_node!(Expression::Literal(Literal::True)),
        block: test_node!(Block(vec![])),
    };

    let mock_lexer = LexerMock::new(series);
    let mut parser = Parser::new(mock_lexer);

    let node = parser.parse_switch_case().unwrap().unwrap();
    assert_eq!(node.value, expected);
}
