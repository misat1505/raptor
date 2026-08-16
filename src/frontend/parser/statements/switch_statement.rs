use crate::{
    common::errors::IError,
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
    pub fn parse_switch_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
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
        let _ = self.consume_must_be(TokenCategory::BraceClose)?;

        let node = Node {
            value: Statement::Switch {
                expressions: switch_expressions,
                cases: switch_cases,
            },
            position: switch_token.position,
        };
        Ok(Some(node))
    }

    pub fn parse_switch_expressions(&mut self) -> Result<Vec<Node<SwitchExpression>>, Box<dyn IError>> {
        // switch_expressions = switch_expression, { ",", switch_expression };
        let mut switch_expressions: Vec<Node<SwitchExpression>> = vec![];
        let mut expression = match self.parse_switch_expression()? {
            Some(t) => t,
            None => return Ok(vec![]),
        };

        switch_expressions.push(expression);
        while let Some(_) = self.consume_if_matches(TokenCategory::Comma)? {
            expression = self
                .parse_switch_expression()?
                .ok_or_else(|| self.create_parser_error(String::from("Couldn't create switch expression while parsing switch expressions.")))?;

            switch_expressions.push(expression);
        }
        Ok(switch_expressions)
    }

    pub fn parse_switch_expression(&mut self) -> Result<Option<Node<SwitchExpression>>, Box<dyn IError>> {
        // switch_expression = expression, [ ":", identifier ];
        let expression = try_consume!(self, parse_expression);

        let position = expression.position;
        let mut alias = None;
        if let Some(_) = self.consume_if_matches(TokenCategory::Colon)? {
            alias = self.parse_identifier()?;
        };
        let node = Node {
            value: SwitchExpression { expression, alias },
            position,
        };
        Ok(Some(node))
    }

    pub fn parse_switch_case(&mut self) -> Result<Option<Node<SwitchCase>>, Box<dyn IError>> {
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
            value: SwitchCase { condition, block },
            position: paren_open_token.position,
        };
        Ok(Some(node))
    }
}
