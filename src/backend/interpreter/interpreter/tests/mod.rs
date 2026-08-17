mod control_flow;
mod expressions;
mod functions;
mod statements;

use std::collections::HashMap;

use crate::{backend::interpreter::interpreter::Interpreter, common::position::Position, frontend::ast::Program};

pub(super) fn default_position() -> Position {
    Position {
        filename: None,
        line: 0,
        column: 0,
        offset: 0,
    }
}

pub(super) fn setup_program() -> Program {
    Program {
        statements: vec![],
        functions: HashMap::new(),
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
    }
}

pub(super) fn create_interpreter<'a>(program: &'a Program) -> Interpreter<'a> {
    Interpreter::new(program)
}

macro_rules! test_node {
    ($value:expr) => {
        crate::frontend::ast::Node {
            value: $value,
            position: crate::backend::interpreter::interpreter::tests::default_position(),
        }
    };
}

pub(super) use test_node;
