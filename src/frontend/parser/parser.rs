use std::{collections::HashMap, rc::Rc, unreachable, vec};

use crate::{
    backend::std_functions::std_functions::get_std_functions,
    common::{
        errors::{ErrorSeverity, IError, ParserError},
        types::Type,
    },
    frontend::{
        ast::{
            Argument, Block, Expression, ExternFunctionDeclaration, FunctionDeclaration, Literal, Node, Parameter, PassedBy, Program, Statement,
            SwitchCase, SwitchExpression,
        },
        lexer::lexer::ILexer,
        tokens::{Token, TokenCategory, TokenValue},
    },
};

#[cfg(test)]
mod tests {
    use std::{assert_eq, vec};

    use crate::common::{
        errors::{ErrorSeverity, LexerError},
        position::Position,
    };

    use super::*;

    macro_rules! test_node {
        ($value:expr) => {
            Node {
                value: $value,
                position: default_position(),
            }
        };
    }

    struct LexerMock {
        current_token: Option<Token>,
        pub tokens: Vec<Token>,
    }

    impl LexerMock {
        fn new(mut tokens: Vec<Token>) -> LexerMock {
            let current_token = tokens.remove(0);
            LexerMock {
                current_token: Some(current_token),
                tokens,
            }
        }
    }

    impl ILexer for LexerMock {
        fn current(&self) -> &Option<Token> {
            &self.current_token
        }

        fn next(&mut self) -> Result<Token, Box<dyn IError>> {
            if self.tokens.len() == 0 {
                return Err(Box::new(LexerError::new(ErrorSeverity::HIGH, String::new())));
            }
            let next_token = self.tokens.remove(0);
            self.current_token = Some(next_token.clone());
            Ok(next_token)
        }
    }

    fn default_position() -> Position {
        Position {
            filename: None,
            line: 0,
            column: 0,
            offset: 0,
        }
    }

    fn create_token(category: TokenCategory, value: TokenValue) -> Token {
        Token {
            category,
            value,
            position: default_position(),
        }
    }

    #[test]
    fn parse_statement_block_fail() {
        let series = vec![
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_statement_block().is_err());
    }

    #[test]
    fn parse_statement_block() {
        let token_series = [
            vec![
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Assign, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Assign, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Assign, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            Block(vec![]),
            Block(vec![test_node!(Statement::Assignment {
                identifier: test_node!(String::from("x")),
                value: test_node!(Expression::Literal(Literal::I64(5))),
                indices: vec![]
            })]),
            Block(vec![
                test_node!(Statement::Assignment {
                    identifier: test_node!(String::from("x")),
                    value: test_node!(Expression::Literal(Literal::I64(5))),
                    indices: vec![]
                }),
                test_node!(Statement::Assignment {
                    identifier: test_node!(String::from("x")),
                    value: test_node!(Expression::Literal(Literal::I64(5))),
                    indices: vec![]
                }),
            ]),
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_statement_block().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
    }

    #[test]
    fn parse_statement_fail() {
        let series = vec![
            // i64 a = 5
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("a"))),
            create_token(TokenCategory::Assign, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_statement().is_err());
    }

    #[test]
    fn parse_statement() {
        let token_series = [
            vec![
                // x = 5;
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Assign, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // print();
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // if (true) {}
                create_token(TokenCategory::If, TokenValue::Null),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::True, TokenValue::Null),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // for(;true;) {}
                create_token(TokenCategory::For, TokenValue::Null),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::True, TokenValue::Null),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // switch(x) {
                //      (true) -> {}
                // }
                create_token(TokenCategory::Switch, TokenValue::Null),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::True, TokenValue::Null),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::Arrow, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // return;
                create_token(TokenCategory::Return, TokenValue::Null),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // break;
                create_token(TokenCategory::Break, TokenValue::Null),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // i64 a = 5;
                create_token(TokenCategory::I64, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("a"))),
                create_token(TokenCategory::Assign, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            Statement::Assignment {
                identifier: test_node!(String::from("x")),
                value: test_node!(Expression::Literal(Literal::I64(5))),
                indices: vec![],
            },
            Statement::FunctionCall {
                identifier: test_node!(String::from("print")),
                arguments: vec![],
            },
            Statement::Conditional {
                condition: test_node!(Expression::Literal(Literal::True)),
                if_block: test_node!(Block(vec![])),
                else_block: None,
            },
            Statement::ForLoop {
                declaration: None,
                condition: test_node!(Expression::Literal(Literal::True)),
                assignment: None,
                block: test_node!(Block(vec![])),
            },
            Statement::Switch {
                expressions: vec![test_node!(SwitchExpression {
                    expression: test_node!(Expression::Variable(String::from("x"))),
                    alias: None,
                })],
                cases: vec![test_node!(SwitchCase {
                    condition: test_node!(Expression::Literal(Literal::True)),
                    block: test_node!(Block(vec![])),
                })],
            },
            Statement::Return(None),
            Statement::Break,
            Statement::Declaration {
                var_type: test_node!(Type::I64),
                identifier: test_node!(String::from("a")),
                value: Some(test_node!(Expression::Literal(Literal::I64(5)))),
            },
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_statement().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
    }

    #[test]
    fn parse_function_declaration_fail() {
        let series = vec![
            // fn add(): , {}
            create_token(TokenCategory::Fn, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::Comma, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_function_declaration().is_err());
    }

    #[test]
    fn parse_function_declaration() {
        let token_series = [
            vec![
                // fn add(): i64 {}
                create_token(TokenCategory::Fn, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::Colon, TokenValue::Null),
                create_token(TokenCategory::I64, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // fn add(): void {}
                create_token(TokenCategory::Fn, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("add"))),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::Colon, TokenValue::Null),
                create_token(TokenCategory::Void, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            FunctionDeclaration {
                identifier: test_node!(String::from("add")),
                parameters: vec![],
                return_type: test_node!(Type::I64),
                block: test_node!(Block(vec![])),
            },
            FunctionDeclaration {
                identifier: test_node!(String::from("add")),
                parameters: vec![],
                return_type: test_node!(Type::Void),
                block: test_node!(Block(vec![])),
            },
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_function_declaration().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
    }

    #[test]
    fn parse_parameters_fail() {
        let tokens = vec![
            // i64 x,
            create_token(TokenCategory::I64, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Comma, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_parameters().is_err());
    }

    #[test]
    fn parse_parameters() {
        let token_series = [
            vec![
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // i64 x
                create_token(TokenCategory::I64, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // i64 x, i64 y
                create_token(TokenCategory::I64, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Comma, TokenValue::Null),
                create_token(TokenCategory::I64, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("y"))),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            vec![],
            vec![test_node!(Parameter {
                passed_by: PassedBy::Value,
                parameter_type: test_node!(Type::I64),
                identifier: test_node!(String::from("x")),
            })],
            vec![
                test_node!(Parameter {
                    passed_by: PassedBy::Value,
                    parameter_type: test_node!(Type::I64),
                    identifier: test_node!(String::from("x")),
                }),
                test_node!(Parameter {
                    passed_by: PassedBy::Value,
                    parameter_type: test_node!(Type::I64),
                    identifier: test_node!(String::from("y")),
                }),
            ],
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let vector = parser.parse_parameters().unwrap();
            assert_eq!(vector, expected[idx]);
        }
    }

    #[test]
    fn parse_parameter() {
        let token_series = [
            vec![
                // &i64 x = 0
                create_token(TokenCategory::Reference, TokenValue::Null),
                create_token(TokenCategory::I64, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Assign, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(0)),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // i64 x
                create_token(TokenCategory::I64, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            Parameter {
                passed_by: PassedBy::Reference,
                parameter_type: test_node!(Type::I64),
                identifier: test_node!(String::from("x")),
            },
            Parameter {
                passed_by: PassedBy::Value,
                parameter_type: test_node!(Type::I64),
                identifier: test_node!(String::from("x")),
            },
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_parameter().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
    }

    #[test]
    fn parse_for_statement_fail() {
        let token_series = [
            vec![
                // for (
                create_token(TokenCategory::For, TokenValue::Null),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // for (;;) {}
                create_token(TokenCategory::For, TokenValue::Null),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                //  for (;x; {}
                create_token(TokenCategory::For, TokenValue::Null),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        for idx in 0..token_series.len() {
            let mock_lexer = LexerMock::new(token_series[idx].clone());
            let mut parser = Parser::new(mock_lexer);

            assert!(parser.parse_for_statement().is_err());
        }
    }

    #[test]
    fn parse_for_statement() {
        let token_series = [
            vec![
                // for (i64 x = 0; x < 5; x = x + 1) {}
                create_token(TokenCategory::For, TokenValue::Null),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::I64, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Assign, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(0)),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Less, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Assign, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Plus, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(1)),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // for (;x < 5;) {}
                create_token(TokenCategory::For, TokenValue::Null),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Less, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::Semicolon, TokenValue::Null),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            Statement::ForLoop {
                declaration: Some(Box::new(test_node!(Statement::Declaration {
                    var_type: test_node!(Type::I64),
                    identifier: test_node!(String::from("x")),
                    value: Some(test_node!(Expression::Literal(Literal::I64(0)))),
                }))),
                condition: test_node!(Expression::Less(
                    Box::new(test_node!(Expression::Variable(String::from("x")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
                )),
                assignment: Some(Box::new(test_node!(Statement::Assignment {
                    identifier: test_node!(String::from("x")),
                    value: test_node!(Expression::Addition(
                        Box::new(test_node!(Expression::Variable(String::from("x")))),
                        Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
                    )),
                    indices: vec![]
                }))),
                block: test_node!(Block(vec![])),
            },
            Statement::ForLoop {
                declaration: None,
                condition: test_node!(Expression::Less(
                    Box::new(test_node!(Expression::Variable(String::from("x")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
                )),
                assignment: None,
                block: test_node!(Block(vec![])),
            },
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_for_statement().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
    }

    #[test]
    fn parse_if_statement_fail() {
        let token_series = [
            vec![
                // if true) {}
                create_token(TokenCategory::If, TokenValue::Null),
                create_token(TokenCategory::True, TokenValue::Null),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // if (true {}
                create_token(TokenCategory::If, TokenValue::Null),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::True, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        for idx in 0..token_series.len() {
            let mock_lexer = LexerMock::new(token_series[idx].to_vec());
            let mut parser = Parser::new(mock_lexer);

            assert!(parser.parse_if_statement().is_err());
        }
    }

    #[test]
    fn parse_if_statement() {
        let token_series = [
            vec![
                // if (true) {}
                create_token(TokenCategory::If, TokenValue::Null),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::True, TokenValue::Null),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // if (true) {} else {}
                create_token(TokenCategory::If, TokenValue::Null),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::True, TokenValue::Null),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::Else, TokenValue::Null),
                create_token(TokenCategory::BraceOpen, TokenValue::Null),
                create_token(TokenCategory::BraceClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            Statement::Conditional {
                condition: test_node!(Expression::Literal(Literal::True)),
                if_block: test_node!(Block(vec![])),
                else_block: None,
            },
            Statement::Conditional {
                condition: test_node!(Expression::Literal(Literal::True)),
                if_block: test_node!(Block(vec![])),
                else_block: Some(test_node!(Block(vec![]))),
            },
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_if_statement().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
    }

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
                indices: vec![],
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
                var_type: test_node!(Type::I64),
                identifier: test_node!(String::from("a")),
                value: None,
            },
            Statement::Declaration {
                var_type: test_node!(Type::I64),
                identifier: test_node!(String::from("a")),
                value: Some(test_node!(Expression::Literal(Literal::I64(5)))),
            },
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_declaration().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
    }

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

    #[test]
    fn parse_break_statement_fail() {
        let series = vec![
            // break
            create_token(TokenCategory::Break, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_break_statement().is_err());
    }

    #[test]
    fn parse_break_statement() {
        let tokens = vec![
            // break;
            create_token(TokenCategory::Break, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_break_statement().unwrap().unwrap();
        assert_eq!(node.value, Statement::Break);
    }

    #[test]
    fn parse_arguments_comma_end() {
        let tokens = vec![
            // 1,
            create_token(TokenCategory::I64Value, TokenValue::I64(1)),
            create_token(TokenCategory::Comma, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_arguments().is_err());
    }

    #[test]
    fn parse_arguments() {
        let token_series = [
            vec![
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // 1
                create_token(TokenCategory::I64Value, TokenValue::I64(1)),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // 1, 2
                create_token(TokenCategory::Reference, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(1)),
                create_token(TokenCategory::Comma, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(2)),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            vec![],
            vec![test_node!(Argument {
                value: test_node!(Expression::Literal(Literal::I64(1))),
                passed_by: PassedBy::Value
            })],
            vec![
                test_node!(Argument {
                    value: test_node!(Expression::Literal(Literal::I64(1))),
                    passed_by: PassedBy::Reference
                }),
                test_node!(Argument {
                    value: test_node!(Expression::Literal(Literal::I64(2))),
                    passed_by: PassedBy::Value
                }),
            ],
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let vector = parser.parse_arguments().unwrap();
            assert_eq!(vector, expected[idx]);
        }
    }

    #[test]
    fn parse_argument() {
        let token_series = [
            vec![
                // 1
                create_token(TokenCategory::I64Value, TokenValue::I64(1)),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // &x
                create_token(TokenCategory::Reference, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            Argument {
                value: test_node!(Expression::Literal(Literal::I64(1))),
                passed_by: PassedBy::Value,
            },
            Argument {
                value: test_node!(Expression::Variable(String::from("x"))),
                passed_by: PassedBy::Reference,
            },
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_argument().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
    }

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

    #[test]
    fn parse_concatenation_term() {
        let tokens = vec![
            // a && b && c
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("a"))),
            create_token(TokenCategory::And, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("b"))),
            create_token(TokenCategory::And, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("c"))),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_concatenation_term().unwrap().unwrap();
        assert_eq!(
            node,
            test_node!(Expression::Concatenation(
                Box::new(test_node!(Expression::Concatenation(
                    Box::new(test_node!(Expression::Variable(String::from("a")))),
                    Box::new(test_node!(Expression::Variable(String::from("b")))),
                ))),
                Box::new(test_node!(Expression::Variable(String::from("c")))),
            ))
        );
    }

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

    #[test]
    fn parse_additive_term() {
        // 5 + 2.0 - x
        let tokens = vec![
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::Plus, TokenValue::Null),
            create_token(TokenCategory::F64Value, TokenValue::F64(2.0)),
            create_token(TokenCategory::Minus, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_additive_term().unwrap().unwrap();
        assert_eq!(
            node,
            test_node!(Expression::Subtraction(
                Box::new(test_node!(Expression::Addition(
                    Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
                    Box::new(test_node!(Expression::Literal(Literal::F64(2.0))))
                ))),
                Box::new(test_node!(Expression::Variable(String::from("x"))))
            ))
        )
    }

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
    fn parse_casted_term() {
        let token_series = [
            vec![
                // 5 as str
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::As, TokenValue::Null),
                create_token(TokenCategory::String, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // 5
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            Expression::Casting {
                value: Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
                to_type: test_node!(Type::Str),
            },
            Expression::Literal(Literal::I64(5)),
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_casted_term().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
    }

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

    #[test]
    fn parse_factor() {
        let token_series = [
            // (5 + 2)
            vec![
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::Plus, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(2)),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // 5
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // print
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            Expression::Addition(
                Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
                Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
            ),
            Expression::Literal(Literal::I64(5)),
            Expression::Variable(String::from("print")),
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_factor().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
    }

    #[test]
    fn parse_factor_nested_expression_unclosed() {
        let tokens = vec![
            // (5 + 2
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::Plus, TokenValue::Null),
            create_token(TokenCategory::I64Value, TokenValue::I64(2)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_factor().is_err());
    }

    #[test]
    fn parse_identifier_or_call_fail() {
        let token_series = [
            vec![
                // print(5,)
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::Comma, TokenValue::Null),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                create_token(
                    // print(
                    TokenCategory::Identifier,
                    TokenValue::String(String::from("print")),
                ),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        for idx in 0..token_series.len() {
            let mock_lexer = LexerMock::new(token_series[idx].clone());
            let mut parser = Parser::new(mock_lexer);

            assert!(parser.parse_identifier_or_call().is_err());
        }
    }

    #[test]
    fn parse_identifier_or_call() {
        let token_series = [
            vec![
                // print
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
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
                // print(5)
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // print(5, x)
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
                create_token(TokenCategory::ParenOpen, TokenValue::Null),
                create_token(TokenCategory::Reference, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(5)),
                create_token(TokenCategory::Comma, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::ParenClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            Expression::Variable(String::from("print")),
            Expression::FunctionCall {
                identifier: test_node!(String::from("print")),
                arguments: vec![],
            },
            Expression::FunctionCall {
                identifier: test_node!(String::from("print")),
                arguments: vec![Box::new(test_node!(Argument {
                    value: test_node!(Expression::Literal(Literal::I64(5))),
                    passed_by: PassedBy::Value,
                }))],
            },
            Expression::FunctionCall {
                identifier: test_node!(String::from("print")),
                arguments: vec![
                    Box::new(test_node!(Argument {
                        value: test_node!(Expression::Literal(Literal::I64(5))),
                        passed_by: PassedBy::Reference,
                    })),
                    Box::new(test_node!(Argument {
                        value: test_node!(Expression::Variable(String::from("x"))),
                        passed_by: PassedBy::Value,
                    })),
                ],
            },
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_identifier_or_call().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
    }

    #[test]
    fn parse_switch_statement() {
        let series = vec![
            // switch(x) {
            //      (true) -> {}
            // }
            create_token(TokenCategory::Switch, TokenValue::Null),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::True, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Arrow, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let expected = Statement::Switch {
            expressions: vec![test_node!(SwitchExpression {
                expression: test_node!(Expression::Variable(String::from("x"))),
                alias: None,
            })],
            cases: vec![test_node!(SwitchCase {
                condition: test_node!(Expression::Literal(Literal::True)),
                block: test_node!(Block(vec![])),
            })],
        };

        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_switch_statement().unwrap().unwrap();
        assert_eq!(node.value, expected);
    }

    #[test]
    fn parse_switch_expressions_fail() {
        let series = vec![
            // x: temp,
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
            create_token(TokenCategory::Colon, TokenValue::Null),
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("temp"))),
            create_token(TokenCategory::Comma, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_switch_expressions().is_err());
    }

    #[test]
    fn parse_switch_expressions() {
        let token_series = [
            vec![
                // x: temp, y
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Colon, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("temp"))),
                create_token(TokenCategory::Comma, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("y"))),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // x
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected_types = [
            vec![
                test_node!(SwitchExpression {
                    expression: test_node!(Expression::Variable(String::from("x"))),
                    alias: Some(test_node!(String::from("temp"))),
                }),
                test_node!(SwitchExpression {
                    expression: test_node!(Expression::Variable(String::from("y"))),
                    alias: None,
                }),
            ],
            vec![test_node!(SwitchExpression {
                expression: test_node!(Expression::Variable(String::from("x"))),
                alias: None,
            })],
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let vector = parser.parse_switch_expressions().unwrap();
            assert_eq!(vector, expected_types[idx]);
        }
    }

    #[test]
    fn parse_switch_expression() {
        let token_series = [
            vec![
                // x: temp
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::Colon, TokenValue::Null),
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("temp"))),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // x
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected_types = [
            SwitchExpression {
                expression: test_node!(Expression::Variable(String::from("x"))),
                alias: Some(test_node!(String::from("temp"))),
            },
            SwitchExpression {
                expression: test_node!(Expression::Variable(String::from("x"))),
                alias: None,
            },
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_switch_expression().unwrap().unwrap();
            assert_eq!(node.value, expected_types[idx]);
        }
    }

    #[test]
    fn parse_switch_case() {
        let series = vec![
            // (true) -> {}
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::True, TokenValue::Null),
            create_token(TokenCategory::ParenClose, TokenValue::Null),
            create_token(TokenCategory::Arrow, TokenValue::Null),
            create_token(TokenCategory::BraceOpen, TokenValue::Null),
            create_token(TokenCategory::BraceClose, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let expected = SwitchCase {
            condition: test_node!(Expression::Literal(Literal::True)),
            block: test_node!(Block(vec![])),
        };

        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_switch_case().unwrap().unwrap();
        assert_eq!(node.value, expected);
    }

    #[test]
    fn parse_type() {
        let token_series = [
            vec![
                create_token(TokenCategory::I64, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                create_token(TokenCategory::F64, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                create_token(TokenCategory::String, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                create_token(TokenCategory::Bool, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected_types = [Type::I64, Type::F64, Type::Str, Type::Bool];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_type().unwrap().unwrap();
            assert_eq!(node.value, expected_types[idx]);
        }
    }

    #[test]
    fn parse_type_fail() {
        let token_series = [
            vec![
                create_token(TokenCategory::Void, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                create_token(TokenCategory::Comma, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        for series in token_series {
            let mock_lexer = LexerMock::new(series);
            let mut parser = Parser::new(mock_lexer);

            assert!(parser.parse_type().is_ok());
            assert!(parser.parse_type().unwrap().is_none());
        }
    }

    #[test]
    fn parse_literals() {
        let tokens = vec![
            create_token(TokenCategory::True, TokenValue::Null),
            create_token(TokenCategory::False, TokenValue::Null),
            create_token(TokenCategory::StringValue, TokenValue::String(String::from("a"))),
            create_token(TokenCategory::I64Value, TokenValue::I64(5)),
            create_token(TokenCategory::F64Value, TokenValue::F64(5.0)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);

        let mut literal = parser.parse_literal().unwrap().unwrap();
        assert_eq!(literal.value, Literal::True);

        literal = parser.parse_literal().unwrap().unwrap();
        assert_eq!(literal.value, Literal::False);

        literal = parser.parse_literal().unwrap().unwrap();
        assert_eq!(literal.value, Literal::String(String::from("a")));

        literal = parser.parse_literal().unwrap().unwrap();
        assert_eq!(literal.value, Literal::I64(5));

        literal = parser.parse_literal().unwrap().unwrap();
        assert_eq!(literal.value, Literal::F64(5.0));
    }

    #[test]
    fn parse_identifier() {
        let tokens = vec![
            create_token(TokenCategory::Identifier, TokenValue::String(String::from("print"))),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_identifier().unwrap().unwrap();
        assert_eq!(node.value, String::from("print"));
    }

    #[test]
    fn parse_identifier_bad_value_type() {
        let tokens = vec![
            // 5 is not string
            create_token(TokenCategory::Identifier, TokenValue::I64(5)),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);

        let result = parser.parse_identifier();
        assert!(result.is_err());
    }

    #[test]
    fn consume_must_be() {
        let tokens = vec![
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);
        assert_eq!(parser.current_token().clone().category, TokenCategory::ParenOpen);
        let _ = parser.consume_must_be(TokenCategory::ParenOpen).unwrap();

        assert_eq!(parser.current_token().clone().category, TokenCategory::ETX);
    }

    #[test]
    fn consume_must_be_fail() {
        let tokens = vec![
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);
        assert_eq!(parser.current_token().clone().category, TokenCategory::ParenOpen);
        let result = parser.consume_must_be(TokenCategory::Semicolon);

        assert!(result.is_err());
        assert_eq!(parser.current_token().clone().category, TokenCategory::ParenOpen);
    }

    #[test]
    fn consume_if_matches() {
        let tokens = vec![
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);
        assert_eq!(parser.current_token().clone().category, TokenCategory::ParenOpen);
        let _ = parser.consume_if_matches(TokenCategory::ParenOpen).unwrap();

        assert_eq!(parser.current_token().clone().category, TokenCategory::ETX);
    }

    #[test]
    fn consume_if_matches_fail() {
        let tokens = vec![
            create_token(TokenCategory::ParenOpen, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);
        assert_eq!(parser.current_token().clone().category, TokenCategory::ParenOpen);
        let result = parser.consume_if_matches(TokenCategory::Semicolon);

        assert!(result.unwrap().is_none());
        assert_eq!(parser.current_token().clone().category, TokenCategory::ParenOpen);
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
                indices: vec![],
            },
            Statement::Assignment {
                identifier: test_node!(String::from("x")),
                value: test_node!(Expression::Subtraction(
                    Box::new(test_node!(Expression::Variable(String::from("x")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
                )),
                indices: vec![],
            },
            Statement::Assignment {
                identifier: test_node!(String::from("x")),
                value: test_node!(Expression::Multiplication(
                    Box::new(test_node!(Expression::Variable(String::from("x")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
                )),
                indices: vec![],
            },
            Statement::Assignment {
                identifier: test_node!(String::from("x")),
                value: test_node!(Expression::Division(
                    Box::new(test_node!(Expression::Variable(String::from("x")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
                )),
                indices: vec![],
            },
            Statement::Assignment {
                identifier: test_node!(String::from("x")),
                value: test_node!(Expression::Modulo(
                    Box::new(test_node!(Expression::Variable(String::from("x")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
                )),
                indices: vec![],
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
            indices: vec![test_node!(Expression::Literal(Literal::I64(0)))],
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
            indices: vec![test_node!(Expression::Literal(Literal::I64(0)))],
            value: test_node!(Expression::Literal(Literal::I64(5))),
        };

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_assign_or_call().unwrap().unwrap();
        assert_eq!(node.value, expected);
    }

    #[test]
    fn parse_continue_statement_fail() {
        let series = vec![
            // continue
            create_token(TokenCategory::Continue, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(series);
        let mut parser = Parser::new(mock_lexer);

        assert!(parser.parse_continue_statement().is_err());
    }

    #[test]
    fn parse_continue_statement() {
        let tokens = vec![
            // continue;
            create_token(TokenCategory::Continue, TokenValue::Null),
            create_token(TokenCategory::Semicolon, TokenValue::Null),
            create_token(TokenCategory::ETX, TokenValue::Null),
        ];

        let mock_lexer = LexerMock::new(tokens);
        let mut parser = Parser::new(mock_lexer);

        let node = parser.parse_continue_statement().unwrap().unwrap();
        assert_eq!(node.value, Statement::Continue);
    }

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

    #[test]
    fn parse_identifier_or_call_with_index() {
        let token_series = [
            vec![
                // x[0]
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::BracketOpen, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(0)),
                create_token(TokenCategory::BracketClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // x[0][1]
                create_token(TokenCategory::Identifier, TokenValue::String(String::from("x"))),
                create_token(TokenCategory::BracketOpen, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(0)),
                create_token(TokenCategory::BracketClose, TokenValue::Null),
                create_token(TokenCategory::BracketOpen, TokenValue::Null),
                create_token(TokenCategory::I64Value, TokenValue::I64(1)),
                create_token(TokenCategory::BracketClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            Expression::Index {
                collection: Box::new(test_node!(Expression::Variable(String::from("x")))),
                index: Box::new(test_node!(Expression::Literal(Literal::I64(0)))),
            },
            Expression::Index {
                collection: Box::new(test_node!(Expression::Index {
                    collection: Box::new(test_node!(Expression::Variable(String::from("x")))),
                    index: Box::new(test_node!(Expression::Literal(Literal::I64(0)))),
                })),
                index: Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
            },
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_identifier_or_call().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
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

    #[test]
    fn parse_type_vector() {
        let token_series = [
            vec![
                // i64[]
                create_token(TokenCategory::I64, TokenValue::Null),
                create_token(TokenCategory::BracketOpen, TokenValue::Null),
                create_token(TokenCategory::BracketClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
            vec![
                // i64[][]
                create_token(TokenCategory::I64, TokenValue::Null),
                create_token(TokenCategory::BracketOpen, TokenValue::Null),
                create_token(TokenCategory::BracketClose, TokenValue::Null),
                create_token(TokenCategory::BracketOpen, TokenValue::Null),
                create_token(TokenCategory::BracketClose, TokenValue::Null),
                create_token(TokenCategory::ETX, TokenValue::Null),
            ],
        ];

        let expected = [
            Type::Vector(Box::new(Type::I64)),
            Type::Vector(Box::new(Type::Vector(Box::new(Type::I64)))),
        ];

        for (idx, series) in token_series.iter().enumerate() {
            let mock_lexer = LexerMock::new(series.to_vec());
            let mut parser = Parser::new(mock_lexer);

            let node = parser.parse_type().unwrap().unwrap();
            assert_eq!(node.value, expected[idx]);
        }
    }

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
                indices: vec![],
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
        assert_eq!(err.message(), format!("Redeclaration of function 'add'.\nAt: {:?}.", default_position()));
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
        assert_eq!(
            err.message(),
            format!("Redeclaration of function 'print'.\nAt: {:?}.", default_position())
        );
    }
}
