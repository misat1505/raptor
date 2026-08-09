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
                    self.errors
                        .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), lhs.position)));
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
                    self.errors
                        .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), value.position)));
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
                        self.errors.push(Box::new(SemanticCheckerError::expected_found(
                            ErrorSeverity::HIGH,
                            format!("invalid number of arguments for function `{}`", name),
                            std_function.params.len().to_string(),
                            arguments.len().to_string(),
                            *position,
                        )));
                    }

                    let mut collected_types: Vec<Type> = vec![];

                    for idx in 0..arguments.len() {
                        let argument = &arguments[idx];

                        self.visit_expression(&argument.value.value);
                        let actual_type = self.read_last_result().ok();

                        let expected_passed_by = std_function.passed_by.get(idx).unwrap_or(&PassedBy::Value);

                        if &argument.value.passed_by != expected_passed_by {
                            self.errors.push(Box::new(SemanticCheckerError::expected_found(
                                ErrorSeverity::HIGH,
                                format!("parameter {} in function `{}` passed by the wrong mode", idx, name),
                                format!("{:?}", expected_passed_by),
                                format!("{:?}", argument.value.passed_by),
                                argument.position,
                            )));
                        }

                        if *expected_passed_by == PassedBy::Reference && !Self::is_valid_reference_expression(&argument.value.value.value) {
                            self.errors.push(Box::new(SemanticCheckerError::at(
                                ErrorSeverity::HIGH,
                                format!(
                                    "parameter {} in function `{}` must be a variable or index expression when passed by reference",
                                    idx, name
                                ),
                                argument.position,
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
                                self.errors.push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, msg, *position)));
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
                                        self.errors.push(Box::new(SemanticCheckerError::type_mismatch(
                                            ErrorSeverity::HIGH,
                                            format!("parameter {} in function `{}` has the wrong type", idx, name),
                                            expected,
                                            actual,
                                            arguments[idx].position,
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
                        self.errors.push(Box::new(SemanticCheckerError::expected_found(
                            ErrorSeverity::HIGH,
                            format!("invalid number of arguments for function `{}`", name),
                            parameters.len().to_string(),
                            arguments.len().to_string(),
                            *position,
                        )));
                    }

                    for idx in 0..arguments.len() {
                        let argument = &arguments[idx];

                        self.visit_expression(&argument.value.value);
                        let actual_type = self.read_last_result().ok();

                        if let Some(parameter) = parameters.get(idx) {
                            if argument.value.passed_by != parameter.value.passed_by {
                                self.errors.push(Box::new(SemanticCheckerError::expected_found(
                                    ErrorSeverity::HIGH,
                                    format!(
                                        "parameter `{}` in function `{}` passed by the wrong mode",
                                        parameter.value.identifier.value, name
                                    ),
                                    format!("{:?}", parameter.value.passed_by),
                                    format!("{:?}", argument.value.passed_by),
                                    argument.position,
                                )));
                            }

                            if parameter.value.passed_by == PassedBy::Reference && !Self::is_valid_reference_expression(&argument.value.value.value) {
                                self.errors.push(Box::new(SemanticCheckerError::at(
                                    ErrorSeverity::HIGH,
                                    format!(
                                        "parameter `{}` in function `{}` must be a variable or index expression when passed by reference",
                                        parameter.value.identifier.value, name
                                    ),
                                    argument.position,
                                )));
                            }

                            if let Some(actual) = &actual_type {
                                if parameter.value.parameter_type.value != *actual {
                                    self.errors.push(Box::new(SemanticCheckerError::type_mismatch(
                                        ErrorSeverity::HIGH,
                                        format!(
                                            "parameter `{}` in function `{}` has the wrong type",
                                            parameter.value.identifier.value, name
                                        ),
                                        &parameter.value.parameter_type.value,
                                        actual,
                                        argument.position,
                                    )));
                                }
                            }
                        }
                    }

                    self.last_result = Some(function_declaration.value.return_type.value.clone());
                    return;
                }

                self.errors.push(Box::new(SemanticCheckerError::at(
                    ErrorSeverity::HIGH,
                    format!("Use of undeclared function `{}`", name),
                    *position,
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
                self.errors
                    .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), position)));
                return;
            }
        };

        for index_expr in indices {
            self.visit_expression(index_expr);
            if let Ok(idx_type) = self.read_last_result() {
                if idx_type != Type::I64 {
                    self.errors.push(Box::new(SemanticCheckerError::type_mismatch(
                        ErrorSeverity::HIGH,
                        String::from("array index must be `i64`"),
                        &Type::I64,
                        &idx_type,
                        index_expr.position,
                    )));
                    return;
                }
            }

            match current_type {
                Type::Vector(inner) => current_type = *inner,
                other => {
                    self.errors.push(Box::new(SemanticCheckerError::at(
                        ErrorSeverity::HIGH,
                        format!("Cannot index into value of type `{:?}`", other),
                        position,
                    )));
                    return;
                }
            }
        }

        self.visit_expression(value);
        if let Ok(actual_type) = self.read_last_result() {
            if actual_type != current_type {
                self.errors.push(Box::new(SemanticCheckerError::type_mismatch(
                    ErrorSeverity::HIGH,
                    format!("Cannot assign `{:?}` to array element.", actual_type),
                    &current_type,
                    &actual_type,
                    position,
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
                            self.errors.push(Box::new(SemanticCheckerError::at(
                                ErrorSeverity::HIGH,
                                err.message(),
                                expression.position,
                            )));
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
                self.visit_variable(variable, expression.position)?;
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
                        self.errors.push(Box::new(SemanticCheckerError::at(
                            ErrorSeverity::HIGH,
                            format!("cannot index into value of type `{:?}`", other),
                            expression.position,
                        )));
                        self.last_result = None;
                    }
                    (Ok(_), Ok(other)) => {
                        self.errors.push(Box::new(SemanticCheckerError::type_mismatch(
                            ErrorSeverity::HIGH,
                            String::from("array index must be `i64`"),
                            &Type::I64,
                            &other,
                            expression.position,
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
            Statement::FunctionCall { .. } => self.check_function_call(FunctionCallType::Statement(statement)),
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
                        let error = SemanticCheckerError::type_mismatch(
                            ErrorSeverity::HIGH,
                            format!("Cannot assign `{:?}` to `{}`.", actual_type, identifier.value),
                            &var_type.value,
                            &actual_type,
                            statement.position,
                        );
                        self.errors.push(Box::new(error));
                    }
                    if let Err(err) = self.stack.declare_variable(identifier.value.as_str(), var_type.value.clone()) {
                        self.errors
                            .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), statement.position)));
                    }
                }
            }
            Statement::Assignment { identifier, value, indices } => {
                if indices.is_empty() {
                    self.visit_expression(&value)?;
                    let position = statement.position;
                    let value = self.read_last_result().map_err(|_| {
                        let error = SemanticCheckerError::at(
                            ErrorSeverity::HIGH,
                            format!("Cannot assign no value to variable `{}`.", identifier.value),
                            position,
                        );
                        self.errors.push(Box::new(error.clone()));
                        Box::new(error) as Box<dyn IError>
                    })?;

                    if let Err(err) = self.stack.assign_variable(identifier.value.as_str(), value) {
                        self.errors
                            .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), position)));
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
                        let error = SemanticCheckerError::type_mismatch(
                            ErrorSeverity::HIGH,
                            String::from("if condition must be `bool`"),
                            &Type::Bool,
                            &resolved_condition,
                            condition.position,
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
                        let error = SemanticCheckerError::type_mismatch(
                            ErrorSeverity::HIGH,
                            String::from("for loop condition must be `bool`"),
                            &Type::Bool,
                            &resolved_condition,
                            condition.position,
                        );
                        self.errors.push(Box::new(error));
                    }
                }
                if let Some(assign) = assignment {
                    self.visit_statement(&assign);
                }
                self.stack.enter_breakable();
                self.visit_block(&block);
                self.stack.exit_breakable();
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
                if self.stack.size() <= 1 {
                    self.errors.push(Box::new(SemanticCheckerError::at(
                        ErrorSeverity::HIGH,
                        String::from("return statement is not inside a function"),
                        statement.position,
                    )));
                }

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
                        self.errors.push(Box::new(SemanticCheckerError::type_mismatch(
                            ErrorSeverity::HIGH,
                            String::from("wrong return type"),
                            &expected,
                            &got,
                            statement.position,
                        )));
                    }
                }
            }
            Statement::Break => {
                if !self.stack.is_in_breakable() {
                    self.errors.push(Box::new(SemanticCheckerError::at(
                        ErrorSeverity::HIGH,
                        String::from("break statement is not inside a loop nor inside a switch case"),
                        statement.position,
                    )));
                }
            }
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
                let error = SemanticCheckerError::type_mismatch(
                    ErrorSeverity::HIGH,
                    String::from("switch case condition must be `bool`"),
                    &Type::Bool,
                    &resolved_condition,
                    switch_case.position,
                );
                self.errors.push(Box::new(error));
            }
        }
        self.stack.enter_breakable();
        self.visit_block(&switch_case.value.block);
        self.stack.exit_breakable();
        Ok(())
    }

    fn visit_switch_expression(&mut self, switch_expression: &'a Node<SwitchExpression>) -> Result<(), Box<dyn IError>> {
        self.visit_expression(&switch_expression.value.expression);

        match self.read_last_result() {
            Ok(resolved_type) => match &switch_expression.value.alias {
                None => {}
                Some(id) => {
                    if let Err(err) = self.stack.declare_variable(id.value.as_str(), resolved_type) {
                        self.errors.push(Box::new(SemanticCheckerError::at(
                            ErrorSeverity::HIGH,
                            err.message(),
                            switch_expression.position,
                        )));
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

    use crate::{ast::FunctionDeclaration, lazy_stream_reader::Position};

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
            .any(|e| e.contains("break statement is not inside a loop nor inside a switch case")));
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
        assert!(errors.iter().any(|e| e.contains("use of undeclared function `nonexistent`")));
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
}
