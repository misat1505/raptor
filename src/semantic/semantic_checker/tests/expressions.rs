use std::vec;

use crate::{
    common::types::Type,
    frontend::ast::{Block, Expression, Literal, Statement, VariableDeclarationKind},
    semantic::semantic_checker::tests::common::assert_one_unused_warning,
};

use super::common::{empty_program, node, run_check};

#[test]
fn valid_binary_addition_has_no_errors() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::I64),
            value: Some(node!(Expression::Addition(
                Box::new(node!(Expression::Literal(Literal::I64(1)))),
                Box::new(node!(Expression::Literal(Literal::I64(2)))),
            ))),
        },
    }));

    assert_one_unused_warning(&run_check(&program));
}

#[test]
fn invalid_binary_addition_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::I64),
            value: Some(node!(Expression::Addition(
                Box::new(node!(Expression::Literal(Literal::I64(1)))),
                Box::new(node!(Expression::Literal(Literal::True))),
            ))),
        },
    }));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("Cannot perform addition")));
}

#[test]
fn undeclared_variable_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("y")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::I64),
            value: Some(node!(Expression::Variable(String::from("x")))),
        },
    }));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("not declared")));
}

#[test]
fn index_expression_on_vector_returns_element_type() {
    let mut program = empty_program();

    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("arr")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::Vector(Box::new(Type::I64))),
            value: Some(node!(Expression::Vector(vec![Box::new(node!(Expression::Literal(Literal::I64(1)))),]))),
        },
    }));

    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::I64),
            value: Some(node!(Expression::Index {
                collection: Box::new(node!(Expression::Variable(String::from("arr")))),
                index: Box::new(node!(Expression::Literal(Literal::I64(0)))),
            })),
        },
    }));

    assert_one_unused_warning(&run_check(&program));
}

#[test]
fn index_expression_on_non_vector_reports_error() {
    let mut program = empty_program();

    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::I64),
            value: Some(node!(Expression::Literal(Literal::I64(1)))),
        },
    }));

    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("y")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::I64),
            value: Some(node!(Expression::Index {
                collection: Box::new(node!(Expression::Variable(String::from("x")))),
                index: Box::new(node!(Expression::Literal(Literal::I64(0)))),
            })),
        },
    }));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("Cannot index into a value of type")));
}

#[test]
fn index_with_non_i64_index_reports_error() {
    let mut program = empty_program();

    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("arr")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::Vector(Box::new(Type::I64))),
            value: Some(node!(Expression::Vector(vec![Box::new(node!(Expression::Literal(Literal::I64(1)))),]))),
        },
    }));

    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("y")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::I64),
            value: Some(node!(Expression::Index {
                collection: Box::new(node!(Expression::Variable(String::from("arr")))),
                index: Box::new(node!(Expression::Literal(Literal::True))),
            })),
        },
    }));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("Array index must be of type `i64`.")));
}

#[test]
fn casting_valid_types_has_no_errors() {
    let mut program = empty_program();

    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::F64),
            value: Some(node!(Expression::Casting {
                value: Box::new(node!(Expression::Literal(Literal::I64(5)))),
                to_type: node!(Type::F64),
            })),
        },
    }));

    assert_one_unused_warning(&run_check(&program));
}

#[test]
fn casting_invalid_types_reports_error() {
    let mut program = empty_program();

    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::I64),
            value: Some(node!(Expression::Casting {
                value: Box::new(node!(Expression::Vector(vec![]))),
                to_type: node!(Type::I64),
            })),
        },
    }));

    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("Cannot cast")));
}

#[test]
fn boolean_negation_accepts_bool() {
    let mut program = empty_program();

    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::Bool),
            value: Some(node!(Expression::BooleanNegation(Box::new(node!(Expression::Literal(Literal::True)))))),
        },
    }));

    assert_one_unused_warning(&run_check(&program));
}

#[test]
fn boolean_negation_rejects_non_bool() {
    let mut program = empty_program();

    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::Bool),
            value: Some(node!(Expression::BooleanNegation(Box::new(node!(Expression::Literal(Literal::I64(1))))))),
        },
    }));

    assert!(!run_check(&program).is_empty());
}

#[test]
fn arithmetic_negation_accepts_numeric_value() {
    let mut program = empty_program();

    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::I64),
            value: Some(node!(Expression::ArithmeticNegation(Box::new(node!(Expression::Literal(Literal::I64(
                1
            ))))))),
        },
    }));

    assert_one_unused_warning(&run_check(&program));
}

#[test]
fn equal_expression_can_be_used_as_bool_condition() {
    let mut program = empty_program();

    program.statements.push(node!(Statement::Conditional {
        condition: node!(Expression::Equal(
            Box::new(node!(Expression::Literal(Literal::I64(1)))),
            Box::new(node!(Expression::Literal(Literal::I64(1)))),
        )),
        if_block: node!(Block(vec![])),
        else_block: Some(node!(Block(vec![]))),
    }));

    assert!(run_check(&program).is_empty());
}
