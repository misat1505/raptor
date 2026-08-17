use std::{assert_eq, cell::RefCell, rc::Rc, vec};

use super::{create_interpreter, setup_program, test_node};
use crate::{
    backend::interpreter::alu::value::Value,
    common::{types::Type, visitor::Visitor},
    frontend::ast::{Argument, Block, Expression, Literal, PassedBy, Statement},
};

#[test]
fn declare_variable() {
    // i64 x = 5;
    let ast = test_node!(Statement::Declaration {
        var_type: test_node!(Type::I64),
        identifier: test_node!(String::from("x")),
        value: Some(test_node!(Expression::Literal(Literal::I64(5)))),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_statement(&ast);
    assert_eq!(interpreter.stack.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(5))));
}

#[test]
fn declare_variable_with_default_value() {
    // i64 x;
    let ast = test_node!(Statement::Declaration {
        var_type: test_node!(Type::I64),
        identifier: test_node!(String::from("x")),
        value: None,
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_statement(&ast);
    assert_eq!(interpreter.stack.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(0))));
}

#[test]
fn declare_variable_bad_type() {
    // i64 x = false;
    let ast = test_node!(Statement::Declaration {
        var_type: test_node!(Type::I64),
        identifier: test_node!(String::from("x")),
        value: Some(test_node!(Expression::Literal(Literal::False))),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn redeclare_variable_fails() {
    let ast = test_node!(Statement::Declaration {
        var_type: test_node!(Type::I64),
        identifier: test_node!(String::from("x")),
        value: None,
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_statement(&ast);
    assert_eq!(interpreter.stack.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(0))));

    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn declare_with_none_value_fails() {
    // i64 x = print("hello world");
    let ast = test_node!(Statement::Declaration {
        var_type: test_node!(Type::I64),
        identifier: test_node!(String::from("x")),
        value: Some(test_node!(Expression::FunctionCall {
            identifier: test_node!(String::from("print")),
            arguments: vec![Box::new(test_node!(Argument {
                value: test_node!(Expression::Literal(Literal::String(String::from("hello world")))),
                passed_by: PassedBy::Value,
            })),],
        })),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn declare_with_bad_type_fails() {
    // i64 x = true;
    let ast = test_node!(Statement::Declaration {
        var_type: test_node!(Type::I64),
        identifier: test_node!(String::from("x")),
        value: Some(test_node!(Expression::Literal(Literal::True))),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn assigns_to_variable() {
    // i64 x = 0;
    // x = 5;
    let ast = test_node!(Statement::Assignment {
        identifier: test_node!(String::from("x")),
        value: test_node!(Expression::Literal(Literal::I64(1))),
        indices: vec![]
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(0))));

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(interpreter.stack.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(1))));
}

#[test]
fn assigns_bad_type_fails() {
    // i64 x = 0;
    // x = false;
    let ast = test_node!(Statement::Assignment {
        identifier: test_node!(String::from("x")),
        value: test_node!(Expression::Literal(Literal::False)),
        indices: vec![]
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(0))));

    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn assign_with_none_value_fails() {
    // x = print("hello world");
    let ast = test_node!(Statement::Assignment {
        identifier: test_node!(String::from("x")),
        value: test_node!(Expression::FunctionCall {
            identifier: test_node!(String::from("print")),
            arguments: vec![Box::new(test_node!(Argument {
                value: test_node!(Expression::Literal(Literal::String(String::from("hello world")))),
                passed_by: PassedBy::Value,
            })),],
        }),
        indices: vec![]
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(0))));

    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn if_true_branch() {
    // i64 x = 0;
    // if (true) {x = 1;} else {x = 2;}
    let ast = test_node!(Statement::Conditional {
        condition: test_node!(Expression::Literal(Literal::True)),
        if_block: test_node!(Block(vec![test_node!(Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Literal(Literal::I64(1))),
            indices: vec![]
        }),])),
        else_block: Some(test_node!(Block(vec![test_node!(Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Literal(Literal::I64(2))),
            indices: vec![]
        }),]))),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(0))));

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(interpreter.stack.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(1))));
}

#[test]
fn if_false_branch() {
    // i64 x = 0;
    // if (false) {x = 1;} else {x = 2;}
    let ast = test_node!(Statement::Conditional {
        condition: test_node!(Expression::Literal(Literal::False)),
        if_block: test_node!(Block(vec![test_node!(Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Literal(Literal::I64(1))),
            indices: vec![]
        }),])),
        else_block: Some(test_node!(Block(vec![test_node!(Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Literal(Literal::I64(2))),
            indices: vec![]
        }),]))),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(0))));

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(interpreter.stack.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(2))));
}

#[test]
fn if_bad_condition_type_fails() {
    // i64 x = 0;
    // if (2137) {}
    let ast = test_node!(Statement::Conditional {
        condition: test_node!(Expression::Literal(Literal::I64(2137))),
        if_block: test_node!(Block(vec![])),
        else_block: None,
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(0))));

    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn assign_by_index() {
    // x[1] = 99;
    let ast = test_node!(Statement::Assignment {
        identifier: test_node!(String::from("x")),
        value: test_node!(Expression::Literal(Literal::I64(99))),
        indices: vec![test_node!(Expression::Literal(Literal::I64(1)))]
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let values = Rc::new(RefCell::new(vec![
        Rc::new(RefCell::new(Value::I64(10))),
        Rc::new(RefCell::new(Value::I64(20))),
    ]));
    let _ = interpreter.stack.declare_variable(
        "x",
        Rc::new(RefCell::new(Value::Vector {
            kind: Box::new(Type::I64),
            values: values.clone(),
        })),
    );

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(values.borrow()[1].borrow().clone(), Value::I64(99));
}

#[test]
fn declare_vector_variable() {
    // i64[] x = [1, 2, 3];
    let ast = test_node!(Statement::Declaration {
        var_type: test_node!(Type::Vector(Box::new(Type::I64))),
        identifier: test_node!(String::from("x")),
        value: Some(test_node!(Expression::Vector(vec![
            Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(3)))),
        ]))),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    assert!(interpreter.visit_statement(&ast).is_ok());
}

#[test]
fn declare_vector_variable_wrong_inner_type_fails() {
    // i64[] x = ["a"];
    let ast = test_node!(Statement::Declaration {
        var_type: test_node!(Type::Vector(Box::new(Type::I64))),
        identifier: test_node!(String::from("x")),
        value: Some(test_node!(Expression::Vector(vec![Box::new(test_node!(Expression::Literal(
            Literal::String(String::from("a"))
        ))),]))),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    assert!(interpreter.visit_statement(&ast).is_err());
}
