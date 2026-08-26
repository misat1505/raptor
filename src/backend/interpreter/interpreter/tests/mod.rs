mod control_flow;
mod expressions;
mod functions;
mod statements;

use std::collections::HashMap;

use crate::{backend::interpreter::interpreter::Interpreter, frontend::ast::Program};

pub(super) fn setup_program() -> Program {
    Program {
        statements: vec![],
        functions: HashMap::new(),
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    }
}

pub(super) fn create_interpreter<'a>(program: &'a Program) -> Interpreter<'a> {
    Interpreter::new(program)
}

macro_rules! test_node {
    ($value:expr) => {
        crate::frontend::ast::Node {
            value: $value,
            span: crate::common::span::Span::default(),
        }
    };
}

pub(super) use test_node;
