use crate::frontend::ast::VariableDeclarationKind;
use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::{
    common::types::Type,
    frontend::{
        ast::{Block, Expression, Literal, Statement},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    },
};

#[test]
fn parse_for_statement_fail() {
    let token_series = [
        vec![
            // for (
            create_token(TokenCategory::For, TokenValue::Null),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // for (;;) {}
            create_token(TokenCategory::For, TokenValue::Null),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            //  for (;x; {}
            create_token(TokenCategory::For, TokenValue::Null),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    for idx in 0..token_series.len() {
        let mock_lexer = LexerMock::new(token_series[idx].clone());
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_for_statement().is_err());
    }
}

#[test]
fn parse_for_statement() {
    let token_series = [
        vec![
            // for (i64 x = 0; x < 5; x = x + 1) {}
            create_token(TokenCategory::For, TokenValue::Null),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Assign, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(0)),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Less, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Assign, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Plus, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // for (;x < 5;) {}
            create_token(TokenCategory::For, TokenValue::Null),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Less, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        Statement::ForLoop {
            declaration: Some(Box::new(test_node!(Statement::Declaration {
                identifier: test_node!(String::from("x")),
                kind: VariableDeclarationKind::TYPE {
                    var_type: test_node!(Type::I64),
                    value: Some(test_node!(Expression::Literal(Literal::I64(0)))),
                },
            }))),
            condition: test_node!(Expression::Less(
                Box::new(test_node!(Expression::Variable(String::from("x")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            )),
            assignment: Some(Box::new(test_node!(Statement::Assignment {
                identifier: test_node!(String::from("x")),
                value: test_node!(Expression::Addition(
                    Box::new(test_node!(Expression::Variable(String::from("x")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
                )),
                indices: vec![]
            }))),
            block: test_node!(Block(vec![])),
        },
        Statement::ForLoop {
            declaration: None,
            condition: test_node!(Expression::Less(
                Box::new(test_node!(Expression::Variable(String::from("x")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            )),
            assignment: None,
            block: test_node!(Block(vec![])),
        },
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_for_statement().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}
