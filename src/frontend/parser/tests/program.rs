use super::{create_token, LexerMock};
use crate::{
    common::types::Type,
    frontend::{
        ast::{Block, Expression, Literal, Statement, SwitchCase, SwitchExpression, VariableDeclarationKind},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    },
};

#[test]
fn parse_statement_fail() {
    let series = vec![
        // i64 a = 5
        create_token(TokenCategory::I64, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("a"))),
        create_token(TokenCategory::Assign, TokenValue::Null),
        create_token(TokenCategory::I64Value, TokenValue::I64(5)),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(series);
    let mut parser = Parser::new(mock_lexer);

    assert!(parser.parse_statement().is_err());
}

#[test]
fn parse_statement() {
    let token_series = [
        vec![
            // x = 5;
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Assign, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // print();
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // if (true) {}
            create_token(TokenCategory::If, TokenValue::Null),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::True, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // for(;true;) {}
            create_token(TokenCategory::For, TokenValue::Null),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::True, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
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
        ],
        vec![
            // return;
            create_token(TokenCategory::Return, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // break;
            create_token(TokenCategory::Break, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // i64 a = 5;
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("a"))),
            create_token(TokenCategory::Assign, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Literal(Literal::I64(5))),
            accessors: vec![],
        },
        Statement::FunctionCall {
            identifier: test_node!(String::from("print")),
            arguments: vec![],
        },
        Statement::Conditional {
            condition: test_node!(Expression::Literal(Literal::True)),
            if_block: test_node!(Block(vec![])),
            else_block: None,
        },
        Statement::ForLoop {
            declaration: None,
            condition: test_node!(Expression::Literal(Literal::True)),
            assignment: None,
            block: test_node!(Block(vec![])),
        },
        Statement::Switch {
            expressions: vec![test_node!(SwitchExpression {
                expression: test_node!(Expression::Variable(String::from("x"))),
                alias: None,
            })],
            cases: vec![test_node!(SwitchCase {
                condition: test_node!(Expression::Literal(Literal::True)),
                block: test_node!(Block(vec![])),
            })],
        },
        Statement::Return(None),
        Statement::Break,
        Statement::Declaration {
            identifier: test_node!(String::from("a")),
            kind: VariableDeclarationKind::TYPE {
                var_type: test_node!(Type::I64),
                value: Some(test_node!(Expression::Literal(Literal::I64(5)))),
            },
        },
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_statement().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}
