use inkwell::context::Context;

use crate::{
    backend::llvm::compiler::{
        tests::{empty_program, node},
        Compiler,
    },
    common::types::Type,
    frontend::ast::{Expression, Literal},
};

fn with_main<'a, 'ctx>(program: &'a crate::frontend::ast::Program, context: &'ctx Context) -> Compiler<'a, 'ctx> {
    let mut c = Compiler::new(program, context);
    c.declare_main_function();
    c
}

#[test]
fn compile_i64_literal() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::Literal(Literal::I64(42)));
    assert!(compiler.compile_expression(&expr).is_ok());
    let v = compiler.read_last_value().unwrap();
    assert_eq!(v.to_type(), Type::I64);
}

#[test]
fn compile_f64_literal() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::Literal(Literal::F64(3.14)));
    assert!(compiler.compile_expression(&expr).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::F64);
}

#[test]
fn compile_bool_literals() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let t = node(Expression::Literal(Literal::True));
    assert!(compiler.compile_expression(&t).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::Bool);

    let f = node(Expression::Literal(Literal::False));
    assert!(compiler.compile_expression(&f).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::Bool);
}

#[test]
fn compile_string_and_char_literals() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let s = node(Expression::Literal(Literal::String("hi".into())));
    assert!(compiler.compile_expression(&s).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::Str);

    let c = node(Expression::Literal(Literal::Char('x')));
    assert!(compiler.compile_expression(&c).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::Char);
}

#[test]
fn compile_boolean_negation() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::BooleanNegation(Box::new(node(Expression::Literal(Literal::True)))));
    assert!(compiler.compile_expression(&expr).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::Bool);
}

#[test]
fn compile_arithmetic_negation() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::ArithmeticNegation(Box::new(node(Expression::Literal(Literal::I64(5))))));
    assert!(compiler.compile_expression(&expr).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::I64);
}

#[test]
fn compile_boolean_negation_on_int_fails() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::BooleanNegation(Box::new(node(Expression::Literal(Literal::I64(1))))));
    assert!(compiler.compile_expression(&expr).is_err());
}

#[test]
fn compile_addition() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::Addition(
        Box::new(node(Expression::Literal(Literal::I64(2)))),
        Box::new(node(Expression::Literal(Literal::I64(3)))),
    ));
    assert!(compiler.compile_expression(&expr).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::I64);
}

#[test]
fn compile_subtraction_multiplication_division_modulo() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let ops: Vec<fn(Box<Node<Expression>>, Box<Node<Expression>>) -> Expression> = vec![
        |l, r| Expression::Subtraction(l, r),
        |l, r| Expression::Multiplication(l, r),
        |l, r| Expression::Division(l, r),
        |l, r| Expression::Modulo(l, r),
    ];

    let exprs: Vec<Node<Expression>> = ops
        .into_iter()
        .map(|op| {
            node(op(
                Box::new(node(Expression::Literal(Literal::I64(10)))),
                Box::new(node(Expression::Literal(Literal::I64(3)))),
            ))
        })
        .collect();

    for expr in &exprs {
        assert!(compiler.compile_expression(expr).is_ok());
        assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::I64);
    }
}

#[test]
fn compile_addition_type_mismatch_fails() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::Addition(
        Box::new(node(Expression::Literal(Literal::I64(1)))),
        Box::new(node(Expression::Literal(Literal::True))),
    ));
    assert!(compiler.compile_expression(&expr).is_err());
}

#[test]
fn compile_concatenation_and_alternative() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let and = node(Expression::Concatenation(
        Box::new(node(Expression::Literal(Literal::True))),
        Box::new(node(Expression::Literal(Literal::False))),
    ));
    assert!(compiler.compile_expression(&and).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::Bool);

    let or = node(Expression::Alternative(
        Box::new(node(Expression::Literal(Literal::True))),
        Box::new(node(Expression::Literal(Literal::False))),
    ));
    assert!(compiler.compile_expression(&or).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::Bool);
}

#[test]
fn compile_ordered_comparisons() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let ops: Vec<fn(Box<Node<Expression>>, Box<Node<Expression>>) -> Expression> = vec![
        |l, r| Expression::Greater(l, r),
        |l, r| Expression::GreaterEqual(l, r),
        |l, r| Expression::Less(l, r),
        |l, r| Expression::LessEqual(l, r),
    ];

    let exprs: Vec<Node<Expression>> = ops
        .into_iter()
        .map(|op| {
            node(op(
                Box::new(node(Expression::Literal(Literal::I64(5)))),
                Box::new(node(Expression::Literal(Literal::I64(3)))),
            ))
        })
        .collect();

    for expr in &exprs {
        assert!(compiler.compile_expression(expr).is_ok());
        assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::Bool);
    }
}

#[test]
fn compile_equality() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let eq = node(Expression::Equal(
        Box::new(node(Expression::Literal(Literal::I64(1)))),
        Box::new(node(Expression::Literal(Literal::I64(1)))),
    ));
    assert!(compiler.compile_expression(&eq).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::Bool);

    let ne = node(Expression::NotEqual(
        Box::new(node(Expression::Literal(Literal::True))),
        Box::new(node(Expression::Literal(Literal::False))),
    ));
    assert!(compiler.compile_expression(&ne).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::Bool);
}

#[test]
fn compile_cast_i64_to_f64() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::Casting {
        value: Box::new(node(Expression::Literal(Literal::I64(7)))),
        to_type: node(Type::F64),
    });
    assert!(compiler.compile_expression(&expr).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::F64);
}

#[test]
fn compile_cast_i64_to_bool() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::Casting {
        value: Box::new(node(Expression::Literal(Literal::I64(1)))),
        to_type: node(Type::Bool),
    });
    assert!(compiler.compile_expression(&expr).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::Bool);
}

#[test]
fn compile_variable_undeclared_fails() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::Variable(String::from("missing")));
    assert!(compiler.compile_expression(&expr).is_err());
}

#[test]
fn compile_variable_after_alloca() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    // manually declare a variable in the compiler's map
    let i64_ty = context.i64_type();
    let ptr = compiler.builder.build_alloca(i64_ty, "x").unwrap();
    compiler.builder.build_store(ptr, i64_ty.const_int(99, true)).unwrap();
    compiler.variables.insert("x".into(), (ptr, Type::I64));

    let expr = node(Expression::Variable(String::from("x")));
    assert!(compiler.compile_expression(&expr).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::I64);
}

#[test]
fn compile_index_into_int_fails() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::Index {
        collection: Box::new(node(Expression::Literal(Literal::I64(1)))),
        index: Box::new(node(Expression::Literal(Literal::I64(0)))),
    });
    assert!(compiler.compile_expression(&expr).is_err());
}

#[test]
fn compile_nested_arithmetic() {
    // (1 + 2) * 3
    let context = Context::create();
    let program = empty_program();
    let mut compiler = with_main(&program, &context);

    let expr = node(Expression::Multiplication(
        Box::new(node(Expression::Addition(
            Box::new(node(Expression::Literal(Literal::I64(1)))),
            Box::new(node(Expression::Literal(Literal::I64(2)))),
        ))),
        Box::new(node(Expression::Literal(Literal::I64(3)))),
    ));
    assert!(compiler.compile_expression(&expr).is_ok());
    assert_eq!(compiler.read_last_value().unwrap().to_type(), Type::I64);
}

use crate::frontend::ast::Node;
