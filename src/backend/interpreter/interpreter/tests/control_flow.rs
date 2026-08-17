use std::{assert_eq, cell::RefCell, collections::HashMap, rc::Rc, vec};

use super::{create_interpreter, setup_program, test_node};
use crate::{
    backend::interpreter::{alu::value::Value, interpreter::Interpreter},
    common::{types::Type, visitor::Visitor},
    frontend::ast::{Block, Expression, FunctionDeclaration, Literal, Node, Program, Statement, SwitchCase, SwitchExpression},
};

#[test]
fn for_loop() {
    // i64 total = 0;
    // for (i64 i = 1; i <= 5; i = i + 1) {total = total + i;}
    let ast = test_node!(Statement::ForLoop {
        declaration: Some(Box::new(test_node!(Statement::Declaration {
            var_type: test_node!(Type::I64),
            identifier: test_node!(String::from("i")),
            value: Some(test_node!(Expression::Literal(Literal::I64(1)))),
        }))),
        condition: test_node!(Expression::LessEqual(
            Box::new(test_node!(Expression::Variable(String::from("i")))),
            Box::new(test_node!(Expression::Literal(Literal::I64(5))))
        )),
        assignment: Some(Box::new(test_node!(Statement::Assignment {
            identifier: test_node!(String::from("i")),
            value: test_node!(Expression::Addition(
                Box::new(test_node!(Expression::Variable(String::from("i")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(1))))
            )),
            indices: vec![]
        }))),
        block: test_node!(Block(vec![test_node!(Statement::Assignment {
            identifier: test_node!(String::from("total")),
            value: test_node!(Expression::Addition(
                Box::new(test_node!(Expression::Variable(String::from("total")))),
                Box::new(test_node!(Expression::Variable(String::from("i"))))
            )),
            indices: vec![]
        }),])),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("total", Rc::new(RefCell::new(Value::I64(0))));

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(
        interpreter.stack.get_variable("total").unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(15)))
    );
}

#[test]
fn for_loop_second_variant() {
    // i64 total = 0;
    // i64 i = 1;
    // for (;i <= 5;) {total = total + i; i = i + 1}
    let ast = test_node!(Statement::ForLoop {
        declaration: None,
        condition: test_node!(Expression::LessEqual(
            Box::new(test_node!(Expression::Variable(String::from("i")))),
            Box::new(test_node!(Expression::Literal(Literal::I64(5))))
        )),
        assignment: None,
        block: test_node!(Block(vec![
            test_node!(Statement::Assignment {
                identifier: test_node!(String::from("total")),
                value: test_node!(Expression::Addition(
                    Box::new(test_node!(Expression::Variable(String::from("total")))),
                    Box::new(test_node!(Expression::Variable(String::from("i"))))
                )),
                indices: vec![]
            }),
            test_node!(Statement::Assignment {
                identifier: test_node!(String::from("i")),
                value: test_node!(Expression::Addition(
                    Box::new(test_node!(Expression::Variable(String::from("i")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(1))))
                )),
                indices: vec![]
            }),
        ])),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("total", Rc::new(RefCell::new(Value::I64(0))));
    let _ = interpreter.stack.declare_variable("i", Rc::new(RefCell::new(Value::I64(1))));

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(
        interpreter.stack.get_variable("total").unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(15)))
    );
}

#[test]
fn for_loop_bad_condition_type() {
    // for (;1;) {}
    let ast = test_node!(Statement::ForLoop {
        declaration: None,
        condition: test_node!(Expression::Literal(Literal::I64(1))),
        assignment: None,
        block: test_node!(Block(vec![])),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    assert!(interpreter.visit_statement(&ast).is_err());
}

#[test]
fn for_loop_with_break() {
    // i64 i = 0;
    // for (;true; i = i + 1) {if (i == 5) {break;}}
    let ast = test_node!(Statement::ForLoop {
        declaration: None,
        condition: test_node!(Expression::Literal(Literal::True)),
        assignment: Some(Box::new(test_node!(Statement::Assignment {
            identifier: test_node!(String::from("i")),
            value: test_node!(Expression::Addition(
                Box::new(test_node!(Expression::Variable(String::from("i")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(1))))
            )),
            indices: vec![]
        }))),
        block: test_node!(Block(vec![test_node!(Statement::Conditional {
            condition: test_node!(Expression::Equal(
                Box::new(test_node!(Expression::Variable(String::from("i")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(5))))
            )),
            if_block: test_node!(Block(vec![test_node!(Statement::Break)])),
            else_block: None,
        })])),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("i", Rc::new(RefCell::new(Value::I64(0))));

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(interpreter.abort_state, None);
    assert_eq!(interpreter.stack.get_variable("i").unwrap().clone(), Rc::new(RefCell::new(Value::I64(5))));
}

#[test]
fn for_loop_with_continue() {
    // i64 total = 0;
    // for (i64 i = 0; i < 5; i = i + 1) { if (i == 2) { continue; } total = total + i; }
    let ast = test_node!(Statement::ForLoop {
        declaration: Some(Box::new(test_node!(Statement::Declaration {
            var_type: test_node!(Type::I64),
            identifier: test_node!(String::from("i")),
            value: Some(test_node!(Expression::Literal(Literal::I64(0)))),
        }))),
        condition: test_node!(Expression::Less(
            Box::new(test_node!(Expression::Variable(String::from("i")))),
            Box::new(test_node!(Expression::Literal(Literal::I64(5))))
        )),
        assignment: Some(Box::new(test_node!(Statement::Assignment {
            identifier: test_node!(String::from("i")),
            value: test_node!(Expression::Addition(
                Box::new(test_node!(Expression::Variable(String::from("i")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(1))))
            )),
            indices: vec![]
        }))),
        block: test_node!(Block(vec![
            test_node!(Statement::Conditional {
                condition: test_node!(Expression::Equal(
                    Box::new(test_node!(Expression::Variable(String::from("i")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(2))))
                )),
                if_block: test_node!(Block(vec![test_node!(Statement::Continue)])),
                else_block: None,
            }),
            test_node!(Statement::Assignment {
                identifier: test_node!(String::from("total")),
                value: test_node!(Expression::Addition(
                    Box::new(test_node!(Expression::Variable(String::from("total")))),
                    Box::new(test_node!(Expression::Variable(String::from("i"))))
                )),
                indices: vec![]
            }),
        ])),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("total", Rc::new(RefCell::new(Value::I64(0))));

    assert!(interpreter.visit_statement(&ast).is_ok());
    // 0+1+3+4 = 8 (pomija i == 2)
    assert_eq!(
        interpreter.stack.get_variable("total").unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(8)))
    );
    assert_eq!(interpreter.abort_state, None);
}

fn create_test_switch_case() -> Node<Statement> {
    // switch (x) {
    //      (x < 15) {
    //          result = 15;
    //      } (x < 10) {
    //          result = 10;
    //          break;
    //      } (x < 5) {
    //          result = 5;
    //      }
    // }

    fn create_assignment(val: i64) -> Node<Statement> {
        test_node!(Statement::Assignment {
            identifier: test_node!(String::from("result")),
            value: test_node!(Expression::Literal(Literal::I64(val))),
            indices: vec![]
        })
    }

    fn create_condition(val: i64) -> Node<Expression> {
        test_node!(Expression::Less(
            Box::new(test_node!(Expression::Variable(String::from("x")))),
            Box::new(test_node!(Expression::Literal(Literal::I64(val)))),
        ))
    }

    test_node!(Statement::Switch {
        expressions: vec![test_node!(SwitchExpression {
            expression: test_node!(Expression::Variable(String::from("x"))),
            alias: None,
        }),],
        cases: vec![
            test_node!(SwitchCase {
                condition: create_condition(15),
                block: test_node!(Block(vec![create_assignment(15)])),
            }),
            test_node!(SwitchCase {
                condition: create_condition(10),
                block: test_node!(Block(vec![create_assignment(10), test_node!(Statement::Break),])),
            }),
            test_node!(SwitchCase {
                condition: create_condition(5),
                block: test_node!(Block(vec![create_assignment(5)])),
            }),
        ],
    })
}

#[test]
fn switch_enters() {
    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(12))));
    let _ = interpreter
        .stack
        .declare_variable("result", Rc::new(RefCell::new(Value::default_value(&Type::I64).unwrap())));

    let switch_case = &create_test_switch_case();
    let _ = interpreter.visit_statement(switch_case);

    assert_eq!(
        interpreter.stack.get_variable("result").unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(15)))
    );
    assert_eq!(interpreter.abort_state, None);
}

#[test]
fn switch_breaks() {
    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(3))));
    let _ = interpreter
        .stack
        .declare_variable("result", Rc::new(RefCell::new(Value::default_value(&Type::I64).unwrap())));

    let switch_case = &create_test_switch_case();
    let _ = interpreter.visit_statement(switch_case);

    assert_eq!(
        interpreter.stack.get_variable("result").unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(10)))
    );
    assert_eq!(interpreter.abort_state, None);
}

#[test]
fn switch_no_entry() {
    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(2137))));
    let _ = interpreter
        .stack
        .declare_variable("result", Rc::new(RefCell::new(Value::default_value(&Type::I64).unwrap())));

    let switch_case = &create_test_switch_case();
    let _ = interpreter.visit_statement(switch_case);

    assert_eq!(
        interpreter.stack.get_variable("result").unwrap().clone(),
        Rc::new(RefCell::new(Value::I64(0)))
    );
    assert_eq!(interpreter.abort_state, None);
}

#[test]
fn switch_bad_condition_type() {
    // switch () {
    //      (1) -> {}
    // }
    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let ast = test_node!(Statement::Switch {
        expressions: vec![],
        cases: vec![test_node!(SwitchCase {
            condition: test_node!(Expression::Literal(Literal::I64(1))),
            block: test_node!(Block(vec![])),
        }),],
    });

    assert!(interpreter.visit_statement(&ast).is_err())
}

#[test]
fn break_called_outside_for_or_switch() {
    let program = Program {
        functions: HashMap::new(),
        std_functions: HashMap::new(),
        statements: vec![test_node!(Statement::Conditional {
            condition: test_node!(Expression::Literal(Literal::True)),
            if_block: test_node!(Block(vec![test_node!(Statement::Break),])),
            else_block: None,
        })],
        extern_functions: HashMap::new(),
    };

    let mut interpreter = Interpreter::new(&program);
    assert!(interpreter.interpret().is_err())
}

#[test]
fn break_called_outside_for_or_switch_in_function() {
    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let ast = FunctionDeclaration {
        identifier: test_node!(String::from("fun")),
        parameters: vec![],
        return_type: test_node!(Type::Void),
        block: test_node!(Block(vec![test_node!(Statement::Break),])),
    };

    assert!(interpreter.execute_function(&ast).is_err())
}

#[test]
fn return_called_outside_for_or_switch() {
    let program = Program {
        functions: HashMap::new(),
        std_functions: HashMap::new(),
        statements: vec![test_node!(Statement::Conditional {
            condition: test_node!(Expression::Literal(Literal::True)),
            if_block: test_node!(Block(vec![test_node!(Statement::Return(None)),])),
            else_block: None,
        })],
        extern_functions: HashMap::new(),
    };

    let mut interpreter = Interpreter::new(&program);
    assert!(interpreter.interpret().is_err())
}

#[test]
fn continue_called_outside_for_or_while() {
    let program = Program {
        functions: HashMap::new(),
        std_functions: HashMap::new(),
        statements: vec![test_node!(Statement::Conditional {
            condition: test_node!(Expression::Literal(Literal::True)),
            if_block: test_node!(Block(vec![test_node!(Statement::Continue),])),
            else_block: None,
        })],
        extern_functions: HashMap::new(),
    };

    let mut interpreter = Interpreter::new(&program);
    assert!(interpreter.interpret().is_err())
}

#[test]
fn continue_called_outside_for_or_while_in_function() {
    let program = setup_program();
    let mut interpreter = create_interpreter(&program);

    let ast = FunctionDeclaration {
        identifier: test_node!(String::from("fun")),
        parameters: vec![],
        return_type: test_node!(Type::Void),
        block: test_node!(Block(vec![test_node!(Statement::Continue),])),
    };

    assert!(interpreter.execute_function(&ast).is_err())
}

#[test]
fn while_loop() {
    // i64 i = 0;
    // while (i < 5) { i = i + 1; }
    let ast = test_node!(Statement::WhileLoop {
        condition: test_node!(Expression::Less(
            Box::new(test_node!(Expression::Variable(String::from("i")))),
            Box::new(test_node!(Expression::Literal(Literal::I64(5))))
        )),
        block: test_node!(Block(vec![test_node!(Statement::Assignment {
            identifier: test_node!(String::from("i")),
            value: test_node!(Expression::Addition(
                Box::new(test_node!(Expression::Variable(String::from("i")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(1))))
            )),
            indices: vec![]
        }),])),
    });

    let program = setup_program();
    let mut interpreter = create_interpreter(&program);
    let _ = interpreter.stack.declare_variable("i", Rc::new(RefCell::new(Value::I64(0))));

    assert!(interpreter.visit_statement(&ast).is_ok());
    assert_eq!(interpreter.stack.get_variable("i").unwrap().clone(), Rc::new(RefCell::new(Value::I64(5))));
    assert_eq!(interpreter.abort_state, None);
}
