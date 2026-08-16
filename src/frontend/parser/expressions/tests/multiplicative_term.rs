use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::frontend::{
        ast::{Expression, Literal},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    };

#[test]
fn parse_multiplicative_term() {
    let tokens = vec![
        // 5 * 2.0 / x
        create_token(TokenCategory::I64Value, TokenValue::I64(5)),
        create_token(TokenCategory::Multiply, TokenValue::Null),
        create_token(TokenCategory::F64Value, TokenValue::F64(2.0)),
        create_token(TokenCategory::Divide, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let node = parser.parse_multiplicative_term().unwrap().unwrap();
    assert_eq!(
        node,
        test_node!(Expression::Division(
            Box::new(test_node!(Expression::Multiplication(
                Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
                Box::new(test_node!(Expression::Literal(Literal::F64(2.0))))
            ))),
            Box::new(test_node!(Expression::Variable(String::from("x"))))
        ))
    )
}

#[test]
fn parse_multiplicative_term_modulo() {
    let tokens = vec![
        // 5 % 2
        create_token(TokenCategory::I64Value, TokenValue::I64(5)),
        create_token(TokenCategory::Modulo, TokenValue::Null),
        create_token(TokenCategory::I64Value, TokenValue::I64(2)),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let node = parser.parse_multiplicative_term().unwrap().unwrap();
    assert_eq!(
        node.value,
        Expression::Modulo(
            Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
        )
    );
}
