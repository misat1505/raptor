use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::{
    common::types::Type,
    frontend::{
        ast::{Block, Expression, Literal, Statement, SwitchCase, SwitchExpression},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    },
};

#[test]
fn parse_relation_term() {
    let token_series = [
        vec![
            // 1 == 2
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::Equal, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(2)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // 1 != 2
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::NotEqual, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(2)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // 1 > 2
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::Greater, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(2)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // 1 >= 2
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::GreaterOrEqual, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(2)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // 1 < 2
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::Less, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(2)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // 1 <= 2
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::LessOrEqual, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(2)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // 1
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        Expression::Equal(
            Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
        ),
        Expression::NotEqual(
            Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
        ),
        Expression::Greater(
            Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
        ),
        Expression::GreaterEqual(
            Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
        ),
        Expression::Less(
            Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
        ),
        Expression::LessEqual(
            Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
        ),
        Expression::Literal(Literal::I64(1)),
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_relation_term().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}
