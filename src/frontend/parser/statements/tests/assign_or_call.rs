use crate::frontend::ast::Accessor;
use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::frontend::{
    ast::{Expression, Literal, Statement},
    parser::{tests::test_node, IParser, Parser},
    tokens::{TokenCategory, TokenValue},
};

#[test]
fn parse_assign_or_call_fail() {
    let token_series = [
        vec![
            // print(;
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // print()
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // x = 5
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Assign, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Comma, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    for idx in 0..token_series.len() {
        let mock_lexer = LexerMock::new(token_series[idx].clone());
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_assign_or_call().is_err());
    }
}

#[test]
fn parse_assign_or_call() {
    let token_series = [
        vec![
            // print();
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // x = 5;
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Assign, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        Statement::FunctionCall {
            identifier: test_node!(String::from("print")),
            arguments: vec![],
        },
        Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Literal(Literal::I64(5))),
            accessors: vec![],
        },
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_assign_or_call().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}

#[test]
fn parse_compound_assignments() {
    let token_series = [
        vec![
            // x += 1;
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::PlusEquals, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // x -= 1;
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::MinusEquals, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // x *= 2;
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::TimesEquals, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(2)),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // x /= 2;
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::DivideEquals, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(2)),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // x %= 2;
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::ModuloEquals, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(2)),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Addition(
                Box::new(test_node!(Expression::Variable(String::from("x")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
            )),
            accessors: vec![],
        },
        Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Subtraction(
                Box::new(test_node!(Expression::Variable(String::from("x")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
            )),
            accessors: vec![],
        },
        Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Multiplication(
                Box::new(test_node!(Expression::Variable(String::from("x")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
            )),
            accessors: vec![],
        },
        Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Division(
                Box::new(test_node!(Expression::Variable(String::from("x")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
            )),
            accessors: vec![],
        },
        Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Modulo(
                Box::new(test_node!(Expression::Variable(String::from("x")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
            )),
            accessors: vec![],
        },
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_assign_or_call().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}

#[test]
fn parse_compound_assignment_with_index() {
    // x[0] += 1;
    let tokens = vec![
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
        create_token(TokenCategory::BracketOpen, TokenValue::Null),
        create_token(TokenCategory::I64Value, TokenValue::I64(0)),
        create_token(TokenCategory::BracketClose, TokenValue::Null),
        create_token(TokenCategory::PlusEquals, TokenValue::Null),
        create_token(TokenCategory::I64Value, TokenValue::I64(1)),
        create_token(TokenCategory::Semicolon, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let expected = Statement::Assignment {
        identifier: test_node!(String::from("x")),
        accessors: vec![test_node!(Accessor::Index(test_node!(Expression::Literal(Literal::I64(0)))))],
        value: test_node!(Expression::Addition(
            Box::new(test_node!(Expression::Index {
                collection: Box::new(test_node!(Expression::Variable(String::from("x")))),
                index: Box::new(test_node!(Expression::Literal(Literal::I64(0)))),
            })),
            Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
        )),
    };

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let node = parser.parse_assign_or_call().unwrap().unwrap();
    assert_eq!(node.value, expected);
}

#[test]
fn parse_assignment_with_index() {
    // x[0] = 5;
    let tokens = vec![
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
        create_token(TokenCategory::BracketOpen, TokenValue::Null),
        create_token(TokenCategory::I64Value, TokenValue::I64(0)),
        create_token(TokenCategory::BracketClose, TokenValue::Null),
        create_token(TokenCategory::Assign, TokenValue::Null),
        create_token(TokenCategory::I64Value, TokenValue::I64(5)),
        create_token(TokenCategory::Semicolon, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let expected = Statement::Assignment {
        identifier: test_node!(String::from("x")),
        accessors: vec![test_node!(Accessor::Index(test_node!(Expression::Literal(Literal::I64(0)))))],
        value: test_node!(Expression::Literal(Literal::I64(5))),
    };

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let node = parser.parse_assign_or_call().unwrap().unwrap();
    assert_eq!(node.value, expected);
}
