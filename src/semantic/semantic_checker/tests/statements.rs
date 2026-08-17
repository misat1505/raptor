use std::{collections::HashMap, vec};

use crate::{
    common::types::Type,
    frontend::ast::{Block, Expression, Literal, Program, Statement, SwitchCase, SwitchExpression},
    semantic::semantic_checker::tests::common::make_function,
};

use super::common::{empty_program, node, run_check};

#[test]
fn valid_declaration_has_no_errors() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        var_type: node!(Type::I64),
        identifier: node!(String::from("x")),
        value: Some(node!(Expression::Literal(Literal::I64(5)))),
    }));

    assert!(run_check(&program).is_empty());
}

#[test]
fn declaration_type_mismatch_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        var_type: node!(Type::I64),
        identifier: node!(String::from("x")),
        value: Some(node!(Expression::Literal(Literal::True))),
    }));

    let errors = run_check(&program);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("Cannot assign `bool` to `x`."));
    assert!(errors[0].contains("expected: i64"));
    assert!(errors[0].contains("found:    bool"));
}

#[test]
fn declaration_without_value_uses_default_type() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        var_type: node!(Type::I64),
        identifier: node!(String::from("x")),
        value: None,
    }));

    assert!(run_check(&program).is_empty());
}

#[test]
fn empty_vector_literal_matches_any_vector_type() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        var_type: node!(Type::Vector(Box::new(Type::I64))),
        identifier: node!(String::from("x")),
        value: Some(node!(Expression::Vector(vec![]))),
    }));

    assert!(run_check(&program).is_empty());
}

#[test]
fn vector_literal_with_mixed_types_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        var_type: node!(Type::Vector(Box::new(Type::I64))),
        identifier: node!(String::from("x")),
        value: Some(node!(Expression::Vector(vec![
            Box::new(node!(Expression::Literal(Literal::I64(1)))),
            Box::new(node!(Expression::Literal(Literal::True))),
        ]))),
    }));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("vector elements have mismatched types")));
}

#[test]
fn assignment_to_undeclared_variable_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Assignment {
        identifier: node!(String::from("x")),
        indices: vec![],
        value: node!(Expression::Literal(Literal::I64(5))),
    }));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("not declared")));
}

#[test]
fn assignment_type_mismatch_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        var_type: node!(Type::I64),
        identifier: node!(String::from("x")),
        value: Some(node!(Expression::Literal(Literal::I64(0)))),
    }));
    program.statements.push(node!(Statement::Assignment {
        identifier: node!(String::from("x")),
        indices: vec![],
        value: node!(Expression::Literal(Literal::True)),
    }));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("Cannot assign")));
}

#[test]
fn condition_must_be_bool_in_if() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Conditional {
        condition: node!(Expression::Literal(Literal::I64(1))),
        if_block: node!(Block(vec![])),
        else_block: None,
    }));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("if condition must be `bool`")));
}

#[test]
fn condition_must_be_bool_in_for_loop() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::ForLoop {
        declaration: None,
        condition: node!(Expression::Literal(Literal::I64(1))),
        assignment: None,
        block: node!(Block(vec![])),
    }));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("for loop condition must be `bool`")));
}

#[test]
fn break_outside_loop_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Break));

    let errors = run_check(&program);
    assert!(errors
        .iter()
        .any(|e| e.contains("Break statement is not inside a loop nor inside a switch case")));
}

#[test]
fn break_inside_for_loop_is_ok() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::ForLoop {
        declaration: None,
        condition: node!(Expression::Literal(Literal::True)),
        assignment: None,
        block: node!(Block(vec![node!(Statement::Break)])),
    }));

    assert!(run_check(&program).is_empty());
}

#[test]
fn while_loop_with_bool_condition_has_no_errors() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::WhileLoop {
        condition: node!(Expression::Literal(Literal::True)),
        block: node!(Block(vec![])),
    }));
    assert!(run_check(&program).is_empty());
}

#[test]
fn continue_outside_loop_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Continue));
    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("Continue statement is not inside a loop")));
}

#[test]
fn continue_inside_loop_is_ok() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::WhileLoop {
        condition: node!(Expression::Literal(Literal::True)),
        block: node!(Block(vec![node!(Statement::Continue)])),
    }));
    assert!(run_check(&program).is_empty());
}

#[test]
fn continue_inside_switch_case_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Switch {
        expressions: vec![],
        cases: vec![node!(SwitchCase {
            condition: node!(Expression::Literal(Literal::True)),
            block: node!(Block(vec![node!(Statement::Continue)])),
        })],
    }));
    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("Continue statement is not inside a loop")));
}

#[test]
fn switch_case_condition_must_be_bool() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Switch {
        expressions: vec![],
        cases: vec![node!(SwitchCase {
            condition: node!(Expression::Literal(Literal::I64(1))),
            block: node!(Block(vec![])),
        })],
    }));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("switch case condition must be `bool`")));
}

#[test]
fn switch_case_with_bool_condition_has_no_errors() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Switch {
        expressions: vec![],
        cases: vec![node!(SwitchCase {
            condition: node!(Expression::Literal(Literal::True)),
            block: node!(Block(vec![])),
        })],
    }));
    assert!(run_check(&program).is_empty());
}

#[test]
fn switch_expression_alias_is_declared_in_scope() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Switch {
        expressions: vec![node!(SwitchExpression {
            expression: node!(Expression::Literal(Literal::I64(5))),
            alias: Some(node!(String::from("x"))),
        })],
        cases: vec![node!(SwitchCase {
            condition: node!(Expression::Equal(
                Box::new(node!(Expression::Variable(String::from("x")))),
                Box::new(node!(Expression::Literal(Literal::I64(5)))),
            )),
            block: node!(Block(vec![])),
        })],
    }));

    assert!(run_check(&program).is_empty());
}

#[test]
fn break_inside_switch_case_is_ok() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Switch {
        expressions: vec![],
        cases: vec![node!(SwitchCase {
            condition: node!(Expression::Literal(Literal::True)),
            block: node!(Block(vec![node!(Statement::Break)])),
        })],
    }));

    assert!(run_check(&program).is_empty());
}

#[test]
fn break_inside_if_inside_for_loop_is_ok() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::ForLoop {
        declaration: None,
        condition: node!(Expression::Literal(Literal::True)),
        assignment: None,
        block: node!(Block(vec![node!(Statement::Conditional {
            condition: node!(Expression::Literal(Literal::True)),
            if_block: node!(Block(vec![node!(Statement::Break)])),
            else_block: None,
        })])),
    }));

    assert!(run_check(&program).is_empty());
}

#[test]
fn return_outside_function_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Return(None)));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("return statement is not inside a function")));
}

#[test]
fn return_with_value_outside_function_reports_error() {
    let mut program = empty_program();
    program
        .statements
        .push(node!(Statement::Return(Some(node!(Expression::Literal(Literal::I64(5)))))));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("return statement is not inside a function")));
}

#[test]
fn return_inside_function_does_not_report_placement_error() {
    let mut functions = HashMap::new();
    let (name, func) = make_function(
        "get_five",
        vec![],
        Type::I64,
        Block(vec![node!(Statement::Return(Some(node!(Expression::Literal(Literal::I64(5))))))]),
    );
    functions.insert(name, func);

    let program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
    };

    let errors = run_check(&program);
    assert!(!errors.iter().any(|e| e.contains("return statement is not inside a function")));
}

#[test]
fn return_inside_nested_if_inside_function_is_ok() {
    let mut functions = HashMap::new();
    let (name, func) = make_function(
        "conditional_return",
        vec![],
        Type::I64,
        Block(vec![node!(Statement::Conditional {
            condition: node!(Expression::Literal(Literal::True)),
            if_block: node!(Block(vec![node!(Statement::Return(Some(node!(Expression::Literal(Literal::I64(1))))))])),
            else_block: None,
        })]),
    );
    functions.insert(name, func);

    let program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
    };

    let errors = run_check(&program);
    assert!(!errors.iter().any(|e| e.contains("return statement is not inside a function")));
}

#[test]
fn else_block_is_type_checked() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Conditional {
        condition: node!(Expression::Literal(Literal::True)),
        if_block: node!(Block(vec![])),
        else_block: Some(node!(Block(vec![node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::Literal(Literal::True))),
        })]))),
    }));
    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("Cannot assign `bool` to `x`.")));
}

#[test]
fn nested_index_assignment_is_valid() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        var_type: node!(Type::Vector(Box::new(Type::Vector(Box::new(Type::I64))))),
        identifier: node!(String::from("matrix")),
        value: Some(node!(Expression::Vector(vec![Box::new(node!(Expression::Vector(vec![Box::new(
            node!(Expression::Literal(Literal::I64(1)))
        ),])))]))),
    }));
    program.statements.push(node!(Statement::Assignment {
        identifier: node!(String::from("matrix")),
        indices: vec![node!(Expression::Literal(Literal::I64(0))), node!(Expression::Literal(Literal::I64(0))),],
        value: node!(Expression::Literal(Literal::I64(42))),
    }));
    assert!(run_check(&program).is_empty());
}

#[test]
fn index_assignment_with_non_i64_index_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        var_type: node!(Type::Vector(Box::new(Type::I64))),
        identifier: node!(String::from("arr")),
        value: Some(node!(Expression::Vector(vec![Box::new(node!(Expression::Literal(Literal::I64(1))))]))),
    }));
    program.statements.push(node!(Statement::Assignment {
        identifier: node!(String::from("arr")),
        indices: vec![node!(Expression::Literal(Literal::True))],
        value: node!(Expression::Literal(Literal::I64(2))),
    }));
    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("array index must be `i64`")));
}

#[test]
fn index_assignment_into_non_vector_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        var_type: node!(Type::I64),
        identifier: node!(String::from("x")),
        value: Some(node!(Expression::Literal(Literal::I64(1)))),
    }));
    program.statements.push(node!(Statement::Assignment {
        identifier: node!(String::from("x")),
        indices: vec![node!(Expression::Literal(Literal::I64(0)))],
        value: node!(Expression::Literal(Literal::I64(2))),
    }));
    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("Cannot index into value of type")));
}

#[test]
fn index_assignment_type_mismatch_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        var_type: node!(Type::Vector(Box::new(Type::I64))),
        identifier: node!(String::from("arr")),
        value: Some(node!(Expression::Vector(vec![Box::new(node!(Expression::Literal(Literal::I64(1))))]))),
    }));
    program.statements.push(node!(Statement::Assignment {
        identifier: node!(String::from("arr")),
        indices: vec![node!(Expression::Literal(Literal::I64(0)))],
        value: node!(Expression::Literal(Literal::True)),
    }));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("Cannot assign `bool` to array element.")));
}

#[test]
fn index_assignment_updates_element() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        var_type: node!(Type::Vector(Box::new(Type::I64))),
        identifier: node!(String::from("arr")),
        value: Some(node!(Expression::Vector(vec![Box::new(node!(Expression::Literal(Literal::I64(1))))]))),
    }));
    program.statements.push(node!(Statement::Assignment {
        identifier: node!(String::from("arr")),
        indices: vec![node!(Expression::Literal(Literal::I64(0)))],
        value: node!(Expression::Literal(Literal::I64(99))),
    }));

    assert!(run_check(&program).is_empty());
}
