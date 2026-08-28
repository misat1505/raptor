use std::rc::Rc;

use inkwell::context::Context;

use crate::{
    backend::llvm::{
        compiler::{
            tests::{empty_program, node, span},
            Compiler,
        },
        OverflowPolicy,
    },
    common::types::Type,
    frontend::ast::{Argument, Block, Expression, ExternFunctionDeclaration, FunctionDeclaration, Literal, Parameter, PassedBy, Program, Statement},
};

fn with_main<'a, 'ctx>(program: &'a Program, context: &'ctx Context) -> Compiler<'a, 'ctx> {
    let mut c = Compiler::new(program, context, OverflowPolicy::Ignore);
    c.declare_main_function();
    c
}

fn param(name: &str, ty: Type, by: PassedBy) -> crate::frontend::ast::Node<Parameter> {
    node(Parameter {
        identifier: node(name.to_string()),
        parameter_type: node(ty),
        passed_by: by,
    })
}

fn fn_decl(name: &str, params: Vec<crate::frontend::ast::Node<Parameter>>, ret: Type, body: Block) -> FunctionDeclaration {
    FunctionDeclaration {
        identifier: node(name.to_string()),
        parameters: params,
        return_type: node(ret),
        block: node(body),
    }
}

fn extern_fn_decl(name: &str, params: Vec<crate::frontend::ast::Node<Parameter>>, ret: Type) -> ExternFunctionDeclaration {
    ExternFunctionDeclaration {
        identifier: node(name.to_string()),
        parameters: params,
        return_type: node(ret),
        alias: None,
    }
}

fn program_with_fn(name: &str, decl: FunctionDeclaration) -> Program {
    let mut p = empty_program();
    p.functions.insert(name.to_string(), Rc::new(node(decl)));
    p
}

fn program_with_extern(name: &str, decl: ExternFunctionDeclaration) -> Program {
    let mut p = empty_program();
    p.extern_functions.insert(name.to_string(), Rc::new(node(decl)));
    p
}

#[test]
fn declare_void_function_no_params() {
    let context = Context::create();
    let decl = fn_decl("foo", vec![], Type::Void, Block(vec![]));
    let program = program_with_fn("foo", decl);
    let mut compiler = with_main(&program, &context);

    assert!(compiler.declare_functions().is_ok());
    assert!(compiler.functions.contains_key("foo"));
    let f = compiler.functions["foo"];
    assert_eq!(f.get_name().to_str().unwrap(), "foo");
    assert_eq!(f.count_params(), 0);
}

#[test]
fn declare_function_with_value_params_and_return() {
    let context = Context::create();
    let decl = fn_decl(
        "add",
        vec![param("a", Type::I64, PassedBy::Value), param("b", Type::I64, PassedBy::Value)],
        Type::I64,
        Block(vec![]),
    );
    let program = program_with_fn("add", decl);
    let mut compiler = with_main(&program, &context);

    assert!(compiler.declare_functions().is_ok());
    let f = compiler.functions["add"];
    assert_eq!(f.count_params(), 2);
}

#[test]
fn declare_function_with_reference_param() {
    let context = Context::create();
    let decl = fn_decl("inc", vec![param("x", Type::I64, PassedBy::Reference)], Type::Void, Block(vec![]));
    let program = program_with_fn("inc", decl);
    let mut compiler = with_main(&program, &context);

    assert!(compiler.declare_functions().is_ok());
    assert_eq!(compiler.functions["inc"].count_params(), 1);
}

#[test]
fn declare_extern_function() {
    let context = Context::create();
    let decl = extern_fn_decl("puts", vec![param("s", Type::Str, PassedBy::Value)], Type::I32);
    let program = program_with_extern("puts", decl);
    let mut compiler = with_main(&program, &context);

    assert!(compiler.declare_extern_functions().is_ok());
    assert!(compiler.functions.contains_key("puts"));
}

#[test]
fn compile_void_function_body_adds_ret_void() {
    let context = Context::create();
    let decl = fn_decl("noop", vec![], Type::Void, Block(vec![]));
    let program = program_with_fn("noop", decl);
    let mut compiler = with_main(&program, &context);

    compiler.declare_functions().unwrap();
    assert!(compiler.compile_functions().is_ok());

    let f = compiler.functions["noop"];
    let entry = f.get_first_basic_block().unwrap();
    assert!(entry.get_terminator().is_some());
}

#[test]
fn compile_function_body_with_value_param() {
    let context = Context::create();
    let decl = fn_decl(
        "id",
        vec![param("x", Type::I64, PassedBy::Value)],
        Type::I64,
        Block(vec![node(Statement::Return(Some(node(Expression::Variable(String::from("x"))))))]),
    );
    let program = program_with_fn("id", decl);
    let mut compiler = with_main(&program, &context);

    compiler.declare_functions().unwrap();
    assert!(compiler.compile_functions().is_ok());
    // variables map should be restored after body
    assert!(compiler.variables.is_empty());
}

#[test]
fn build_function_call_value_args() {
    let context = Context::create();
    let decl = fn_decl(
        "add",
        vec![param("a", Type::I64, PassedBy::Value), param("b", Type::I64, PassedBy::Value)],
        Type::I64,
        Block(vec![node(Statement::Return(Some(node(Expression::Addition(
            Box::new(node(Expression::Variable(String::from("a")))),
            Box::new(node(Expression::Variable(String::from("b")))),
        )))))]),
    );
    let program = program_with_fn("add", decl);
    let mut compiler = with_main(&program, &context);

    compiler.declare_functions().unwrap();
    compiler.compile_functions().unwrap();

    // call add(1, 2) from main
    let args = vec![
        Box::new(node(Argument {
            value: node(Expression::Literal(Literal::I64(1))),
            passed_by: PassedBy::Value,
        })),
        Box::new(node(Argument {
            value: node(Expression::Literal(Literal::I64(2))),
            passed_by: PassedBy::Value,
        })),
    ];

    let binding = node(String::from("add"));
    assert!(compiler.build_function_call(&binding, &args, span()).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::I64);
}

#[test]
fn build_function_call_void_returns_no_value() {
    let context = Context::create();
    let decl = fn_decl("noop", vec![], Type::Void, Block(vec![]));
    let program = program_with_fn("noop", decl);
    let mut compiler = with_main(&program, &context);

    compiler.declare_functions().unwrap();
    compiler.compile_functions().unwrap();

    let args: Vec<Box<_>> = vec![];
    let binding = node(String::from("noop"));
    assert!(compiler.build_function_call(&binding, &args, span()).is_ok());
    // void → last_value is None
    assert!(compiler.read_last_value().is_err());
}

#[test]
fn build_function_call_unknown_fails() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let args: Vec<Box<_>> = vec![];
    let err = compiler.build_function_call(&node(String::from("missing")), &args, span()).unwrap_err();
    assert!(err.message().contains("not yet supported") || err.message().contains("Unknown"));
}

#[test]
fn build_function_call_by_reference() {
    let context = Context::create();
    let decl = fn_decl("touch", vec![param("x", Type::I64, PassedBy::Reference)], Type::Void, Block(vec![]));
    let program = program_with_fn("touch", decl);
    let mut compiler = with_main(&program, &context);

    compiler.declare_functions().unwrap();
    compiler.compile_functions().unwrap();

    // allocate a local and pass by reference
    let i64_ty = context.i64_type();
    let ptr = compiler.builder.build_alloca(i64_ty, "x").unwrap();
    compiler.builder.build_store(ptr, i64_ty.const_int(0, true)).unwrap();
    compiler.variables.insert("x".into(), (ptr, Type::I64));

    let args = vec![Box::new(node(Argument {
        value: node(Expression::Variable(String::from("x"))),
        passed_by: PassedBy::Reference,
    }))];
    assert!(compiler.build_function_call(&node(String::from("touch")), &args, span()).is_ok());
}

#[test]
fn resolve_reference_variable() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let i64_ty = context.i64_type();
    let ptr = compiler.builder.build_alloca(i64_ty, "v").unwrap();
    compiler.variables.insert("v".into(), (ptr, Type::I64));

    let expr = node(Expression::Variable(String::from("v")));
    let resolved = compiler.resolve_reference(&expr).unwrap();
    assert_eq!(resolved, ptr);
}

#[test]
fn resolve_reference_non_variable_fails() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::Literal(Literal::I64(1)));
    let err = compiler.resolve_reference(&expr).unwrap_err();
    assert!(err.message().contains("by reference"));
}

#[test]
fn resolve_reference_undeclared_fails() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::Variable(String::from("nope")));
    assert!(compiler.resolve_reference(&expr).is_err());
}
