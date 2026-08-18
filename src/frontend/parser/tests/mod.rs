mod core;
mod parser;
mod program;

use crate::{
    common::{
        errors::{ErrorSeverity, IError, LexerError},
        position::Position,
        span::Span,
    },
    frontend::{
        lexer::lexer::ILexer,
        tokens::{Token, TokenCategory, TokenValue},
    },
};

pub struct LexerMock {
    current_token: Option<Token>,
    pub tokens: Vec<Token>,
}

impl LexerMock {
    pub fn new(mut tokens: Vec<Token>) -> LexerMock {
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
        if self.tokens.is_empty() {
            return Err(Box::new(LexerError::new(ErrorSeverity::HIGH, String::new(), Span::default())));
        }
        let next_token = self.tokens.remove(0);
        self.current_token = Some(next_token.clone());
        Ok(next_token)
    }
}

pub fn default_position() -> Position {
    Position {
        filename: None,
        line: 0,
        column: 0,
        offset: 0,
    }
}

pub fn create_token(category: TokenCategory, value: TokenValue) -> Token {
    Token {
        category,
        value,
        span: Span::default(),
    }
}

macro_rules! test_node {
    ($value:expr) => {
        crate::frontend::ast::Node {
            value: $value,
            span: crate::common::span::Span::default(),
        }
    };
}
pub(crate) use test_node;
