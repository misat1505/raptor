use std::{assert_eq, cell::RefCell, collections::HashMap, rc::Rc, vec};

use super::{create_interpreter, setup_program, test_node};
use crate::{
    backend::interpreter::{alu::value::Value, interpreter::Interpreter},
    common::{span::Span, types::Type, visitor::Visitor},
    frontend::ast::{Argument, Block, Expression, FunctionDeclaration, Literal, Node, Parameter, PassedBy, Program, Statement},
};

#[test]
fn test_function_call() {
    let ast = test_node!(Statement::FunctionCall {
        identifier: test_node!(String::from("add")),
        arguments: vec![
            Box::new(test_node!(Argument {
                value: test_node!(Expression::Literal(Literal::I64(3))),
                passed_by: PassedBy::Value,
            })),
            Box::new(test_node!(Argument {
                value: test_node!(Expression::Literal(Literal::I64(4))),
                passed_by: PassedBy::Value,
            })),
        ],
    });

    let mut functions: HashMap<String, Rc<Node<FunctionDeclaration>>> = HashMap::new();

    functions.insert(
        String::from("add"),
        Rc::new(test_node!(FunctionDeclaration {
            identifier: test_node!(String::from("add")),
            parameters: vec![
                test_node!(Parameter {
                    passed_by: PassedBy::Value,
                    parameter_type: test_node!(Type::I64),
                    identifier: test_node!(String::from("a")),
                }),
                test_node!(Parameter {
                    passed_by: PassedBy::Value,
                    parameter_type: test_node!(Type::I64),
                    identifier: test_node!(String::from("b")),
                }),
            ],
            return_type: test_node!(Type::I64),
            block: test_node!(Block(vec![test_node!(Statement::Return(Some(test_node!(Expression::Addition(
                Box::new(test_node!(Expression::Variable(String::from("a")))),
                Box::new(test_node!(Expression::Variable(String::from("b")))),
            )))))])),
        })),
    );

    let program = Program {
        statements: vec![],
        std_functions: HashMap::new(),
        functions,
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    let mut interpreter = Interpreter::new(&program);
    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(interpreter.last_result, Some(Value::I64(7)));
    assert_eq!(interpreter.abort_state, None);
}

#[test]
fn bad_arg_type() {
    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let ast = FunctionDeclaration {
        identifier: test_node!(String::from("fun")),
        parameters: vec![test_node!(Parameter {
            passed_by: PassedBy::Value,
            parameter_type: test_node!(Type::I64),
            identifier: test_node!(String::from("x")),
        })],
        return_type: test_node!(Type::Void),
        block: test_node!(Block(vec![])),
    };

    interpreter.last_arguments = vec![Rc::new(RefCell::new(Value::F64(3.2)))];

    assert!(interpreter.execute_function(&ast, Span::default()).is_err())
}

#[test]
fn bad_return_type() {
    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let ast = FunctionDeclaration {
        identifier: test_node!(String::from("fun")),
        parameters: vec![],
        return_type: test_node!(Type::Void),
        block: test_node!(Block(vec![test_node!(Statement::Return(Some(test_node!(Expression::Literal(
            Literal::I64(1)
        ))))),])),
    };

    assert!(interpreter.execute_function(&ast, Span::default()).is_err())
}

#[test]
fn call_undeclared_function_fails() {
    let ast = test_node!(Statement::FunctionCall {
        identifier: test_node!(String::from("does_not_exist")),
        arguments: vec![],
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn call_function_wrong_arg_count_fails() {
    let ast = test_node!(Statement::FunctionCall {
        identifier: test_node!(String::from("add")),
        arguments: vec![Box::new(test_node!(Argument {
            value: test_node!(Expression::Literal(Literal::I64(1))),
            passed_by: PassedBy::Value,
        })),],
    });

    let mut functions: HashMap<String, Rc<Node<FunctionDeclaration>>> = HashMap::new();
    functions.insert(
        String::from("add"),
        Rc::new(test_node!(FunctionDeclaration {
            identifier: test_node!(String::from("add")),
            parameters: vec![
                test_node!(Parameter {
                    passed_by: PassedBy::Value,
                    parameter_type: test_node!(Type::I64),
                    identifier: test_node!(String::from("a")),
                }),
                test_node!(Parameter {
                    passed_by: PassedBy::Value,
                    parameter_type: test_node!(Type::I64),
                    identifier: test_node!(String::from("b")),
                }),
            ],
            return_type: test_node!(Type::I64),
            block: test_node!(Block(vec![])),
        })),
    );

    let program = Program {
        statements: vec![],
        std_functions: HashMap::new(),
        functions,
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    let mut interpreter = Interpreter::new(&program);
    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn call_function_by_reference() {
    // fn increment(&i64 x): void { x = x + 1; }
    // i64 y = 5; increment(&y);
    let mut functions: HashMap<String, Rc<Node<FunctionDeclaration>>> = HashMap::new();
    functions.insert(
        String::from("increment"),
        Rc::new(test_node!(FunctionDeclaration {
            identifier: test_node!(String::from("increment")),
            parameters: vec![test_node!(Parameter {
                passed_by: PassedBy::Reference,
                parameter_type: test_node!(Type::I64),
                identifier: test_node!(String::from("x")),
            }),],
            return_type: test_node!(Type::Void),
            block: test_node!(Block(vec![test_node!(Statement::Assignment {
                identifier: test_node!(String::from("x")),
                value: test_node!(Expression::Addition(
                    Box::new(test_node!(Expression::Variable(String::from("x")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(1))))
                )),
                accessors: vec![]
            }),])),
        })),
    );

    let ast = test_node!(Statement::FunctionCall {
        identifier: test_node!(String::from("increment")),
        arguments: vec![Box::new(test_node!(Argument {
            value: test_node!(Expression::Variable(String::from("y"))),
            passed_by: PassedBy::Reference,
        })),],
    });

    let program = Program {
        statements: vec![],
        std_functions: HashMap::new(),
        functions,
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    let mut interpreter = Interpreter::new(&program);
    let _ = interpreter
        .stack
        .declare_variable("y", Rc::new(RefCell::new(Value::I64(5))), Span::default());

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(
        interpreter.stack.get_variable("y", Span::default()).unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(6)))
    );
}
