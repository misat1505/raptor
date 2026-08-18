use crate::{
    common::{errors::IError, span::Span},
    frontend::{
        ast::{Expression, Node, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_assign_or_call(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // assign_or_call = identifier, ( { "[", expression, "]" }, "=", expression | "(", arguments, ")"), ";";
        let node = self.parse_assign_or_call_without_semicolon()?;

        match node {
            None => Ok(None),
            Some(mut n) => {
                self.consume_must_be(TokenCategory::Semicolon)?;
                n.span = Span::new(n.span.start(), self.current_token().span.end());
                Ok(Some(n))
            }
        }
    }

    pub(in crate::frontend::parser) fn parse_assign_or_call_without_semicolon(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // assign_or_call = identifier, ( { "[", expression, "]" }, ("=" | "+=" | "-=" | "*=" | "/=" | "%="), expression | "(", arguments, ")");

        let identifier = try_consume!(self, parse_identifier);
        let identifier_start = identifier.span.start();

        let mut indices: Vec<Node<Expression>> = vec![];

        while self.consume_if_matches(TokenCategory::BracketOpen)?.is_some() {
            let index_expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression inside '[]' index.")))?;

            self.consume_must_be(TokenCategory::BracketClose)?;
            indices.push(index_expr);
        }

        if self.consume_if_matches(TokenCategory::Assign)?.is_some() {
            let expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing assignment.")))?;

            let span = Span::new(identifier_start, expr.span.end());

            let node = Node {
                value: Statement::Assignment {
                    identifier,
                    value: expr,
                    indices,
                },
                span,
            };

            return Ok(Some(node));
        }

        if self.consume_if_matches(TokenCategory::PlusEquals)?.is_some() {
            let expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing assignment.")))?;

            let mut result = Node {
                value: Expression::Variable(identifier.clone().value),
                span: identifier.span,
            };

            for index in &indices {
                let span = Span::new(result.span.start(), index.span.end());

                result = Node {
                    value: Expression::Index {
                        collection: Box::new(result),
                        index: Box::new(index.clone()),
                    },
                    span,
                };
            }

            let value_span = Span::new(result.span.start(), expr.span.end());

            let value = Node {
                value: Expression::Addition(Box::new(result), Box::new(expr)),
                span: value_span,
            };

            let node_span = Span::new(identifier_start, value.span.end());

            let node = Node {
                value: Statement::Assignment { identifier, indices, value },
                span: node_span,
            };

            return Ok(Some(node));
        }

        if self.consume_if_matches(TokenCategory::MinusEquals)?.is_some() {
            let expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing assignment.")))?;

            let mut result = Node {
                value: Expression::Variable(identifier.clone().value),
                span: identifier.span,
            };

            for index in &indices {
                let span = Span::new(result.span.start(), index.span.end());

                result = Node {
                    value: Expression::Index {
                        collection: Box::new(result),
                        index: Box::new(index.clone()),
                    },
                    span,
                };
            }

            let value_span = Span::new(result.span.start(), expr.span.end());

            let value = Node {
                value: Expression::Subtraction(Box::new(result), Box::new(expr)),
                span: value_span,
            };

            let node_span = Span::new(identifier_start, value.span.end());

            let node = Node {
                value: Statement::Assignment { identifier, indices, value },
                span: node_span,
            };

            return Ok(Some(node));
        }

        if self.consume_if_matches(TokenCategory::TimesEquals)?.is_some() {
            let expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing assignment.")))?;

            let mut result = Node {
                value: Expression::Variable(identifier.clone().value),
                span: identifier.span,
            };

            for index in &indices {
                let span = Span::new(result.span.start(), index.span.end());

                result = Node {
                    value: Expression::Index {
                        collection: Box::new(result),
                        index: Box::new(index.clone()),
                    },
                    span,
                };
            }

            let value_span = Span::new(result.span.start(), expr.span.end());

            let value = Node {
                value: Expression::Multiplication(Box::new(result), Box::new(expr)),
                span: value_span,
            };

            let node_span = Span::new(identifier_start, value.span.end());

            let node = Node {
                value: Statement::Assignment { identifier, indices, value },
                span: node_span,
            };

            return Ok(Some(node));
        }

        if self.consume_if_matches(TokenCategory::DivideEquals)?.is_some() {
            let expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing assignment.")))?;

            let mut result = Node {
                value: Expression::Variable(identifier.clone().value),
                span: identifier.span,
            };

            for index in &indices {
                let span = Span::new(result.span.start(), index.span.end());

                result = Node {
                    value: Expression::Index {
                        collection: Box::new(result),
                        index: Box::new(index.clone()),
                    },
                    span,
                };
            }

            let value_span = Span::new(result.span.start(), expr.span.end());

            let value = Node {
                value: Expression::Division(Box::new(result), Box::new(expr)),
                span: value_span,
            };

            let node_span = Span::new(identifier_start, value.span.end());

            let node = Node {
                value: Statement::Assignment { identifier, indices, value },
                span: node_span,
            };

            return Ok(Some(node));
        }

        if self.consume_if_matches(TokenCategory::ModuloEquals)?.is_some() {
            let expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing assignment.")))?;

            let mut result = Node {
                value: Expression::Variable(identifier.clone().value),
                span: identifier.span,
            };

            for index in &indices {
                let span = Span::new(result.span.start(), index.span.end());

                result = Node {
                    value: Expression::Index {
                        collection: Box::new(result),
                        index: Box::new(index.clone()),
                    },
                    span,
                };
            }

            let value_span = Span::new(result.span.start(), expr.span.end());

            let value = Node {
                value: Expression::Modulo(Box::new(result), Box::new(expr)),
                span: value_span,
            };

            let node_span = Span::new(identifier_start, value.span.end());

            let node = Node {
                value: Statement::Assignment { identifier, indices, value },
                span: node_span,
            };

            return Ok(Some(node));
        }

        if self.consume_if_matches(TokenCategory::ParenOpen)?.is_some() {
            let arguments = self.parse_arguments()?.into_iter().map(Box::new).collect();

            let close_paren = self.consume_must_be(TokenCategory::ParenClose)?;

            let span = Span::new(identifier_start, close_paren.span.end());

            let node = Node {
                value: Statement::FunctionCall { identifier, arguments },
                span,
            };

            return Ok(Some(node));
        }

        Err(self.create_parser_error(String::from("Couldn't create assignment or call.")))
    }
}
