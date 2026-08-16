use crate::{
    common::errors::IError,
    frontend::{ast::Program, lexer::lexer::ILexer},
};

mod core;
mod expressions;
mod functions;
mod misc;
mod program;
mod statements;
#[cfg(test)]
mod tests;

pub struct Parser<L: ILexer> {
    pub lexer: L,
}

pub trait IParser<L: ILexer> {
    fn new(lexer: L) -> Parser<L>;
    fn parse(&mut self) -> Result<Program, Box<dyn IError>>;
}

impl<L: ILexer> IParser<L> for Parser<L> {
    fn new(lexer: L) -> Parser<L> {
        Parser { lexer }
    }

    fn parse(&mut self) -> Result<Program, Box<dyn IError>> {
        self.parse_program()
    }
}
