use std::{assert_eq, cell::RefCell, rc::Rc, vec};

use super::{create_interpreter, setup_program, test_node};
use crate::{
    backend::interpreter::alu::value::Value,
    common::{span::Span, types::Type, visitor::Visitor},
    frontend::ast::{Accessor, Argument, Block, Expression, Literal, PassedBy, Statement, VariableDeclarationKind},
};

#[test]
fn declare_variable() {
    // i64 x = 5;
    let ast = test_node!(Statement::Declaration {
        identifier: test_node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: test_node!(Type::I64),
            value: Some(test_node!(Expression::Literal(Literal::I64(5)))),
        },
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_statement(&ast);
    assert_eq!(
        interpreter.stack.get_variable("x", Span::default()).unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(5)))
    );
}

#[test]
fn declare_variable_with_default_value() {
    // i64 x;
    let ast = test_node!(Statement::Declaration {
        identifier: test_node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: test_node!(Type::I64),
            value: None,
        },
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_statement(&ast);
    assert_eq!(
        interpreter.stack.get_variable("x", Span::default()).unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(0)))
    );
}

#[test]
fn declare_variable_bad_type() {
    // i64 x = false;
    let ast = test_node!(Statement::Declaration {
        identifier: test_node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: test_node!(Type::I64),
            value: Some(test_node!(Expression::Literal(Literal::False))),
        },
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn redeclare_variable_fails() {
    let ast = test_node!(Statement::Declaration {
        identifier: test_node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: test_node!(Type::I64),
            value: None,
        },
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let _ = interpreter.visit_statement(&ast);
    assert_eq!(
        interpreter.stack.get_variable("x", Span::default()).unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(0)))
    );

    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn declare_with_none_value_fails() {
    // i64 x = print("hello world");
    let ast = test_node!(Statement::Declaration {
        identifier: test_node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: test_node!(Type::I64),
            value: Some(test_node!(Expression::FunctionCall {
                identifier: test_node!(String::from("print")),
                arguments: vec![Box::new(test_node!(Argument {
                    value: test_node!(Expression::Literal(Literal::String(String::from("hello world")))),
                    passed_by: PassedBy::Value,
                }))],
            })),
        },
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn declare_with_bad_type_fails() {
    // i64 x = true;
    let ast = test_node!(Statement::Declaration {
        identifier: test_node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: test_node!(Type::I64),
            value: Some(test_node!(Expression::Literal(Literal::True))),
        },
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
        accessors: vec![]
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter
        .stack
        .declare_variable("x", Rc::new(RefCell::new(Value::I64(0))), Span::default());

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(
        interpreter.stack.get_variable("x", Span::default()).unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(1)))
    );
}

#[test]
fn assigns_bad_type_fails() {
    // i64 x = 0;
    // x = false;
    let ast = test_node!(Statement::Assignment {
        identifier: test_node!(String::from("x")),
        value: test_node!(Expression::Literal(Literal::False)),
        accessors: vec![]
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter
        .stack
        .declare_variable("x", Rc::new(RefCell::new(Value::I64(0))), Span::default());

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
        accessors: vec![]
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter
        .stack
        .declare_variable("x", Rc::new(RefCell::new(Value::I64(0))), Span::default());

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
            accessors: vec![]
        }),])),
        else_block: Some(test_node!(Block(vec![test_node!(Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Literal(Literal::I64(2))),
            accessors: vec![]
        }),]))),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter
        .stack
        .declare_variable("x", Rc::new(RefCell::new(Value::I64(0))), Span::default());

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(
        interpreter.stack.get_variable("x", Span::default()).unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(1)))
    );
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
            accessors: vec![]
        }),])),
        else_block: Some(test_node!(Block(vec![test_node!(Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Literal(Literal::I64(2))),
            accessors: vec![]
        }),]))),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter
        .stack
        .declare_variable("x", Rc::new(RefCell::new(Value::I64(0))), Span::default());

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(
        interpreter.stack.get_variable("x", Span::default()).unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(2)))
    );
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
    let _ = interpreter
        .stack
        .declare_variable("x", Rc::new(RefCell::new(Value::I64(0))), Span::default());

    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn assign_by_index() {
    // x[1] = 99;
    let ast = test_node!(Statement::Assignment {
        identifier: test_node!(String::from("x")),
        value: test_node!(Expression::Literal(Literal::I64(99))),
        accessors: vec![test_node!(Accessor::Index(test_node!(Expression::Literal(Literal::I64(1)))))]
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
        Span::default(),
    );

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(values.borrow()[1].borrow().clone(), Value::I64(99));
}

#[test]
fn declare_vector_variable() {
    // i64[] x = [1, 2, 3];
    let ast = test_node!(Statement::Declaration {
        identifier: test_node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: test_node!(Type::Vector(Box::new(Type::I64))),
            value: Some(test_node!(Expression::Vector(vec![
                Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
                Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
                Box::new(test_node!(Expression::Literal(Literal::I64(3)))),
            ]))),
        },
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    assert!(interpreter.visit_statement(&ast).is_ok());
}

#[test]
fn declare_vector_variable_wrong_inner_type_fails() {
    // i64[] x = ["a"];
    let ast = test_node!(Statement::Declaration {
        identifier: test_node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: test_node!(Type::Vector(Box::new(Type::I64))),
            value: Some(test_node!(Expression::Vector(vec![Box::new(test_node!(Expression::Literal(
                Literal::String(String::from("a"))
            ))),]))),
        },
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn index_assignment_out_of_bounds_fails() {
    let ast = test_node!(Statement::Assignment {
        identifier: test_node!(String::from("x")),
        value: test_node!(Expression::Literal(Literal::I64(99))),
        accessors: vec![test_node!(Accessor::Index(test_node!(Expression::Literal(Literal::I64(5)))))],
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
            values,
        })),
        Span::default(),
    );
    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn index_assignment_on_non_vector_fails() {
    let ast = test_node!(Statement::Assignment {
        identifier: test_node!(String::from("x")),
        value: test_node!(Expression::Literal(Literal::I64(1))),
        accessors: vec![test_node!(Accessor::Index(test_node!(Expression::Literal(Literal::I64(0)))))],
    });
    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter
        .stack
        .declare_variable("x", Rc::new(RefCell::new(Value::I64(0))), Span::default());
    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn string_index_assignment() {
    // s[0] = 'Z';
    let ast = test_node!(Statement::Assignment {
        identifier: test_node!(String::from("s")),
        value: test_node!(Expression::Literal(Literal::Char('Z'))),
        accessors: vec![test_node!(Accessor::Index(test_node!(Expression::Literal(Literal::I64(0)))))],
    });
    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter
        .stack
        .declare_variable("s", Rc::new(RefCell::new(Value::String(String::from("abc")))), Span::default());
    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(
        *interpreter.stack.get_variable("s", Span::default()).unwrap().borrow(),
        Value::String(String::from("Zbc"))
    );
}

#[test]
fn string_index_assignment_non_char_fails() {
    let ast = test_node!(Statement::Assignment {
        identifier: test_node!(String::from("s")),
        value: test_node!(Expression::Literal(Literal::I64(65))),
        accessors: vec![test_node!(Accessor::Index(test_node!(Expression::Literal(Literal::I64(0)))))],
    });
    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter
        .stack
        .declare_variable("s", Rc::new(RefCell::new(Value::String(String::from("abc")))), Span::default());
    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn nested_index_assignment() {
    // m[0][1] = 99;
    let ast = test_node!(Statement::Assignment {
        identifier: test_node!(String::from("m")),
        value: test_node!(Expression::Literal(Literal::I64(99))),
        accessors: vec![
            test_node!(Accessor::Index(test_node!(Expression::Literal(Literal::I64(0))))),
            test_node!(Accessor::Index(test_node!(Expression::Literal(Literal::I64(1))))),
        ],
    });
    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let inner0 = Rc::new(RefCell::new(vec![
        Rc::new(RefCell::new(Value::I64(1))),
        Rc::new(RefCell::new(Value::I64(2))),
    ]));
    let inner1 = Rc::new(RefCell::new(vec![
        Rc::new(RefCell::new(Value::I64(3))),
        Rc::new(RefCell::new(Value::I64(4))),
    ]));
    let outer = Rc::new(RefCell::new(vec![
        Rc::new(RefCell::new(Value::Vector {
            kind: Box::new(Type::I64),
            values: inner0.clone(),
        })),
        Rc::new(RefCell::new(Value::Vector {
            kind: Box::new(Type::I64),
            values: inner1,
        })),
    ]));
    let _ = interpreter.stack.declare_variable(
        "m",
        Rc::new(RefCell::new(Value::Vector {
            kind: Box::new(Type::Vector(Box::new(Type::I64))),
            values: outer,
        })),
        Span::default(),
    );
    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(*inner0.borrow()[1].borrow(), Value::I64(99));
}
