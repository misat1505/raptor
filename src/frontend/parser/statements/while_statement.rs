use crate::{
    common::{
        errors::{ErrorSeverity, IError, ParserError},
        position::Position,
        span::Span,
    },
    frontend::{
        ast::{Block, Expression, Literal, Node, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume_token, Parser},
        tokens::TokenCategory,
    },
};

fn synthetic_break_block(span: Span) -> Node<Block> {
    Node {
        value: Block(vec![Node {
            value: Statement::Break,
            span,
        }]),
        span,
    }
}

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_while_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // while_statement = "while", "(", expression, (
        //      ( ")", statement_block ) |
        //      ( [ "not" ], "matches", match_arm_without_block, ")", statement_block ) );

        let while_token = try_consume_token!(self, TokenCategory::While);

        let _ = self.consume_must_be(TokenCategory::ParenOpen)?;

        let condition = self
            .parse_expression()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing while statement.")))?;

        if self.consume_if_matches(TokenCategory::ParenClose)?.is_none() {
            return self.finish_while_matches_statement(condition, while_token.span.start());
        }

        let block = self
            .parse_statement_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement block while parsing while statement.")))?;

        let block_end = block.span.end();

        let node = Node {
            value: Statement::WhileLoop { condition, block },
            span: Span::new(while_token.span.start(), block_end),
        };

        Ok(Some(node))
    }

    fn finish_while_matches_statement(
        &mut self,
        condition: Node<Expression>,
        start_pos: Position,
    ) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        let is_negating = self.consume_if_matches(TokenCategory::Not)?.is_some();
        let _ = self.consume_must_be(TokenCategory::Matches)?;

        let mut arm = self
            .parse_match_arm_without_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create match arm while parsing while-matches statement.")))?;

        let _ = self.consume_must_be(TokenCategory::ParenClose)?;

        let body = self
            .parse_statement_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement block while parsing while-matches statement.")))?;

        if is_negating {
            if let Some(var_val) = &arm.value.variant_value {
                return Err(Box::new(ParserError::at(
                    ErrorSeverity::HIGH,
                    String::from("Cannot take the inner value of enum variant in while-not-matches statement."),
                    var_val.span,
                )));
            }
        }

        let match_span = Span::new(condition.span.start(), body.span.end());
        let loop_span = Span::new(start_pos, body.span.end());
        let break_block = synthetic_break_block(condition.span);

        let true_condition = Node {
            value: Expression::Literal(Literal::True),
            span: condition.span,
        };

        let (arm_block, rest_block) = if is_negating { (break_block, body) } else { (body, break_block) };
        arm.value.block = arm_block;

        let match_stmt = Node {
            value: Statement::Match {
                expression: condition,
                match_arms: vec![arm],
                rest_arm: Some(rest_block),
            },
            span: match_span,
        };

        let loop_block = Node {
            value: Block(vec![match_stmt]),
            span: match_span,
        };

        Ok(Some(Node {
            value: Statement::WhileLoop {
                condition: true_condition,
                block: loop_block,
            },
            span: loop_span,
        }))
    }
}
