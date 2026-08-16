use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::frontend::{
        ast::{Expression, Literal},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    };

#[test]
fn parse_vector_literal() {
    let token_series = [
        vec![
            // []
            create_token(TokenCategory::BracketOpen, TokenValue::Null),
            create_token(TokenCategory::BracketClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // [1, 2, 3]
            create_token(TokenCategory::BracketOpen, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::Comma, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(2)),
            create_token(TokenCategory::Comma, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(3)),
            create_token(TokenCategory::BracketClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        Expression::Vector(vec![]),
        Expression::Vector(vec![
            Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(3)))),
        ]),
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_vector_literal().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}
