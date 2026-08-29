use super::common::{empty_program, node, run_check};
use crate::{
    common::types::Type,
    frontend::ast::{Argument, Block, Expression, Literal, Parameter, PassedBy, Program, Statement, VariableDeclarationKind},
    semantic::semantic_checker::tests::common::make_function,
};
use std::{collections::HashMap, vec};

#[test]
fn function_call_with_correct_arg_types_has_no_errors() {
    let mut functions = HashMap::new();
    let (name, func) = make_function(
        "add",
        vec![
            node!(Parameter {
                passed_by: PassedBy::Value,
                parameter_type: node!(Type::I64),
                identifier: node!(String::from("a")),
            }),
            node!(Parameter {
                passed_by: PassedBy::Value,
                parameter_type: node!(Type::I64),
                identifier: node!(String::from("b")),
            }),
        ],
        Type::I64,
        Block(vec![node!(Statement::Return(Some(node!(Expression::Addition(
            Box::new(node!(Expression::Variable(String::from("a")))),
            Box::new(node!(Expression::Variable(String::from("b")))),
        )))))]),
    );
    functions.insert(name, func);
    let mut program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    program.statements.push(node!(Statement::FunctionCall {
        identifier: node!(String::from("add")),
        arguments: vec![
            Box::new(node!(Argument {
                value: node!(Expression::Literal(Literal::I64(1))),
                passed_by: PassedBy::Value,
            })),
            Box::new(node!(Argument {
                value: node!(Expression::Literal(Literal::I64(2))),
                passed_by: PassedBy::Value,
            })),
        ],
    }));
    assert!(run_check(&program).is_empty());
}

#[test]
fn function_call_wrong_arg_count_reports_error() {
    let mut functions = HashMap::new();
    let (name, func) = make_function(
        "add",
        vec![node!(Parameter {
            passed_by: PassedBy::Value,
            parameter_type: node!(Type::I64),
            identifier: node!(String::from("a")),
        })],
        Type::I64,
        Block(vec![node!(Statement::Return(Some(node!(Expression::Variable(String::from("a"))))))]),
    );
    functions.insert(name, func);
    let mut program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    program.statements.push(node!(Statement::FunctionCall {
        identifier: node!(String::from("add")),
        arguments: vec![],
    }));
    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("Invalid number of arguments")));
}

#[test]
fn function_call_wrong_arg_type_reports_error() {
    let mut functions = HashMap::new();
    let (name, func) = make_function(
        "takes_i64",
        vec![node!(Parameter {
            passed_by: PassedBy::Value,
            parameter_type: node!(Type::I64),
            identifier: node!(String::from("a")),
        })],
        Type::Void,
        Block(vec![]),
    );
    functions.insert(name, func);
    let mut program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    program.statements.push(node!(Statement::FunctionCall {
        identifier: node!(String::from("takes_i64")),
        arguments: vec![Box::new(node!(Argument {
            value: node!(Expression::Literal(Literal::True)),
            passed_by: PassedBy::Value,
        }))],
    }));
    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("has an incompatible type")));
}

#[test]
fn function_call_reference_with_non_variable_reports_error() {
    let mut functions = HashMap::new();
    let (name, func) = make_function(
        "takes_ref",
        vec![node!(Parameter {
            passed_by: PassedBy::Reference,
            parameter_type: node!(Type::I64),
            identifier: node!(String::from("a")),
        })],
        Type::Void,
        Block(vec![]),
    );
    functions.insert(name, func);
    let mut program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    program.statements.push(node!(Statement::FunctionCall {
        identifier: node!(String::from("takes_ref")),
        arguments: vec![Box::new(node!(Argument {
            value: node!(Expression::Addition(
                Box::new(node!(Expression::Literal(Literal::I64(1)))),
                Box::new(node!(Expression::Literal(Literal::I64(2)))),
            )),
            passed_by: PassedBy::Reference,
        }))],
    }));
    let errors = run_check(&program);
    assert!(errors
        .iter()
        .any(|e| e.contains("must be a variable, index, or field access when passed by reference")));
}

#[test]
fn function_call_reference_with_index_expression_is_valid() {
    let mut functions = HashMap::new();
    let (name, func) = make_function(
        "takes_ref",
        vec![node!(Parameter {
            passed_by: PassedBy::Reference,
            parameter_type: node!(Type::I64),
            identifier: node!(String::from("a")),
        })],
        Type::Void,
        Block(vec![]),
    );
    functions.insert(name, func);
    let mut program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("arr")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::Vector(Box::new(Type::I64))),
            value: Some(node!(Expression::Vector(vec![Box::new(node!(Expression::Literal(Literal::I64(1))))]))),
        },
    }));
    program.statements.push(node!(Statement::FunctionCall {
        identifier: node!(String::from("takes_ref")),
        arguments: vec![Box::new(node!(Argument {
            value: node!(Expression::Index {
                collection: Box::new(node!(Expression::Variable(String::from("arr")))),
                index: Box::new(node!(Expression::Literal(Literal::I64(0)))),
            }),
            passed_by: PassedBy::Reference,
        }))],
    }));
    assert!(run_check(&program).is_empty());
}

#[test]
fn undeclared_function_call_reports_error() {
    let mut program = empty_program();
    program.statements.push(node!(Statement::FunctionCall {
        identifier: node!(String::from("nonexistent")),
        arguments: vec![],
    }));
    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("Use of undeclared function `nonexistent`")));
}

#[test]
fn function_with_correct_return_type_has_no_errors() {
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
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    assert!(run_check(&program).is_empty());
}

#[test]
fn function_with_bad_return_type_reports_error() {
    let mut functions = HashMap::new();
    let (name, func) = make_function(
        "should_return_void",
        vec![],
        Type::Void,
        Block(vec![node!(Statement::Return(Some(node!(Expression::Literal(Literal::I64(5))))))]),
    );
    functions.insert(name, func);
    let program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("must use `return;` without a value")));
}

#[test]
fn void_function_without_return_has_no_errors() {
    let mut functions = HashMap::new();
    let (name, func) = make_function("do_nothing", vec![], Type::Void, Block(vec![]));
    functions.insert(name, func);
    let program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    assert!(run_check(&program).is_empty());
}

#[test]
fn function_parameters_are_declared_in_scope() {
    let mut functions = HashMap::new();
    let (name, func) = make_function(
        "identity",
        vec![node!(Parameter {
            passed_by: PassedBy::Value,
            parameter_type: node!(Type::I64),
            identifier: node!(String::from("x")),
        })],
        Type::I64,
        Block(vec![node!(Statement::Return(Some(node!(Expression::Variable(String::from("x"))))))]),
    );
    functions.insert(name, func);
    let program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    assert!(run_check(&program).is_empty());
}

#[test]
fn function_call_expression_produces_return_type() {
    let mut functions = HashMap::new();
    let (name, func) = make_function("get_value", vec![], Type::I64, Block(vec![]));
    functions.insert(name, func);
    let mut program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::I64),
            value: Some(node!(Expression::FunctionCall {
                identifier: node!(String::from("get_value")),
                arguments: vec![],
            })),
        },
    }));
    assert!(run_check(&program).is_empty());
}

#[test]
fn function_call_with_wrong_passed_by_mode_reports_error() {
    let mut functions = HashMap::new();
    let (name, func) = make_function(
        "takes_ref",
        vec![node!(Parameter {
            passed_by: PassedBy::Reference,
            parameter_type: node!(Type::I64),
            identifier: node!(String::from("a")),
        })],
        Type::Void,
        Block(vec![]),
    );
    functions.insert(name, func);
    let mut program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::I64),
            value: Some(node!(Expression::Literal(Literal::I64(1)))),
        },
    }));
    program.statements.push(node!(Statement::FunctionCall {
        identifier: node!(String::from("takes_ref")),
        arguments: vec![Box::new(node!(Argument {
            value: node!(Expression::Variable(String::from("x"))),
            passed_by: PassedBy::Value,
        }))],
    }));
    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("expects to be passed by")));
}

#[test]
fn reference_parameter_with_correct_variable_is_valid() {
    let mut functions = HashMap::new();
    let (name, func) = make_function(
        "takes_ref",
        vec![node!(Parameter {
            passed_by: PassedBy::Reference,
            parameter_type: node!(Type::I64),
            identifier: node!(String::from("a")),
        })],
        Type::Void,
        Block(vec![]),
    );
    functions.insert(name, func);
    let mut program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    program.statements.push(node!(Statement::Declaration {
        identifier: node!(String::from("x")),
        kind: VariableDeclarationKind::TYPE {
            var_type: node!(Type::I64),
            value: Some(node!(Expression::Literal(Literal::I64(1)))),
        },
    }));
    program.statements.push(node!(Statement::FunctionCall {
        identifier: node!(String::from("takes_ref")),
        arguments: vec![Box::new(node!(Argument {
            value: node!(Expression::Variable(String::from("x"))),
            passed_by: PassedBy::Reference,
        }))],
    }));
    assert!(run_check(&program).is_empty());
}

#[test]
fn missing_return_value_for_non_void_function_reports_error() {
    let mut functions = HashMap::new();
    let (name, func) = make_function("must_return", vec![], Type::I64, Block(vec![node!(Statement::Return(None))]));
    functions.insert(name, func);
    let program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    let errors = run_check(&program);
    assert!(errors.iter().any(|e| e.contains("does not match the declared return type")));
}

#[test]
fn void_function_return_without_value_is_valid() {
    let mut functions = HashMap::new();
    let (name, func) = make_function("return_void", vec![], Type::Void, Block(vec![node!(Statement::Return(None))]));
    functions.insert(name, func);
    let program = Program {
        statements: vec![],
        functions,
        std_functions: HashMap::new(),
        extern_functions: HashMap::new(),
        declared_types: HashMap::new(),
        types: HashMap::new(),
    };
    assert!(run_check(&program).is_empty());
}
