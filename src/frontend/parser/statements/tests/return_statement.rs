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
fn parse_return_statement_fail() {
    let token_series = [
        vec![
            // return
            create_token(TokenCategory::Return, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // return 5
            create_token(TokenCategory::Return, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    for series in token_series {
        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_return_statement().is_err());
    }
}

#[test]
fn parse_return_statement() {
    let token_series = [
        vec![
            // return;
            create_token(TokenCategory::Return, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // return 5;
            create_token(TokenCategory::Return, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        Statement::Return(None),
        Statement::Return(Some(test_node!(Expression::Literal(Literal::I64(5))))),
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_return_statement().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}
