use std::{println, unimplemented};

use crate::{
    ast::{Argument, Block, Expression, Literal, Node, Parameter, PassedBy, Program, Statement, SwitchCase, SwitchExpression, Type},
    errors::{ErrorSeverity, IError, SemanticCheckerError},
    lazy_stream_reader::Position,
    static_checker_stack::StaticCheckerStack,
    type_alu::TypeALU,
    visitor::Visitor,
};

enum FunctionCallType {
    Statement(Node<Statement>),
    Expression(Node<Expression>),
}

pub struct SemanticChecker<'a> {
    program: &'a Program,
    stack: StaticCheckerStack<'a>,
    last_result: Option<Type>,
    position: Position,
    pub errors: Vec<Box<dyn IError>>,
}

impl<'a> SemanticChecker<'a> {
    #![allow(unused_must_use)]
    pub fn new(program: &'a Program) -> Result<Self, Box<dyn IError>> {
        let errors: Vec<Box<dyn IError>> = vec![];
        let stack = StaticCheckerStack::new();
        Ok(Self {
            program,
            errors,
            stack,
            last_result: None,
            position: Position {
                line: 0,
                column: 0,
                offset: 0,
            },
        })
    }

    pub fn check(&mut self) {
        self.visit_program(self.program);
    }

    fn read_last_result(&mut self) -> Result<Type, Box<dyn IError>> {
        match self.last_result.take() {
            Some(t) => Ok(t),
            None => {
                let error = SemanticCheckerError::new(ErrorSeverity::HIGH, String::from("No type produced where it is needed."));
                let error_clone = error.clone();
                // self.errors.push(Box::new(error_clone));
                Err(Box::new(error))
            }
        }
    }

    fn evaluate_binary_op<F>(&mut self, lhs: &'a Box<Node<Expression>>, rhs: &'a Box<Node<Expression>>, op: F) -> Result<(), Box<dyn IError>>
    where
        F: Fn(Type, Type) -> Result<Type, SemanticCheckerError>,
    {
        self.visit_expression(lhs)?;
        let left_value = self.read_last_result();
        self.visit_expression(rhs)?;
        let right_value = self.read_last_result();

        match (left_value, right_value) {
            (Ok(l), Ok(r)) => match op(l, r) {
                Ok(result_type) => self.last_result = Some(result_type),
                Err(err) => {
                    self.errors.push(Box::new(err));
                    self.last_result = None;
                }
            },
            _ => self.last_result = None,
        }

        Ok(())
    }

    fn evaluate_unary_op<F>(&mut self, value: &'a Box<Node<Expression>>, op: F) -> Result<(), Box<dyn IError>>
    where
        F: Fn(Type) -> Result<Type, SemanticCheckerError>,
    {
        self.visit_expression(value)?;
        let computed_type = self.read_last_result();

        match computed_type {
            Ok(t) => match op(t) {
                Ok(result_type) => self.last_result = Some(result_type),
                Err(err) => {
                    self.errors.push(Box::new(err));
                    self.last_result = None;
                }
            },
            Err(_) => self.last_result = None,
        }

        Ok(())
    }

    fn check_function_call(&mut self, function: FunctionCallType) {
        match function {
            FunctionCallType::Statement(Node {
                value: Statement::FunctionCall { identifier, arguments },
                position,
            })
            | FunctionCallType::Expression(Node {
                value: Expression::FunctionCall { identifier, arguments },
                position,
            }) => {
                let name = &identifier.value;

                // std function
                if let Some(std_function) = self.program.std_functions.get(&String::from(name)) {
                    if arguments.len() != std_function.params.len() {
                        self.errors.push(Box::new(SemanticCheckerError::new(
                            ErrorSeverity::HIGH,
                            format!(
                                "Invalid number of arguments for function '{}'. Expected {}, given {}.\nAt {:?}.\n",
                                name,
                                std_function.params.len(),
                                arguments.len(),
                                position
                            ),
                        )));
                    }

                    for idx in 0..std_function.params.len() {
                        if let Some(argument) = arguments.get(idx) {
                            let expected = std_function.passed_by.get(idx).unwrap_or(&PassedBy::Value);

                            if &argument.value.passed_by != expected {
                                self.errors.push(Box::new(SemanticCheckerError::new(
                                    ErrorSeverity::HIGH,
                                    format!(
                                        "Parameter {} in function '{}' passed by {:?} - should be passed by {:?}.\nAt {:?}.\n",
                                        idx, identifier.value, argument.value.passed_by, expected, argument.position
                                    ),
                                )));
                            }

                            if *expected == PassedBy::Reference {
                                if let Expression::Variable(_) = argument.value.value.value {
                                } else {
                                    self.errors.push(Box::new(SemanticCheckerError::new(
                                        ErrorSeverity::HIGH,
                                        format!(
                                            "Parameter {} in function '{}' is passed by reference, but complex expression was found.\nAt {:?}.\n",
                                            idx, identifier.value, argument.position
                                        ),
                                    )));
                                }
                            }
                        }
                    }

                    return;
                }

                // user function
                if let Some(function_declaration) = self.program.functions.get(&String::from(name)) {
                    let parameters = &function_declaration.value.parameters;
                    if arguments.len() != parameters.len() {
                        self.errors.push(Box::new(SemanticCheckerError::new(
                            ErrorSeverity::HIGH,
                            format!(
                                "Invalid number of arguments for function '{}'. Expected {}, given {}.\nAt {:?}.\n",
                                name,
                                parameters.len(),
                                arguments.len(),
                                position
                            ),
                        )))
                    }

                    for idx in 0..parameters.len() {
                        let parameter = parameters.get(idx).unwrap();
                        if let Some(argument) = arguments.get(idx) {
                            if argument.value.passed_by != parameter.value.passed_by {
                                self.errors.push(Box::new(SemanticCheckerError::new(
                                    ErrorSeverity::HIGH,
                                    format!(
                                        "Parameter '{}' in function '{}' passed by {:?} - should be passed by {:?}.\nAt {:?}.\n",
                                        parameter.value.identifier.value,
                                        identifier.value,
                                        argument.value.passed_by,
                                        parameter.value.passed_by,
                                        argument.position
                                    ),
                                )));
                            }

                            if argument.value.passed_by == PassedBy::Reference {
                                if let Expression::Variable(_) = argument.value.value.value {
                                } else {
                                    self.errors.push(Box::new(SemanticCheckerError::new(ErrorSeverity::HIGH, format!(
                                            "Parameter '{}' in function '{}' is passed by {:?}. Thus it needs to an identifier, but a complex expression was found.\nAt {:?}.\n",
                                            parameter.value.identifier.value,
                                            identifier.value,
                                            PassedBy::Reference,
                                            argument.position
                                        ),
                                    )));
                                }
                            }
                        }
                    }

                    return;
                }

                self.errors.push(Box::new(SemanticCheckerError::new(
                    ErrorSeverity::HIGH,
                    format!("Use of undeclared function '{}'.\nAt {:?}.\n", name, position),
                )))
            }
            _ => {}
        }
    }
}

impl<'a> Visitor<'a> for SemanticChecker<'a> {
    #![allow(unused_must_use)]
    fn visit_program(&mut self, program: &'a Program) -> Result<(), Box<dyn IError>> {
        for statement in &program.statements {
            self.visit_statement(&statement);
        }

        for (_, function) in &program.functions {
            self.visit_block(&function.value.block);
        }
        Ok(())
    }

    fn visit_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        match &expression.value {
            Expression::FunctionCall { .. } => {
                self.check_function_call(FunctionCallType::Expression(expression.clone()));
            }
            _ => {}
        }

        match &expression.value {
            Expression::Alternative(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::alternative)?,
            Expression::Concatenation(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::concatenation)?,
            Expression::Greater(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::greater)?,
            Expression::GreaterEqual(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::greater_or_equal)?,
            Expression::Less(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::less)?,
            Expression::LessEqual(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::less_or_equal)?,
            Expression::Equal(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::equal)?,
            Expression::NotEqual(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::not_equal)?,
            Expression::Addition(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::add)?,
            Expression::Subtraction(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::subtract)?,
            Expression::Multiplication(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::multiplication)?,
            Expression::Division(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::division)?,

            Expression::BooleanNegation(value) => self.evaluate_unary_op(value, TypeALU::boolean_negate)?,
            Expression::ArithmeticNegation(value) => self.evaluate_unary_op(value, TypeALU::arithmetic_negate)?,

            Expression::Casting { value, to_type } => {
                self.visit_expression(value)?;
                let from_type = self.read_last_result();

                match from_type {
                    Ok(t) => match TypeALU::cast_to_type(t, &to_type.value) {
                        Ok(result_type) => self.last_result = Some(result_type),
                        Err(err) => {
                            self.errors.push(Box::new(err));
                            self.last_result = None;
                        }
                    },
                    Err(_) => self.last_result = None,
                }
            }

            Expression::Literal(literal) => {
                self.visit_literal(literal)?;
            }
            Expression::Variable(variable) => {
                self.visit_variable(variable)?;
            }
            Expression::FunctionCall { arguments, .. } => {
                for arg in arguments {
                    self.visit_argument(arg)?;
                }
                // TODO: last_result should be set by the called function
                self.last_result = None;
            }
            Expression::Vector(vector) => {
                self.visit_vector_literal(vector)?;
            }
            Expression::Index { collection, index } => {
                self.visit_expression(collection)?;
                let collection_type = self.read_last_result();

                self.visit_expression(index)?;
                let index_type = self.read_last_result();

                match (collection_type, index_type) {
                    (Ok(Type::Vector(inner)), Ok(Type::I64)) => {
                        self.last_result = Some(*inner);
                    }
                    (Ok(other), Ok(Type::I64)) => {
                        self.errors.push(Box::new(SemanticCheckerError::new(
                            ErrorSeverity::HIGH,
                            format!("Cannot index into value of type '{:?}'.\nAt {:?}.\n", other, expression.position),
                        )));
                        self.last_result = None;
                    }
                    (Ok(_), Ok(other)) => {
                        self.errors.push(Box::new(SemanticCheckerError::new(
                            ErrorSeverity::HIGH,
                            format!("Array index must be of type 'i64', got '{:?}'.\nAt {:?}.\n", other, expression.position),
                        )));
                        self.last_result = None;
                    }
                    _ => self.last_result = None,
                }
            }
        }
        Ok(())
    }

    fn visit_statement(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        match &statement.value {
            &Statement::FunctionCall { .. } => {
                self.check_function_call(FunctionCallType::Statement(statement.clone()));
            }
            _ => {}
        }

        match &statement.value {
            Statement::FunctionCall { arguments, .. } => {
                for arg in arguments {
                    self.visit_argument(&arg);
                }
            }
            Statement::Declaration { var_type, value, identifier } => {
                self.visit_type(&var_type);

                let resolved_type = match value {
                    Some(val) => {
                        self.visit_expression(val);
                        match self.read_last_result() {
                            Ok(t) => Some(t),
                            Err(_) => None,
                        }
                    }
                    None => Some(var_type.value.clone()),
                };

                if let Some(actual_type) = resolved_type {
                    if var_type.value != actual_type {
                        let error = SemanticCheckerError::new(
                            ErrorSeverity::HIGH,
                            format!(
                                "Cannot assign value of type '{:?}' to variable '{}' of type '{:?}'.\nAt {:?}.\n",
                                actual_type, identifier.value, var_type.value, statement.position
                            ),
                        );
                        self.errors.push(Box::new(error));
                    } else if let Err(err) = self.stack.declare_variable(identifier.value.as_str(), var_type.value.clone()) {
                        self.errors.push(Box::new(err));
                    }
                }
            }
            Statement::Assignment { identifier, value, indices } => {
                if indices.is_empty() {
                    self.visit_expression(&value)?;
                    let value = self.read_last_result().map_err(|_| {
                        let error =
                            SemanticCheckerError::new(ErrorSeverity::HIGH, format!("Cannot assign no value to variable '{}'.", identifier.value));
                        self.errors.push(Box::new(error.clone()));
                        Box::new(error) as Box<dyn IError>
                    })?;

                    if let Err(err) = self.stack.assign_variable(identifier.value.as_str(), value) {
                        self.errors.push(Box::new(err));
                    }
                } else {
                    unimplemented!("self.visit_index_assignment is not implemented yet");
                    // self.visit_index_assignment(identifier, indices, value)?;
                }
            }
            Statement::Conditional {
                condition,
                if_block,
                else_block,
            } => {
                self.visit_expression(&condition);
                self.visit_block(&if_block);
                if let Some(else_blk) = else_block {
                    self.visit_block(&else_blk);
                }
            }
            Statement::ForLoop {
                declaration,
                condition,
                assignment,
                block,
            } => {
                if let Some(decl) = declaration {
                    self.visit_statement(&decl);
                }
                self.visit_expression(&condition);
                if let Some(assign) = assignment {
                    self.visit_statement(&assign);
                }
                self.visit_block(&block);
            }
            Statement::Switch { expressions, cases } => {
                for expr in expressions {
                    self.visit_switch_expression(&expr);
                }
                for case in cases {
                    self.visit_switch_case(&case);
                }
            }
            Statement::Return(value) => {
                if let Some(val) = value {
                    self.visit_expression(&val);
                }
            }
            Statement::Break => {}
        }
        Ok(())
    }

    fn visit_argument(&mut self, argument: &'a Node<Argument>) -> Result<(), Box<dyn IError>> {
        self.visit_expression(&argument.value.value);
        Ok(())
    }

    fn visit_block(&mut self, block: &'a Node<Block>) -> Result<(), Box<dyn IError>> {
        for statement in &block.value.0 {
            self.visit_statement(statement);
        }
        Ok(())
    }

    fn visit_parameter(&mut self, parameter: &'a Node<Parameter>) -> Result<(), Box<dyn IError>> {
        self.visit_type(&parameter.value.parameter_type);
        Ok(())
    }

    fn visit_switch_case(&mut self, switch_case: &'a Node<SwitchCase>) -> Result<(), Box<dyn IError>> {
        self.visit_expression(&switch_case.value.condition);
        self.visit_block(&switch_case.value.block);
        Ok(())
    }

    fn visit_switch_expression(&mut self, switch_expression: &'a Node<SwitchExpression>) -> Result<(), Box<dyn IError>> {
        self.visit_expression(&switch_expression.value.expression);
        Ok(())
    }

    fn visit_type(&mut self, _node_type: &'a Node<Type>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_literal(&mut self, literal: &'a Literal) -> Result<(), Box<dyn IError>> {
        let t = match literal {
            Literal::F64(_) => Type::F64,
            Literal::I64(_) => Type::I64,
            Literal::String(str) => Type::Str,
            Literal::False => Type::Bool,
            Literal::True => Type::Bool,
        };

        self.last_result = Some(t);
        Ok(())
    }

    fn visit_variable(&mut self, variable: &'a String) -> Result<(), Box<dyn IError>> {
        let value = self.stack.get_variable(variable.as_str()).map_err(|err| {
            self.errors.push(Box::new(err.clone()));
            Box::new(err) as Box<dyn IError>
        })?;
        self.last_result = Some(value.clone());
        Ok(())
    }

    fn visit_vector_literal(&mut self, vector: &'a Vec<Box<Node<Expression>>>) -> Result<(), Box<dyn IError>> {
        for expression in vector {
            self.visit_expression(expression)?;
        }

        Ok(())
    }
}
