use super::{create_token, test_node, LexerMock};
use crate::{
    common::types::Type,
    frontend::{
        ast::{Block, Expression, Literal, Statement},
        parser::{IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    },
};

#[test]
fn parse_full_program() {
    let tokens = vec![
        // fn add(): i64 { return 1; }
        // x = 5;
        create_token(TokenCategory::Fn, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::ParenClose, TokenValue::Null),
        create_token(TokenCategory::Colon, TokenValue::Null),
        create_token(TokenCategory::I64, TokenValue::Null),
        create_token(TokenCategory::BraceOpen, TokenValue::Null),
        create_token(TokenCategory::Return, TokenValue::Null),
        create_token(TokenCategory::I64Value, TokenValue::I64(1)),
        create_token(TokenCategory::Semicolon, TokenValue::Null),
        create_token(TokenCategory::BraceClose, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
        create_token(TokenCategory::Assign, TokenValue::Null),
        create_token(TokenCategory::I64Value, TokenValue::I64(5)),
        create_token(TokenCategory::Semicolon, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let program = parser.parse().unwrap();
    assert_eq!(program.statements.len(), 1);
    assert_eq!(
        program.statements[0].value,
        Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Literal(Literal::I64(5))),
            accessors: vec![],
        }
    );

    let add_fn = program.functions.get("add").expect("function 'add' should be registered");
    assert_eq!(add_fn.value.return_type.value, Type::I64);
    assert_eq!(add_fn.value.parameters, vec![]);
    assert_eq!(
        add_fn.value.block.value,
        Block(vec![test_node!(Statement::Return(Some(test_node!(Expression::Literal(Literal::I64(
            1
        ))))))])
    );
}

#[test]
fn parse_program_redeclared_function_fails() {
    let tokens = vec![
        // fn add(): void {} fn add(): void {}
        create_token(TokenCategory::Fn, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::ParenClose, TokenValue::Null),
        create_token(TokenCategory::Colon, TokenValue::Null),
        create_token(TokenCategory::Void, TokenValue::Null),
        create_token(TokenCategory::BraceOpen, TokenValue::Null),
        create_token(TokenCategory::BraceClose, TokenValue::Null),
        create_token(TokenCategory::Fn, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::ParenClose, TokenValue::Null),
        create_token(TokenCategory::Colon, TokenValue::Null),
        create_token(TokenCategory::Void, TokenValue::Null),
        create_token(TokenCategory::BraceOpen, TokenValue::Null),
        create_token(TokenCategory::BraceClose, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let err = parser.parse().unwrap_err();
    assert_eq!(err.message(), format!("Redeclaration of 'add'."));
}

#[test]
fn parse_program_function_shadows_std_function_fails() {
    let tokens = vec![
        // fn print(): void {}
        create_token(TokenCategory::Fn, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::ParenClose, TokenValue::Null),
        create_token(TokenCategory::Colon, TokenValue::Null),
        create_token(TokenCategory::Void, TokenValue::Null),
        create_token(TokenCategory::BraceOpen, TokenValue::Null),
        create_token(TokenCategory::BraceClose, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let err = parser.parse().unwrap_err();
    assert_eq!(err.message(), format!("Redeclaration of 'print'."));
}
