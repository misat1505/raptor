use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::frontend::{
        ast::{Expression, Literal},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    };

#[test]
fn parse_unary_term() {
    let token_series = [
        vec![
            // !True
            create_token(TokenCategory::Negate, TokenValue::Null),
            create_token(TokenCategory::True, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // -5
            create_token(TokenCategory::Minus, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // 5
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        Expression::BooleanNegation(Box::new(test_node!(Expression::Literal(Literal::True)))),
        Expression::ArithmeticNegation(Box::new(test_node!(Expression::Literal(Literal::I64(5))))),
        Expression::Literal(Literal::I64(5)),
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_unary_term().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}
