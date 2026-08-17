use std::{collections::HashMap, rc::Rc};

use crate::{
    common::position::Position,
    frontend::ast::{Block, FunctionDeclaration, Node, Parameter, Program},
    semantic::semantic_checker::SemanticChecker,
};

pub fn pos() -> Position {
    Position {
        filename: None,
        line: 0,
        column: 0,
        offset: 0,
    }
}

#[macro_export]
macro_rules! node {
    ($value:expr) => {
        crate::frontend::ast::Node {
            value: $value,
            position: crate::semantic::semantic_checker::tests::common::pos(),
        }
    };
}
pub use node;

pub fn empty_program() -> Program {
    Program {
        statements: vec![],
        functions: HashMap::new(),
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
    }
}

pub fn run_check(program: &Program) -> Vec<String> {
    let mut checker = SemanticChecker::new(program).unwrap();
    checker.check();

    checker.errors.iter().map(|e| e.message()).collect()
}

pub fn make_function(
    name: &str,
    parameters: Vec<Node<Parameter>>,
    return_type: crate::common::types::Type,
    block: Block,
) -> (String, Rc<Node<FunctionDeclaration>>) {
    (
        name.to_string(),
        Rc::new(node!(FunctionDeclaration {
            identifier: node!(String::from(name)),
            parameters,
            return_type: node!(return_type),
            block: node!(block),
        })),
    )
}
