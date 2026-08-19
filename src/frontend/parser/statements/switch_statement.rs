use crate::{
    common::{errors::IError, span::Span},
    frontend::{
        ast::{Node, Statement, SwitchCase, SwitchExpression},
        lexer::lexer::ILexer,
        parser::{
            core::{try_consume, try_consume_token},
            Parser,
        },
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn parse_switch_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        // switch_statement = "switch", "(", switch_expressions, ")", "{", {switch_case}, "}";

        let switch_token = try_consume_token!(self, TokenCategory::Switch);

        let _ = self.consume_must_be(TokenCategory::ParenOpen)?;

        let switch_expressions = self.parse_switch_expressions()?;

        let _ = self.consume_must_be(TokenCategory::ParenClose)?;
        let _ = self.consume_must_be(TokenCategory::BraceOpen)?;

        let mut switch_cases: Vec<Node<SwitchCase>> = vec![];

        while self.current_token().category != TokenCategory::BraceClose {
            let switch_case = self
                .parse_switch_case()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create switch case while parsing switch statement.")))?;

            switch_cases.push(switch_case);
        }

        let closing_brace = self.consume_must_be(TokenCategory::BraceClose)?;

        let node = Node {
            value: Statement::Switch {
                expressions: switch_expressions,
                cases: switch_cases,
            },
            span: Span::new(switch_token.span.start(), closing_brace.span.end()),
        };

        Ok(Some(node))
    }

    pub(in crate::frontend::parser) fn parse_switch_expressions(&mut self) -> Result<Vec<Node<SwitchExpression>>, Box<dyn IError>> {
        // switch_expressions = switch_expression, { ",", switch_expression };

        let mut switch_expressions: Vec<Node<SwitchExpression>> = vec![];

        let mut expression = match self.parse_switch_expression()? {
            Some(t) => t,
            None => return Ok(vec![]),
        };

        switch_expressions.push(expression);

        while self.consume_if_matches(TokenCategory::Comma)?.is_some() {
            expression = self
                .parse_switch_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create switch expression while parsing switch expressions.")))?;

            switch_expressions.push(expression);
        }

        Ok(switch_expressions)
    }

    pub(in crate::frontend::parser) fn parse_switch_expression(&mut self) -> Result<Option<Node<SwitchExpression>>, Box<dyn IError>> {
        // switch_expression = expression, [ ":", identifier ];

        let expression = try_consume!(self, parse_expression);

        let start = expression.span.start();

        let alias = if self.consume_if_matches(TokenCategory::Colon)?.is_some() {
            self.parse_identifier()?
        } else {
            None
        };

        let end = alias.as_ref().map(|alias| alias.span.end()).unwrap_or_else(|| expression.span.end());

        let node = Node {
            value: SwitchExpression { expression, alias },
            span: Span::new(start, end),
        };

        Ok(Some(node))
    }

    pub(in crate::frontend::parser) fn parse_switch_case(&mut self) -> Result<Option<Node<SwitchCase>>, Box<dyn IError>> {
        // switch_case = "(", expression, ")", "->", statement_block;

        let paren_open_token = try_consume_token!(self, TokenCategory::ParenOpen);

        let condition = self
            .parse_expression()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create expression while parsing switch case.")))?;

        let _ = self.consume_must_be(TokenCategory::ParenClose)?;
        let _ = self.consume_must_be(TokenCategory::Arrow)?;

        let block = self
            .parse_statement_block()?
            .ok_or_else(|| self.create_parser_error(String::from("Couldn't create statement block while parsing switch case.")))?;

        let node = Node {
            value: SwitchCase {
                condition,
                block: block.clone(),
            },
            span: Span::new(paren_open_token.span.start(), block.span.end()),
        };

        Ok(Some(node))
    }
}
