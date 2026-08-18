use std::{assert_eq, cell::RefCell, rc::Rc};

use super::{create_interpreter, setup_program, test_node};
use crate::{
    backend::interpreter::alu::value::Value,
    common::{span::Span, types::Type, visitor::Visitor},
    frontend::ast::{Expression, Literal},
};

#[test]
fn interpret_casting() {
    let ast = test_node!(Expression::Casting {
        value: Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
        to_type: test_node!(Type::F64),
    });

    let exp = Some(Value::F64(2.0));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_boolean_negation() {
    let ast = test_node!(Expression::BooleanNegation(Box::new(test_node!(Expression::Literal(Literal::False)))));

    let exp = Some(Value::Bool(true));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_arithmetic_negation() {
    let ast = test_node!(Expression::ArithmeticNegation(Box::new(test_node!(Expression::Literal(Literal::I64(5))))));

    let exp = Some(Value::I64(-5));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_addition() {
    let ast = test_node!(Expression::Addition(
        Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
        Box::new(test_node!(Expression::Literal(Literal::I64(2))))
    ));

    let exp = Some(Value::I64(7));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_subtraction() {
    let ast = test_node!(Expression::Subtraction(
        Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
        Box::new(test_node!(Expression::Literal(Literal::I64(2))))
    ));

    let exp = Some(Value::I64(3));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_multiplication() {
    let ast = test_node!(Expression::Multiplication(
        Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
        Box::new(test_node!(Expression::Literal(Literal::I64(2))))
    ));

    let exp = Some(Value::I64(10));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_division() {
    let ast = test_node!(Expression::Division(
        Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
        Box::new(test_node!(Expression::Literal(Literal::I64(2))))
    ));

    let exp = Some(Value::I64(2));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_modulo() {
    let ast = test_node!(Expression::Modulo(
        Box::new(test_node!(Expression::Literal(Literal::I64(7)))),
        Box::new(test_node!(Expression::Literal(Literal::I64(3))))
    ));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, Some(Value::I64(1)));
}

#[test]
fn interpret_concatenation() {
    let ast = test_node!(Expression::Concatenation(
        Box::new(test_node!(Expression::Literal(Literal::True))),
        Box::new(test_node!(Expression::Literal(Literal::False)))
    ));

    let exp = Some(Value::Bool(false));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_alternative() {
    let ast = test_node!(Expression::Alternative(
        Box::new(test_node!(Expression::Literal(Literal::True))),
        Box::new(test_node!(Expression::Literal(Literal::False)))
    ));

    let exp = Some(Value::Bool(true));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_greater() {
    let ast = test_node!(Expression::Greater(
        Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
        Box::new(test_node!(Expression::Literal(Literal::I64(5))))
    ));

    let exp = Some(Value::Bool(false));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_greater_equal() {
    let ast = test_node!(Expression::GreaterEqual(
        Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
        Box::new(test_node!(Expression::Literal(Literal::I64(5))))
    ));

    let exp = Some(Value::Bool(true));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_less() {
    let ast = test_node!(Expression::Less(
        Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
        Box::new(test_node!(Expression::Literal(Literal::I64(5))))
    ));

    let exp = Some(Value::Bool(false));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_less_equal() {
    let ast = test_node!(Expression::LessEqual(
        Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
        Box::new(test_node!(Expression::Literal(Literal::I64(5))))
    ));

    let exp = Some(Value::Bool(true));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_equal() {
    let ast = test_node!(Expression::Equal(
        Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
        Box::new(test_node!(Expression::Literal(Literal::I64(5))))
    ));

    let exp = Some(Value::Bool(true));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_not_equal() {
    let ast = test_node!(Expression::NotEqual(
        Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
        Box::new(test_node!(Expression::Literal(Literal::I64(5))))
    ));

    let exp = Some(Value::Bool(false));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_literal() {
    let ast = test_node!(Expression::Literal(Literal::I64(5)));

    let exp = Some(Value::I64(5));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn interpret_variable() {
    let ast = test_node!(Expression::Variable(String::from("x")));

    let exp = Some(Value::I64(5));

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter
        .stack
        .declare_variable("x", Rc::new(RefCell::new(Value::I64(5))), Span::default());

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, exp);
}

#[test]
fn index_into_vector() {
    // i64[] x = [10, 20, 30];
    // x[1]
    let ast = test_node!(Expression::Index {
        collection: Box::new(test_node!(Expression::Variable(String::from("x")))),
        index: Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let values = Rc::new(RefCell::new(vec![
        Rc::new(RefCell::new(Value::I64(10))),
        Rc::new(RefCell::new(Value::I64(20))),
        Rc::new(RefCell::new(Value::I64(30))),
    ]));
    let _ = interpreter.stack.declare_variable(
        "x",
        Rc::new(RefCell::new(Value::Vector {
            kind: Box::new(Type::I64),
            values,
        })),
        Span::default(),
    );

    let _ = interpreter.visit_expression(&ast);
    assert_eq!(interpreter.last_result, Some(Value::I64(20)));
}

#[test]
fn index_out_of_bounds_fails() {
    let ast = test_node!(Expression::Index {
        collection: Box::new(test_node!(Expression::Variable(String::from("x")))),
        index: Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let values = Rc::new(RefCell::new(vec![Rc::new(RefCell::new(Value::I64(10)))]));
    let _ = interpreter.stack.declare_variable(
        "x",
        Rc::new(RefCell::new(Value::Vector {
            kind: Box::new(Type::I64),
            values,
        })),
        Span::default(),
    );

    assert!(interpreter.visit_expression(&ast).is_err());
}
