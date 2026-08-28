use crate::{
    common::{errors::IError, span::Span},
    frontend::{
        ast::{Accessor, Expression, Node, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_assign_or_call(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // assign_or_call = identifier, ( { access_tail }, ("=" | "+=" | "-=" | "*=" | "/=" | "%="), expression | "(", arguments, ")"), ";";
        let node = self.parse_assign_or_call_without_semicolon()?;

        match node {
            None => Ok(None),
            Some(mut n) => {
                let semicolon_token = self.consume_must_be(TokenCategory::Semicolon)?;
                n.span = Span::new(n.span.start(), semicolon_token.span.end());
                Ok(Some(n))
            }
        }
    }

    pub(in crate::frontend::parser) fn parse_assign_or_call_without_semicolon(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // assign_or_call = identifier, ( { access_tail }, ("=" | "+=" | "-=" | "*=" | "/=" | "%="), expression | "(", arguments, ")");

        let identifier = try_consume!(self, parse_identifier);
        let identifier_start = identifier.span.start();

        let mut accessors = Vec::new();

        loop {
            match self.current_token().category {
                TokenCategory::BracketOpen => {
                    let bracket_open = self.consume_must_be(TokenCategory::BracketOpen)?;

                    let index_expr = self
                        .parse_expression()?
                        .ok_or_else(|| self.create_parser_error(String::from("Expected an expression inside '[]' index.")))?;

                    let bracket_close = self.consume_must_be(TokenCategory::BracketClose)?;

                    accessors.push(Node {
                        span: Span::new(bracket_open.span.start(), bracket_close.span.end()),
                        value: Accessor::Index(index_expr),
                    });
                }

                TokenCategory::Dot => {
                    let dot = self.consume_must_be(TokenCategory::Dot)?;

                    let field = self
                        .parse_identifier()?
                        .ok_or_else(|| self.create_parser_error(String::from("Expected an identifier after '.'.")))?;

                    accessors.push(Node {
                        span: Span::new(dot.span.start(), field.span.end()),
                        value: Accessor::Field(field),
                    });
                }

                _ => break,
            }
        }

        let build_access_expression = || {
            let mut result = Node {
                value: Expression::Variable(identifier.value.clone()),
                span: identifier.span,
            };

            for accessor in &accessors {
                let accessor_span = Span::new(result.span.start(), accessor.span.end());

                result = match &accessor.value {
                    Accessor::Index(index) => Node {
                        value: Expression::Index {
                            collection: Box::new(result),
                            index: Box::new(index.clone()),
                        },
                        span: accessor_span,
                    },

                    Accessor::Field(field) => Node {
                        value: Expression::FieldAccess {
                            instance: Box::new(result),
                            field: field.clone(),
                        },
                        span: accessor_span,
                    },
                };
            }

            result
        };

        if self.consume_if_matches(TokenCategory::Assign)?.is_some() {
            let expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing assignment.")))?;

            let span = Span::new(identifier_start, expr.span.end());

            let node = Node {
                value: Statement::Assignment {
                    identifier,
                    value: expr,
                    accessors,
                },
                span,
            };

            return Ok(Some(node));
        }

        if self.consume_if_matches(TokenCategory::PlusEquals)?.is_some() {
            let expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing assignment.")))?;

            let left = build_access_expression();

            let value = Node {
                span: Span::new(left.span.start(), expr.span.end()),
                value: Expression::Addition(Box::new(left), Box::new(expr)),
            };

            let span = Span::new(identifier_start, value.span.end());

            return Ok(Some(Node {
                value: Statement::Assignment {
                    identifier,
                    accessors,
                    value,
                },
                span,
            }));
        }

        if self.consume_if_matches(TokenCategory::MinusEquals)?.is_some() {
            let expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing assignment.")))?;

            let left = build_access_expression();

            let value = Node {
                span: Span::new(left.span.start(), expr.span.end()),
                value: Expression::Subtraction(Box::new(left), Box::new(expr)),
            };

            let span = Span::new(identifier_start, value.span.end());

            return Ok(Some(Node {
                value: Statement::Assignment {
                    identifier,
                    accessors,
                    value,
                },
                span,
            }));
        }

        if self.consume_if_matches(TokenCategory::TimesEquals)?.is_some() {
            let expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing assignment.")))?;

            let left = build_access_expression();

            let value = Node {
                span: Span::new(left.span.start(), expr.span.end()),
                value: Expression::Multiplication(Box::new(left), Box::new(expr)),
            };

            let span = Span::new(identifier_start, value.span.end());

            return Ok(Some(Node {
                value: Statement::Assignment {
                    identifier,
                    accessors,
                    value,
                },
                span,
            }));
        }

        if self.consume_if_matches(TokenCategory::DivideEquals)?.is_some() {
            let expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing assignment.")))?;

            let left = build_access_expression();

            let value = Node {
                span: Span::new(left.span.start(), expr.span.end()),
                value: Expression::Division(Box::new(left), Box::new(expr)),
            };

            let span = Span::new(identifier_start, value.span.end());

            return Ok(Some(Node {
                value: Statement::Assignment {
                    identifier,
                    accessors,
                    value,
                },
                span,
            }));
        }

        if self.consume_if_matches(TokenCategory::ModuloEquals)?.is_some() {
            let expr = self
                .parse_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing assignment.")))?;

            let left = build_access_expression();

            let value = Node {
                span: Span::new(left.span.start(), expr.span.end()),
                value: Expression::Modulo(Box::new(left), Box::new(expr)),
            };

            let span = Span::new(identifier_start, value.span.end());

            return Ok(Some(Node {
                value: Statement::Assignment {
                    identifier,
                    accessors,
                    value,
                },
                span,
            }));
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
