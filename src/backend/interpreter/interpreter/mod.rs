use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    backend::interpreter::{stack::stack::Stack, Value},
    common::{
        errors::{ErrorSeverity, IError, InterpreterError},
        span::Span,
        visitor::Visitor,
    },
    frontend::ast::Program,
};

pub mod core;
pub mod expressions;
pub mod functions;
pub mod statements;
pub mod visitor;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub(in crate::backend::interpreter::interpreter) enum AbortState {
    Break,
    Continue,
    Return,
}

pub struct Interpreter<'a> {
    pub(in crate::backend::interpreter::interpreter) program: &'a Program,
    pub(in crate::backend::interpreter::interpreter) stack: Stack<'a>,
    pub(in crate::backend::interpreter::interpreter) last_result: Option<Value>,
    pub(in crate::backend::interpreter::interpreter) abort_state: Option<AbortState>,
    pub(in crate::backend::interpreter::interpreter) span: Span,
    pub(in crate::backend::interpreter::interpreter) last_arguments: Vec<Rc<RefCell<Value>>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a Program) -> Self {
        Interpreter {
            program,
            stack: Stack::new(),
            abort_state: None,
            last_result: None,
            span: Span::default(),
            last_arguments: vec![],
        }
    }

    pub fn interpret(&mut self) -> Result<(), Box<dyn IError>> {
        if let Some((name, function)) = self.program.extern_functions.iter().next() {
            return Err(Box::new(InterpreterError::new(
                ErrorSeverity::HIGH,
                format!("Extern function `{}` cannot be used in interpretation mode.", name),
                function.span,
            )));
        }

        self.visit_program(self.program)
    }
}
