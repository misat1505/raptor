mod control_flow;
mod core;
mod expressions;
mod functions;
mod statements;
mod stringify;
mod vectors;
mod visitor;

use std::collections::HashMap;

use inkwell::context::Context;

use crate::{
    backend::llvm::compiler::Compiler,
    common::span::Span,
    frontend::ast::{Node, Program},
};

pub(super) fn empty_program() -> Program {
    Program {
        statements: vec![],
        extern_functions: HashMap::new(),
        functions: HashMap::new(),
        std_functions: HashMap::new(),
    }
}

pub(super) fn span() -> Span {
    Span::default()
}

pub(super) fn node<T>(value: T) -> Node<T> {
    Node { value, span: span() }
}

pub(super) fn with_main<'a, 'ctx>(program: &'a crate::frontend::ast::Program, context: &'ctx Context) -> Compiler<'a, 'ctx> {
    let mut compiler = Compiler::new(program, context);
    compiler.declare_main_function();
    compiler
}
