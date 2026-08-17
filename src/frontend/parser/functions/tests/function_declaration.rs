use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::{
    common::types::Type,
    frontend::{
        ast::{Block, FunctionDeclaration},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    },
};

#[test]
fn parse_function_declaration_fail() {
    let series = vec![
        // fn add(): , {}
        create_token(TokenCategory::Fn, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::ParenClose, TokenValue::Null),
        create_token(TokenCategory::Colon, TokenValue::Null),
        create_token(TokenCategory::Comma, TokenValue::Null),
        create_token(TokenCategory::BraceOpen, TokenValue::Null),
        create_token(TokenCategory::BraceClose, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(series);
    let mut parser = Parser::new(mock_lexer);

    assert!(parser.parse_function_declaration().is_err());
}

#[test]
fn parse_function_declaration() {
    let token_series = [
        vec![
            // fn add(): i64 {}
            create_token(TokenCategory::Fn, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // fn add(): void {}
            create_token(TokenCategory::Fn, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::Void, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        FunctionDeclaration {
            identifier: test_node!(String::from("add")),
            parameters: vec![],
            return_type: test_node!(Type::I64),
            block: test_node!(Block(vec![])),
        },
        FunctionDeclaration {
            identifier: test_node!(String::from("add")),
            parameters: vec![],
            return_type: test_node!(Type::Void),
            block: test_node!(Block(vec![])),
        },
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_function_declaration().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}
