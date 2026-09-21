use crate::{
    common::{
        errors::{ErrorSeverity, IError, ParserError},
        position::Position,
        span::Span,
    },
    frontend::{
        ast::{Block, Expression, Node, Statement},
        lexer::lexer::ILexer,
        parser::{core::try_consume_token, Parser},
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_if_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // if_statement = "if", "(", expression,
        //      ( ( ")", statement_block ) |
        //      ( [ "not" ], "matches", match_arm_without_block, ")", statement_block ) ),
        // [ "else", statement_block ];

        let if_token = try_consume_token!(self, TokenCategory::If);

        let _ = self.consume_must_be(TokenCategory::ParenOpen)?;

        let condition = self
            .parse_expression()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing if statement.")))?;

        if self.consume_if_matches(TokenCategory::ParenClose)?.is_none() {
            return self.finish_if_matches_statement(&condition, if_token.span.start());
        }

        let true_block = self
            .parse_statement_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement block while parsing if statement.")))?;

        let false_block = match self.consume_if_matches(TokenCategory::Else)? {
            Some(_) => self.parse_statement_block()?,
            None => None,
        };

        let end = false_block
            .as_ref()
            .map(|block| block.span.end())
            .unwrap_or_else(|| true_block.span.end());

        let node = Node {
            value: Statement::Conditional {
                condition,
                if_block: true_block,
                else_block: false_block,
            },
            span: Span::new(if_token.span.start(), end),
        };

        Ok(Some(node))
    }

    fn finish_if_matches_statement(&mut self, condition: &Node<Expression>, start_pos: Position) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        let is_negating = self.consume_if_matches(TokenCategory::Not)?.is_some();
        let _ = self.consume_must_be(TokenCategory::Matches)?;

        let mut arm = self
            .parse_match_arm_without_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create match arm while parsing if-matches statement.")))?;

        let _ = self.consume_must_be(TokenCategory::ParenClose)?;

        let match_block = self
            .parse_statement_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement block while parsing if-matches statement.")))?;

        let rest_block = match self.consume_if_matches(TokenCategory::Else)? {
            Some(_) => self
                .parse_statement_block()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement block while parsing if-matches else statement.")))?,
            None => Node {
                value: Block(vec![]),
                span: Span::new(match_block.span.end(), match_block.span.end()),
            },
        };

        let stmt = if is_negating {
            arm.value.block = rest_block.clone();

            if let Some(var_val) = arm.value.variant_value {
                return Err(Box::new(ParserError::at(
                    ErrorSeverity::HIGH,
                    String::from("Cannot take the inner value of enum variant in if-not-matches statement."),
                    var_val.span,
                )));
            }

            Statement::Match {
                expression: condition.clone(),
                match_arms: vec![arm],
                rest_arm: Some(match_block.clone()),
            }
        } else {
            arm.value.block = match_block.clone();

            Statement::Match {
                expression: condition.clone(),
                match_arms: vec![arm],
                rest_arm: Some(rest_block.clone()),
            }
        };

        Ok(Some(Node {
            value: stmt,
            span: Span::new(start_pos, rest_block.span.end()),
        }))
    }
}
