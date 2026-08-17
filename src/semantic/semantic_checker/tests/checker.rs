use std::vec;

use crate::{
    common::types::Type,
    frontend::ast::{Block, Expression, Literal, Statement},
};

use super::common::{empty_program, node, run_check};

#[test]
fn empty_program_has_no_errors() {
    let program = empty_program();
    assert!(run_check(&program).is_empty());
}

#[test]
fn duplicate_declaration_reports_error() {
    let mut program = empty_program();
    for _ in 0..2 {
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("x")),
            value: None,
        }));
    }
    assert!(!run_check(&program).is_empty());
}

#[test]
fn variable_declared_in_block_does_not_escape_scope() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Conditional {
        condition: node!(Expression::Literal(Literal::True)),
        if_block: node!(Block(vec![node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::Literal(Literal::I64(1)))),
        })])),
        else_block: None,
    }));
    program.statements.push(node!(Statement::Assignment {
        identifier: node!(String::from("x")),
        indices: vec![],
        value: node!(Expression::Literal(Literal::I64(2))),
    }));
    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("not declared")));
}
