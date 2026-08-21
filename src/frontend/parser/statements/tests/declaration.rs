use crate::frontend::ast::VariableDeclarationKind;
use crate::frontend::parser::tests::{create_token, LexerMock};
use crate::{
    common::types::Type,
    frontend::{
        ast::{Expression, Literal, Statement},
        parser::{tests::test_node, IParser, Parser},
        tokens::{TokenCategory, TokenValue},
    },
};

#[test]
fn parse_declaration() {
    let token_series = [
        vec![
            // i64 a
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("a"))),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
        vec![
            // i64 a = 5
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("a"))),
            create_token(TokenCategory::Assign, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ],
    ];

    let expected = [
        Statement::Declaration {
            identifier: test_node!(String::from("a")),
            kind: VariableDeclarationKind::TYPE {
                var_type: test_node!(Type::I64),
                value: None,
            },
        },
        Statement::Declaration {
            identifier: test_node!(String::from("a")),
            kind: VariableDeclarationKind::TYPE {
                var_type: test_node!(Type::I64),
                value: Some(test_node!(Expression::Literal(Literal::I64(5)))),
            },
        },
    ];

    for (idx, series) in token_series.iter().enumerate() {
        let mock_lexer = LexerMock::new(series.to_vec());
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_declaration().unwrap().unwrap();
        assert_eq!(node.value, expected[idx]);
    }
}
