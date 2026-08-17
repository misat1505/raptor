use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::frontend::{
        ast::Expression,
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    };

#[test]
fn parse_expression() {
    let tokens = vec![
        // a || b || c
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("a"))),
        create_token(TokenCategory::Or, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("b"))),
        create_token(TokenCategory::Or, TokenValue::Null),
        create_token(TokenCategory::Identifier, TokenValue::String(String::from("c"))),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let node = parser.parse_expression().unwrap().unwrap();
    assert_eq!(
        node,
        test_node!(Expression::Alternative(
            Box::new(test_node!(Expression::Alternative(
                Box::new(test_node!(Expression::Variable(String::from("a")))),
                Box::new(test_node!(Expression::Variable(String::from("b")))),
            ))),
            Box::new(test_node!(Expression::Variable(String::from("c")))),
        ))
    );
}
