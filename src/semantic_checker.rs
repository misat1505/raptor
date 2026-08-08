use std::{unreachable, vec};

use crate::{
    ast::{Argument, Block, Expression, Literal, Node, Parameter, PassedBy, Program, Statement, SwitchCase, SwitchExpression, Type},
    errors::{ErrorSeverity, IError, SemanticCheckerError},
    lazy_stream_reader::Position,
    static_checker_stack::StaticCheckerStack,
    type_alu::TypeALU,
    visitor::Visitor,
};

enum FunctionCallType<'a> {
    Statement(&'a Node<Statement>),
    Expression(&'a Node<Expression>),
}

pub struct SemanticChecker<'a> {
    program: &'a Program,
    stack: StaticCheckerStack<'a>,
    last_result: Option<Type>,
    pub errors: Vec<Box<dyn IError>>,
    current_function_return_type: Option<Type>,
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
            current_function_return_type: None,
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

    fn check_function_call(&mut self, function: FunctionCallType<'a>) {
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
                if let Some(std_function) = self.program.std_functions.get(name) {
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

                    let mut collected_types: Vec<Type> = vec![];

                    for idx in 0..arguments.len() {
                        let argument = &arguments[idx];

                        self.visit_expression(&argument.value.value);
                        let actual_type = self.read_last_result().ok();

                        let expected_passed_by = std_function.passed_by.get(idx).unwrap_or(&PassedBy::Value);

                        if &argument.value.passed_by != expected_passed_by {
                            self.errors.push(Box::new(SemanticCheckerError::new(
                                ErrorSeverity::HIGH,
                                format!(
                                    "Parameter {} in function '{}' passed by {:?} - should be passed by {:?}.\nAt {:?}.\n",
                                    idx, name, argument.value.passed_by, expected_passed_by, argument.position
                                ),
                            )));
                        }

                        if *expected_passed_by == PassedBy::Reference && !Self::is_valid_reference_expression(&argument.value.value.value) {
                            self.errors.push(Box::new(SemanticCheckerError::new(
                                ErrorSeverity::HIGH,
                                format!(
                                    "Parameter {} in function '{}' is passed by reference, but complex expression was found.\nAt {:?}.\n",
                                    idx, name, argument.position
                                ),
                            )));
                        }

                        if let Some(t) = actual_type {
                            collected_types.push(t);
                        }
                    }

                    match &std_function.type_check {
                        Some(check_fn) if collected_types.len() == arguments.len() => match check_fn(&collected_types) {
                            Ok(return_type) => self.last_result = Some(return_type),
                            Err(msg) => {
                                self.errors.push(Box::new(SemanticCheckerError::new(
                                    ErrorSeverity::HIGH,
                                    format!("{}\nAt {:?}.\n", msg, position),
                                )));
                                self.last_result = None;
                            }
                        },
                        Some(_) => {
                            self.last_result = None;
                        }
                        None => {
                            for idx in 0..collected_types.len() {
                                if let Some(expected) = std_function.params.get(idx) {
                                    let actual = &collected_types[idx];
                                    if !expected.is_compatible(actual) {
                                        self.errors.push(Box::new(SemanticCheckerError::new(
                                            ErrorSeverity::HIGH,
                                            format!(
                                                "Parameter {} in function '{}' expected type '{:?}', but got '{:?}'.\nAt {:?}.\n",
                                                idx, name, expected, actual, arguments[idx].position
                                            ),
                                        )));
                                    }
                                }
                            }
                            self.last_result = Some(std_function.return_type.clone());
                        }
                    }

                    return;
                }

                // user function
                if let Some(function_declaration) = self.program.functions.get(name) {
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
                        )));
                    }

                    for idx in 0..arguments.len() {
                        let argument = &arguments[idx];

                        self.visit_expression(&argument.value.value);
                        let actual_type = self.read_last_result().ok();

                        if let Some(parameter) = parameters.get(idx) {
                            if argument.value.passed_by != parameter.value.passed_by {
                                self.errors.push(Box::new(SemanticCheckerError::new(
                                    ErrorSeverity::HIGH,
                                    format!(
                                        "Parameter '{}' in function '{}' passed by {:?} - should be passed by {:?}.\nAt {:?}.\n",
                                        parameter.value.identifier.value,
                                        name,
                                        argument.value.passed_by,
                                        parameter.value.passed_by,
                                        argument.position
                                    ),
                                )));
                            }

                            if parameter.value.passed_by == PassedBy::Reference && !Self::is_valid_reference_expression(&argument.value.value.value) {
                                self.errors.push(Box::new(SemanticCheckerError::new(
                        ErrorSeverity::HIGH,
                        format!(
                            "Parameter '{}' in function '{}' is passed by {:?}. Thus it needs to be an identifier or indexed value, but a complex expression was found.\nAt {:?}.\n",
                            parameter.value.identifier.value, name, PassedBy::Reference, argument.position
                        ),
                    )));
                            }

                            if let Some(actual) = &actual_type {
                                if parameter.value.parameter_type.value != *actual {
                                    self.errors.push(Box::new(SemanticCheckerError::new(
                                        ErrorSeverity::HIGH,
                                        format!(
                                            "Parameter '{}' in function '{}' expected type '{:?}', but got '{:?}'.\nAt {:?}.\n",
                                            parameter.value.identifier.value, name, parameter.value.parameter_type.value, actual, argument.position
                                        ),
                                    )));
                                }
                            }
                        }
                    }

                    self.last_result = Some(function_declaration.value.return_type.value.clone());
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

    fn is_valid_reference_expression(expression: &Expression) -> bool {
        match expression {
            Expression::Variable(_) => true,
            Expression::Index { collection, .. } => Self::is_valid_reference_expression(&collection.value),
            _ => false,
        }
    }

    fn check_index_assignment(
        &mut self,
        identifier: &'a Node<String>,
        indices: &'a Vec<Node<Expression>>,
        value: &'a Node<Expression>,
        position: Position,
    ) {
        let mut current_type = match self.stack.get_variable(identifier.value.as_str()) {
            Ok(t) => t.clone(),
            Err(err) => {
                self.errors.push(Box::new(err));
                return;
            }
        };

        for index_expr in indices {
            self.visit_expression(index_expr);
            if let Ok(idx_type) = self.read_last_result() {
                if idx_type != Type::I64 {
                    self.errors.push(Box::new(SemanticCheckerError::new(
                        ErrorSeverity::HIGH,
                        format!(
                            "Array index must be of type 'i64', got '{:?}'.\nAt {:?}.\n",
                            idx_type, index_expr.position
                        ),
                    )));
                    return;
                }
            }

            match current_type {
                Type::Vector(inner) => current_type = *inner,
                other => {
                    self.errors.push(Box::new(SemanticCheckerError::new(
                        ErrorSeverity::HIGH,
                        format!("Cannot index into value of type '{:?}'.\nAt {:?}.\n", other, position),
                    )));
                    return;
                }
            }
        }

        self.visit_expression(value);
        if let Ok(actual_type) = self.read_last_result() {
            if actual_type != current_type {
                self.errors.push(Box::new(SemanticCheckerError::new(
                    ErrorSeverity::HIGH,
                    format!(
                        "Cannot assign value of type '{:?}' to element of type '{:?}'.\nAt {:?}.\n",
                        actual_type, current_type, position
                    ),
                )));
            }
        }
    }
}

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
                    self.errors.push(Box::new(err));
                }
            }
            self.visit_block(&function.value.block);

            self.current_function_return_type = None;
            self.stack.pop_stack_frame();
        }
        Ok(())
    }

    fn visit_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        match &expression.value {
            Expression::FunctionCall { .. } => {
                self.check_function_call(FunctionCallType::Expression(expression));
                return Ok(());
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
            Expression::FunctionCall { .. } => {
                unreachable!("Function call is handled seperately.");
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
                self.check_function_call(FunctionCallType::Statement(statement));
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
                    let types_compatible = var_type.value == actual_type
                        || matches!(
                            (&var_type.value, &actual_type),
                            (Type::Vector(_), Type::Vector(inner)) if **inner == Type::Void
                        );

                    if !types_compatible {
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
                    self.check_index_assignment(identifier, indices, value, statement.position);
                }
            }
            Statement::Conditional {
                condition,
                if_block,
                else_block,
            } => {
                self.visit_expression(&condition);
                if let Ok(resolved_condition) = self.read_last_result() {
                    if resolved_condition != Type::Bool {
                        let error = SemanticCheckerError::new(
                            ErrorSeverity::HIGH,
                            format!(
                                "Condition in 'if statement' has to evaluate to type '{:?}' - got '{:?}'.\nAt {:?}.\n",
                                Type::Bool,
                                resolved_condition,
                                condition.position
                            ),
                        );
                        self.errors.push(Box::new(error));
                    }
                }
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
                self.stack.push_scope();
                if let Some(decl) = declaration {
                    self.visit_statement(&decl);
                }
                self.visit_expression(&condition);
                if let Ok(resolved_condition) = self.read_last_result() {
                    if resolved_condition != Type::Bool {
                        let error = SemanticCheckerError::new(
                            ErrorSeverity::HIGH,
                            format!(
                                "Condition in 'for loop' has to evaluate to type '{:?}' - got '{:?}'.\nAt {:?}.\n",
                                Type::Bool,
                                resolved_condition,
                                condition.position
                            ),
                        );
                        self.errors.push(Box::new(error));
                    }
                }
                if let Some(assign) = assignment {
                    self.visit_statement(&assign);
                }
                self.visit_block(&block);
                self.stack.pop_scope();
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
                let actual_type = match value {
                    Some(val) => {
                        self.visit_expression(val);
                        self.read_last_result().ok()
                    }
                    None => None,
                };

                if let Some(expected) = self.current_function_return_type.clone() {
                    let is_ok = match (&actual_type, &expected) {
                        (None, Type::Void) => true,
                        (Some(t), exp) => t == exp,
                        (None, _) => false,
                    };

                    if !is_ok {
                        let got = actual_type.unwrap_or(Type::Void);
                        self.errors.push(Box::new(SemanticCheckerError::new(
                            ErrorSeverity::HIGH,
                            format!(
                                "Bad return type from function. Expected '{:?}', but got '{:?}'.\nAt {:?}.\n",
                                expected, got, statement.position
                            ),
                        )));
                    }
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

    fn visit_switch_case(&mut self, switch_case: &'a Node<SwitchCase>) -> Result<(), Box<dyn IError>> {
        self.visit_expression(&switch_case.value.condition);
        if let Ok(resolved_condition) = self.read_last_result() {
            if resolved_condition != Type::Bool {
                let error = SemanticCheckerError::new(
                    ErrorSeverity::HIGH,
                    format!(
                        "Condition in 'switch case' has to evaluate to type '{:?}' - got '{:?}'.\nAt {:?}.\n",
                        Type::Bool,
                        resolved_condition,
                        switch_case.position
                    ),
                );
                self.errors.push(Box::new(error));
            }
        }
        self.visit_block(&switch_case.value.block);
        Ok(())
    }

    fn visit_switch_expression(&mut self, switch_expression: &'a Node<SwitchExpression>) -> Result<(), Box<dyn IError>> {
        self.visit_expression(&switch_expression.value.expression);

        match self.read_last_result() {
            Ok(resolved_type) => match &switch_expression.value.alias {
                None => {}
                Some(id) => {
                    if let Err(err) = self.stack.declare_variable(id.value.as_str(), resolved_type) {
                        self.errors.push(Box::new(err));
                    }
                }
            },
            Err(_) => {}
        }

        Ok(())
    }

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

    fn visit_variable(&mut self, variable: &'a String) -> Result<(), Box<dyn IError>> {
        let value = self.stack.get_variable(variable.as_str()).map_err(|err| {
            self.errors.push(Box::new(err.clone()));
            Box::new(err) as Box<dyn IError>
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
                        self.errors.push(Box::new(SemanticCheckerError::new(
                            ErrorSeverity::HIGH,
                            format!(
                                "Vector elements must have the same type: expected '{:?}', got '{:?}'.\nAt {:?}.\n",
                                expected, t, expression.position
                            ),
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
