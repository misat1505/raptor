use std::{collections::HashMap, rc::Rc};

use crate::{
    frontend::ast::{Block, FunctionDeclaration, Node, Parameter, Program},
    semantic::semantic_checker::SemanticChecker,
};

#[macro_export]
macro_rules! node {
    ($value:expr) => {
        crate::frontend::ast::Node {
            value: $value,
            span: crate::common::span::Span::default(),
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
        declared_types: HashMap::new(),
        types: HashMap::new(),
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

pub fn assert_one_unused_warning(errors: &Vec<String>) {
    let unused: Vec<_> = errors.iter().filter(|e| e.contains("Unused variable")).collect();
    assert_eq!(
        unused.len(),
        1,
        "expected exactly 1 unused-variable warning, got {} in: {:?}",
        unused.len(),
        errors
    );
}
