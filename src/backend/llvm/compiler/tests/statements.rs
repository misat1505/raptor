use inkwell::context::Context;

use crate::{
    backend::llvm::{
        compiler::{
            tests::{empty_program, node},
            Compiler,
        },
        OverflowPolicy,
    },
    common::types::Type,
    frontend::ast::{Block, Expression, Literal, Statement, VariableDeclarationKind},
};

fn with_main<'a, 'ctx>(program: &'a crate::frontend::ast::Program, context: &'ctx Context) -> Compiler<'a, 'ctx> {
    let mut c = Compiler::new(program, context, OverflowPolicy::Ignore);
    c.declare_main_function();
    c
}

#[test]
fn declare_i64_with_init() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let stmt = node(Statement::Declaration {
        identifier: node(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node(Type::I64),
            value: Some(node(Expression::Literal(Literal::I64(42)))),
        },
    });
    assert!(compiler.compile_statement(&stmt).is_ok());
    assert!(compiler.get_variable("x").is_ok());
}

#[test]
fn declare_i64_with_default() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let stmt = node(Statement::Declaration {
        identifier: node(String::from("y")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node(Type::I64),
            value: None,
        },
    });
    assert!(compiler.compile_statement(&stmt).is_ok());
    assert!(compiler.get_variable("y").is_ok());
}

#[test]
fn declare_bool_and_f64() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let b = node(Statement::Declaration {
        identifier: node(String::from("flag")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node(Type::Bool),
            value: Some(node(Expression::Literal(Literal::True))),
        },
    });
    assert!(compiler.compile_statement(&b).is_ok());

    let f = node(Statement::Declaration {
        identifier: node(String::from("pi")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node(Type::F64),
            value: Some(node(Expression::Literal(Literal::F64(3.14)))),
        },
    });
    assert!(compiler.compile_statement(&f).is_ok());
}

#[test]
fn assign_to_variable() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    // declare first
    let decl = node(Statement::Declaration {
        identifier: node(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node(Type::I64),
            value: Some(node(Expression::Literal(Literal::I64(0)))),
        },
    });
    compiler.compile_statement(&decl).unwrap();

    let assign = node(Statement::Assignment {
        identifier: node(String::from("x")),
        value: node(Expression::Literal(Literal::I64(99))),
        accessors: vec![],
    });
    assert!(compiler.compile_statement(&assign).is_ok());
}

#[test]
fn assign_undeclared_fails() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let assign = node(Statement::Assignment {
        identifier: node(String::from("missing")),
        value: node(Expression::Literal(Literal::I64(1))),
        accessors: vec![],
    });
    assert!(compiler.compile_statement(&assign).is_err());
}

#[test]
fn if_true_branch() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    // i64 x = 0;
    let binding = node(Statement::Declaration {
        identifier: node(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node(Type::I64),
            value: Some(node(Expression::Literal(Literal::I64(0)))),
        },
    });

    compiler.compile_statement(&binding).unwrap();

    let stmt = node(Statement::Conditional {
        condition: node(Expression::Literal(Literal::True)),
        if_block: node(Block(vec![node(Statement::Assignment {
            identifier: node(String::from("x")),
            value: node(Expression::Literal(Literal::I64(1))),
            accessors: vec![],
        })])),
        else_block: Some(node(Block(vec![node(Statement::Assignment {
            identifier: node(String::from("x")),
            value: node(Expression::Literal(Literal::I64(2))),
            accessors: vec![],
        })]))),
    });
    assert!(compiler.compile_statement(&stmt).is_ok());
}

#[test]
fn if_without_else() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let stmt = node(Statement::Conditional {
        condition: node(Expression::Literal(Literal::False)),
        if_block: node(Block(vec![])),
        else_block: None,
    });
    assert!(compiler.compile_statement(&stmt).is_ok());
}

#[test]
fn if_non_bool_condition_fails() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let stmt = node(Statement::Conditional {
        condition: node(Expression::Literal(Literal::I64(1))),
        if_block: node(Block(vec![])),
        else_block: None,
    });
    assert!(compiler.compile_statement(&stmt).is_err());
}

#[test]
fn while_false_never_enters_body() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let stmt = node(Statement::WhileLoop {
        condition: node(Expression::Literal(Literal::False)),
        block: node(Block(vec![])),
    });
    assert!(compiler.compile_statement(&stmt).is_ok());
    assert!(compiler.control_stack.is_empty());
}

#[test]
fn while_with_break() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let stmt = node(Statement::WhileLoop {
        condition: node(Expression::Literal(Literal::True)),
        block: node(Block(vec![node(Statement::Break)])),
    });
    assert!(compiler.compile_statement(&stmt).is_ok());
    assert!(compiler.control_stack.is_empty());
}

#[test]
fn for_loop_basic() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let stmt = node(Statement::ForLoop {
        declaration: Some(Box::new(node(Statement::Declaration {
            identifier: node(String::from("i")),
            kind: VariableDeclarationKind::TYPE {
                var_type: node(Type::I64),
                value: Some(node(Expression::Literal(Literal::I64(0)))),
            },
        }))),
        condition: node(Expression::Literal(Literal::False)),
        assignment: Some(Box::new(node(Statement::Assignment {
            identifier: node(String::from("i")),
            value: node(Expression::Addition(
                Box::new(node(Expression::Variable(String::from("i")))),
                Box::new(node(Expression::Literal(Literal::I64(1)))),
            )),
            accessors: vec![],
        }))),
        block: node(Block(vec![])),
    });
    assert!(compiler.compile_statement(&stmt).is_ok());
    assert!(compiler.control_stack.is_empty());
}

#[test]
fn for_loop_with_break() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let stmt = node(Statement::ForLoop {
        declaration: None,
        condition: node(Expression::Literal(Literal::True)),
        assignment: None,
        block: node(Block(vec![node(Statement::Break)])),
    });
    assert!(compiler.compile_statement(&stmt).is_ok());
}

#[test]
fn return_with_value() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    // main returns i32; returning i64 may still build IR
    let stmt = node(Statement::Return(Some(node(Expression::Literal(Literal::I64(0))))));
    // This terminates the current block — ok for testing the path
    assert!(compiler.compile_statement(&stmt).is_ok());
}

#[test]
fn return_void() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let stmt = node(Statement::Return(None));
    assert!(compiler.compile_statement(&stmt).is_ok());
}

#[test]
fn break_outside_loop_fails() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let stmt = node(Statement::Break);
    let err = compiler.compile_statement(&stmt).unwrap_err();
    assert!(err.message().contains("break"));
}

#[test]
fn continue_outside_loop_fails() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let stmt = node(Statement::Continue);
    let err = compiler.compile_statement(&stmt).unwrap_err();
    assert!(err.message().contains("continue"));
}

#[test]
fn while_with_continue() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    // while (false) { continue; } — body still compiled
    let stmt = node(Statement::WhileLoop {
        condition: node(Expression::Literal(Literal::False)),
        block: node(Block(vec![node(Statement::Continue)])),
    });
    assert!(compiler.compile_statement(&stmt).is_ok());
}
