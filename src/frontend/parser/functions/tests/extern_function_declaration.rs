use crate::frontend::parser::tests::{create_token, test_node, LexerMock};
use crate::{
    common::types::Type,
    frontend::{
        ast::{ExternFunctionDeclaration, Parameter, PassedBy},
        parser::{IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    },
};

#[test]
fn parse_extern_function_declaration_fail() {
    let token_series = [
        vec![
            // extern add(): i64;  (brak 'fn' po 'extern')
            create_token(TokenCategory::Extern, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // extern fn add(): i64  (brak ';' na końcu)
            create_token(TokenCategory::Extern, TokenValue::Null),
            create_token(TokenCategory::Fn, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // extern fn add(): i64 as;  (brak identyfikatora po 'as')
            create_token(TokenCategory::Extern, TokenValue::Null),
            create_token(TokenCategory::Fn, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::As, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    for series in token_series {
        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_extern_function_declaration().is_err());
    }
}

#[test]
fn parse_extern_function_declaration_none_when_no_extern_keyword() {
    // fn add(): i64 {}  -- brak 'extern' na początku, funkcja powinna zwrócić None
    let tokens = vec![
        create_token(TokenCategory::Fn, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::ParenClose, TokenValue::Null),
        create_token(TokenCategory::Colon, TokenValue::Null),
        create_token(TokenCategory::I64, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    assert!(parser.parse_extern_function_declaration().unwrap().is_none());
}

#[test]
fn parse_extern_function_declaration() {
    let token_series = [
        vec![
            // extern fn add(): i64;
            create_token(TokenCategory::Extern, TokenValue::Null),
            create_token(TokenCategory::Fn, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // extern fn add(): void;
            create_token(TokenCategory::Extern, TokenValue::Null),
            create_token(TokenCategory::Fn, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::Void, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // extern fn add(i64 x): i64;
            create_token(TokenCategory::Extern, TokenValue::Null),
            create_token(TokenCategory::Fn, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // extern fn add(): i64 as my_add;
            create_token(TokenCategory::Extern, TokenValue::Null),
            create_token(TokenCategory::Fn, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::As, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("my_add"))),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        ExternFunctionDeclaration {
            identifier: test_node!(String::from("add")),
            parameters: vec![],
            return_type: test_node!(Type::I64),
            alias: None,
        },
        ExternFunctionDeclaration {
            identifier: test_node!(String::from("add")),
            parameters: vec![],
            return_type: test_node!(Type::Void),
            alias: None,
        },
        ExternFunctionDeclaration {
            identifier: test_node!(String::from("add")),
            parameters: vec![test_node!(Parameter {
                passed_by: PassedBy::Value,
                parameter_type: test_node!(Type::I64),
                identifier: test_node!(String::from("x")),
            })],
            return_type: test_node!(Type::I64),
            alias: None,
        },
        ExternFunctionDeclaration {
            identifier: test_node!(String::from("add")),
            parameters: vec![],
            return_type: test_node!(Type::I64),
            alias: Some(test_node!(String::from("my_add"))),
        },
    ];

    for (idx, series) in token_series.into_iter().enumerate() {
        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_extern_function_declaration().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}
