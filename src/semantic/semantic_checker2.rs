use std::{unreachable, vec};

use crate::{
    common::{
        errors::{ErrorSeverity, IError, SemanticCheckerError},
        position::Position,
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::{Argument, Block, Expression, Literal, Node, Parameter, PassedBy, Program, Statement, SwitchCase, SwitchExpression},
    semantic::{stack::stack::StaticCheckerStack, type_alu::TypeALU},
};

pub struct SemanticChecker<'a> {
    program: &'a Program,
    stack: StaticCheckerStack<'a>,
    last_result: Option<Type>,
    pub errors: Vec<Box<dyn IError>>,
    current_function_return_type: Option<Type>,
}

impl<'a> SemanticChecker<'a> {}

impl<'a> Visitor<'a> for SemanticChecker<'a> {
    #![allow(unused_must_use)]
    fn visit_program(&mut self, program: &'a Program) -> Result<(), Box<dyn IError>> {
        for statement in &program.statements {
            self.visit_statement(&statement);
        }

        for (_name, function) in &program.functions {
            self.stack.push_stack_frame();
            self.current_function_return_type = Some(function.value.return_type.value.clone());

            for param in &function.value.parameters {
                let param_name = &param.value.identifier.value;
                let param_type = &param.value.parameter_type.value;
                if let Err(err) = self.stack.declare_variable(param_name, param_type.clone()) {
                    self.errors
                        .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), function.position)));
                }
            }
            self.visit_block(&function.value.block);

            self.current_function_return_type = None;
            self.stack.pop_stack_frame();
        }
        Ok(())
    }

    fn visit_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {}

    fn visit_statement(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {}

    fn visit_argument(&mut self, argument: &'a Node<Argument>) -> Result<(), Box<dyn IError>> {
        self.visit_expression(&argument.value.value);
        Ok(())
    }

    fn visit_block(&mut self, block: &'a Node<Block>) -> Result<(), Box<dyn IError>> {
        self.stack.push_scope();
        for statement in &block.value.0 {
            self.visit_statement(statement);
        }
        self.stack.pop_scope();
        Ok(())
    }

    fn visit_parameter(&mut self, parameter: &'a Node<Parameter>) -> Result<(), Box<dyn IError>> {
        self.visit_type(&parameter.value.parameter_type);
        Ok(())
    }

    fn visit_switch_case(&mut self, switch_case: &'a Node<SwitchCase>) -> Result<(), Box<dyn IError>> {}

    fn visit_switch_expression(&mut self, switch_expression: &'a Node<SwitchExpression>) -> Result<(), Box<dyn IError>> {}

    fn visit_type(&mut self, _node_type: &'a Node<Type>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_literal(&mut self, literal: &'a Literal) -> Result<(), Box<dyn IError>> {
        let t = match literal {
            Literal::F64(_) => Type::F64,
            Literal::I64(_) => Type::I64,
            Literal::String(_) => Type::Str,
            Literal::False => Type::Bool,
            Literal::True => Type::Bool,
        };

        self.last_result = Some(t);
        Ok(())
    }

    fn visit_variable(&mut self, variable: &'a String, position: Position) -> Result<(), Box<dyn IError>> {
        let value = self.stack.get_variable(variable.as_str()).map_err(|err| {
            let error = SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), position);
            self.errors.push(Box::new(error.clone()));
            Box::new(error.clone()) as Box<dyn IError>
        })?;
        self.last_result = Some(value.clone());
        Ok(())
    }

    fn visit_vector_literal(&mut self, vector: &'a Vec<Box<Node<Expression>>>) -> Result<(), Box<dyn IError>> {
        let mut element_type: Option<Type> = None;

        for expression in vector {
            self.visit_expression(expression)?;
            if let Ok(t) = self.read_last_result() {
                match &element_type {
                    None => element_type = Some(t),
                    Some(expected) if *expected != t => {
                        self.errors.push(Box::new(SemanticCheckerError::type_mismatch(
                            ErrorSeverity::HIGH,
                            String::from("vector elements have mismatched types"),
                            expected,
                            &t,
                            expression.position,
                        )));
                    }
                    _ => {}
                }
            }
        }

        self.last_result = Some(Type::Vector(Box::new(element_type.unwrap_or(Type::Void))));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, rc::Rc};

    use crate::{common::position::Position, frontend::ast::FunctionDeclaration};

    use super::*;

    fn pos() -> Position {
        Position {
            filename: None,
            line: 0,
            column: 0,
            offset: 0,
        }
    }

    macro_rules! node {
        ($value:expr) => {
            Node {
                value: $value,
                position: pos(),
            }
        };
    }

    fn empty_program() -> Program {
        Program {
            statements: vec![],
            functions: HashMap::new(),
            std_functions: HashMap::new(),
            extern_functions: HashMap::new(),
        }
    }

    fn run_check(program: &Program) -> Vec<String> {
        let mut checker = SemanticChecker::new(program).unwrap();
        checker.check();
        checker.errors.iter().map(|e| e.message()).collect()
    }

    #[test]
    fn empty_program_has_no_errors() {
        let program = empty_program();
        assert!(run_check(&program).is_empty());
    }

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
    fn valid_binary_addition_has_no_errors() {
        let mut program = empty_program();
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::Addition(
                Box::new(node!(Expression::Literal(Literal::I64(1)))),
                Box::new(node!(Expression::Literal(Literal::I64(2)))),
            ))),
        }));

        assert!(run_check(&program).is_empty());
    }

    #[test]
    fn invalid_binary_addition_reports_error() {
        let mut program = empty_program();
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::Addition(
                Box::new(node!(Expression::Literal(Literal::I64(1)))),
                Box::new(node!(Expression::Literal(Literal::True))),
            ))),
        }));

        let errors = run_check(&program);
        assert!(errors.iter().any(|e| e.contains("Cannot perform addition")));
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
    fn undeclared_variable_reports_error() {
        let mut program = empty_program();
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("y")),
            value: Some(node!(Expression::Variable(String::from("x")))),
        }));

        let errors = run_check(&program);
        assert!(errors.iter().any(|e| e.contains("not declared")));
    }

    #[test]
    fn index_expression_on_vector_returns_element_type() {
        let mut program = empty_program();
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::Vector(Box::new(Type::I64))),
            identifier: node!(String::from("arr")),
            value: Some(node!(Expression::Vector(vec![Box::new(node!(Expression::Literal(Literal::I64(1))))]))),
        }));
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::Index {
                collection: Box::new(node!(Expression::Variable(String::from("arr")))),
                index: Box::new(node!(Expression::Literal(Literal::I64(0)))),
            })),
        }));

        assert!(run_check(&program).is_empty());
    }

    #[test]
    fn index_expression_on_non_vector_reports_error() {
        let mut program = empty_program();
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::Literal(Literal::I64(1)))),
        }));
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("y")),
            value: Some(node!(Expression::Index {
                collection: Box::new(node!(Expression::Variable(String::from("x")))),
                index: Box::new(node!(Expression::Literal(Literal::I64(0)))),
            })),
        }));

        let errors = run_check(&program);
        assert!(errors.iter().any(|e| e.contains("cannot index into value")));
    }

    #[test]
    fn index_with_non_i64_index_reports_error() {
        let mut program = empty_program();
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::Vector(Box::new(Type::I64))),
            identifier: node!(String::from("arr")),
            value: Some(node!(Expression::Vector(vec![Box::new(node!(Expression::Literal(Literal::I64(1))))]))),
        }));
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("y")),
            value: Some(node!(Expression::Index {
                collection: Box::new(node!(Expression::Variable(String::from("arr")))),
                index: Box::new(node!(Expression::Literal(Literal::True))),
            })),
        }));

        let errors = run_check(&program);
        assert!(errors.iter().any(|e| e.contains("array index must be `i64`")));
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

    fn make_function(name: &str, parameters: Vec<Node<Parameter>>, return_type: Type, block: Block) -> (String, Rc<Node<FunctionDeclaration>>) {
        (
            name.to_string(),
            Rc::new(node!(FunctionDeclaration {
                identifier: node!(String::from(name)),
                parameters,
                return_type: node!(return_type),
                block: node!(block),
            })),
        )
    }

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
        };

        program.statements.push(node!(Statement::FunctionCall {
            identifier: node!(String::from("add")),
            arguments: vec![],
        }));

        let errors = run_check(&program);
        assert!(errors.iter().any(|e| e.contains("invalid number of arguments")));
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
        };

        program.statements.push(node!(Statement::FunctionCall {
            identifier: node!(String::from("takes_i64")),
            arguments: vec![Box::new(node!(Argument {
                value: node!(Expression::Literal(Literal::True)),
                passed_by: PassedBy::Value,
            }))],
        }));

        let errors = run_check(&program);
        assert!(errors.iter().any(|e| e.contains("has the wrong type")));
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
        assert!(errors.iter().any(|e| e.contains("must be a variable or index expression")));
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
        };

        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::Vector(Box::new(Type::I64))),
            identifier: node!(String::from("arr")),
            value: Some(node!(Expression::Vector(vec![Box::new(node!(Expression::Literal(Literal::I64(1))))]))),
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
        };

        let errors = run_check(&program);
        assert!(errors.iter().any(|e| e.contains("wrong return type")));
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
        };

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
        };

        assert!(run_check(&program).is_empty());
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
    fn casting_valid_types_has_no_errors() {
        let mut program = empty_program();
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::F64),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::Casting {
                value: Box::new(node!(Expression::Literal(Literal::I64(5)))),
                to_type: node!(Type::F64),
            })),
        }));

        assert!(run_check(&program).is_empty());
    }

    #[test]
    fn casting_invalid_types_reports_error() {
        let mut program = empty_program();
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::Casting {
                value: Box::new(node!(Expression::Vector(vec![]))),
                to_type: node!(Type::I64),
            })),
        }));

        let errors = run_check(&program);
        assert!(errors.iter().any(|e| e.contains("Cannot cast")));
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
        };
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::FunctionCall {
                identifier: node!(String::from("get_value")),
                arguments: vec![],
            })),
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
        };
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::Literal(Literal::I64(1)))),
        }));
        program.statements.push(node!(Statement::FunctionCall {
            identifier: node!(String::from("takes_ref")),
            arguments: vec![Box::new(node!(Argument {
                value: node!(Expression::Variable(String::from("x"))),
                passed_by: PassedBy::Value,
            }))],
        }));
        let errors = run_check(&program);
        assert!(errors.iter().any(|e| e.contains("passed by the wrong mode")));
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
        };
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::Literal(Literal::I64(1)))),
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
        };
        let errors = run_check(&program);
        assert!(errors.iter().any(|e| e.contains("wrong return type")));
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
        };
        assert!(run_check(&program).is_empty());
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

    #[test]
    fn boolean_negation_accepts_bool() {
        let mut program = empty_program();
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::Bool),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::BooleanNegation(Box::new(node!(Expression::Literal(Literal::True))),))),
        }));
        assert!(run_check(&program).is_empty());
    }

    #[test]
    fn boolean_negation_rejects_non_bool() {
        let mut program = empty_program();
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::Bool),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::BooleanNegation(Box::new(node!(Expression::Literal(Literal::I64(1)))),))),
        }));
        assert!(!run_check(&program).is_empty());
    }

    #[test]
    fn arithmetic_negation_accepts_numeric_value() {
        let mut program = empty_program();
        program.statements.push(node!(Statement::Declaration {
            var_type: node!(Type::I64),
            identifier: node!(String::from("x")),
            value: Some(node!(Expression::ArithmeticNegation(Box::new(node!(Expression::Literal(Literal::I64(
                1
            )))),))),
        }));
        assert!(run_check(&program).is_empty());
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
}
