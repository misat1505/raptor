use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::frontend::{
        ast::{Block, Expression, Literal, Statement},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    };

#[test]
fn parse_while_statement_fail() {
    let token_series = [
        vec![
            // while true) {}
            create_token(TokenCategory::While, TokenValue::Null),
            create_token(TokenCategory::True, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // while (true {}
            create_token(TokenCategory::While, TokenValue::Null),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::True, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    for series in token_series {
        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_while_statement().is_err());
    }
}

#[test]
fn parse_while_statement() {
    let tokens = vec![
        // while (true) {}
        create_token(TokenCategory::While, TokenValue::Null),
        create_token(TokenCategory::ParenOpen, TokenValue::Null),
        create_token(TokenCategory::True, TokenValue::Null),
        create_token(TokenCategory::ParenClose, TokenValue::Null),
        create_token(TokenCategory::BraceOpen, TokenValue::Null),
        create_token(TokenCategory::BraceClose, TokenValue::Null),
        create_token(TokenCategory::ETX, TokenValue::Null),
    ];

    let expected = Statement::WhileLoop {
        condition: test_node!(Expression::Literal(Literal::True)),
        block: test_node!(Block(vec![])),
    };

    let mock_lexer = LexerMock::new(tokens);
    let mut parser = Parser::new(mock_lexer);

    let node = parser.parse_while_statement().unwrap().unwrap();
    assert_eq!(node.value, expected);
}
