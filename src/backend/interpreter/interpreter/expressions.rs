use std::{cell::RefCell, rc::Rc};

use crate::{
    backend::interpreter::{
        alu::{value::Value, ALU},
        interpreter::Interpreter,
    },
    common::{
        errors::{ComputationError, ErrorSeverity, IError, InterpreterError},
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::{Expression, Literal, Node},
};

impl<'a> Interpreter<'a> {
    pub(in crate::backend::interpreter::interpreter) fn eval_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        match &expression.value {
            Expression::Casting { value, to_type } => {
                self.visit_expression(value)?;

                let computed_value = self.read_last_result()?;

                let value = ALU::cast_to_type(computed_value, &to_type.value, self.span)
                    .map_err(|err| Box::new(ComputationError::at(ErrorSeverity::HIGH, err.message(), self.span)) as Box<dyn IError>)?;

                self.last_result = Some(value);
            }

            Expression::BooleanNegation(value) => self.evaluate_unary_op(value, ALU::boolean_negate)?,
            Expression::ArithmeticNegation(value) => self.evaluate_unary_op(value, ALU::arithmetic_negate)?,
            Expression::Addition(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::add)?,
            Expression::Subtraction(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::subtract)?,
            Expression::Multiplication(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::multiplication)?,
            Expression::Division(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::division)?,
            Expression::Modulo(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::modulo)?,
            Expression::Alternative(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::alternative)?,
            Expression::Concatenation(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::concatenation)?,
            Expression::Greater(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::greater)?,
            Expression::GreaterEqual(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::greater_or_equal)?,
            Expression::Less(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::less)?,
            Expression::LessEqual(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::less_or_equal)?,
            Expression::Equal(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::equal)?,
            Expression::NotEqual(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::not_equal)?,
            Expression::Literal(literal) => self.visit_literal(literal)?,
            Expression::Vector(vector) => self.visit_vector_literal(vector)?,
            Expression::Variable(variable) => self.visit_variable(variable, expression.span)?,
            Expression::FunctionCall { identifier, arguments } => self.call_function(identifier, arguments, expression.span)?,
            Expression::Index { collection, index } => self.eval_index(collection, index)?,
        }

        Ok(())
    }

    pub(in crate::backend::interpreter::interpreter) fn eval_literal(&mut self, literal: &Literal) -> Result<(), Box<dyn IError>> {
        let value = match literal {
            Literal::F64(value) => Value::F64(*value),
            Literal::I64(value) => Value::I64(*value),
            Literal::String(value) => Value::String(value.to_string()),
            Literal::False => Value::Bool(false),
            Literal::True => Value::Bool(true),
        };

        self.last_result = Some(value);

        Ok(())
    }

    pub(in crate::backend::interpreter::interpreter) fn eval_vector_literal(
        &mut self,
        expressions: &'a Vec<Box<Node<Expression>>>,
    ) -> Result<(), Box<dyn IError>> {
        let values = Rc::new(RefCell::new(Vec::new()));

        for expression in expressions {
            self.visit_expression(expression)?;

            values.borrow_mut().push(Rc::new(RefCell::new(self.read_last_result()?)));
        }

        let kind = if let Some(first) = values.borrow().first() {
            Box::new(Type::Vector(Box::new(first.borrow().to_type())))
        } else {
            Box::new(Type::Void)
        };

        self.last_result = Some(Value::Vector { kind, values });

        Ok(())
    }

    pub(in crate::backend::interpreter::interpreter) fn eval_variable(
        &mut self,
        variable: &'a String,
        span: crate::common::span::Span,
    ) -> Result<(), Box<dyn IError>> {
        let value = self
            .stack
            .get_variable(variable.as_str(), span)
            .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), span)) as Box<dyn IError>)?;

        self.last_result = Some(value.borrow().clone());

        Ok(())
    }

    pub(in crate::backend::interpreter::interpreter) fn eval_index(
        &mut self,
        collection: &'a Node<Expression>,
        index: &'a Node<Expression>,
    ) -> Result<(), Box<dyn IError>> {
        self.visit_expression(collection)?;

        let collection_value = self.read_last_result()?;

        let values = match collection_value {
            Value::Vector { values, .. } => values,

            other => {
                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    String::from("Cannot index into this value."),
                    String::from("Vector"),
                    format!("{:?}", other.to_type()),
                    collection.span,
                )));
            }
        };

        self.visit_expression(index)?;

        let index_value = self.read_last_result()?;

        let idx = match index_value {
            Value::I64(i) if i >= 0 => i as usize,

            other => {
                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    String::from("Array index must be a non-negative i64."),
                    String::from("I64"),
                    format!("{:?}", other.to_type()),
                    index.span,
                )));
            }
        };

        let borrowed = values.borrow();

        let element_cell = borrowed.get(idx).ok_or_else(|| {
            Box::new(InterpreterError::at(
                ErrorSeverity::HIGH,
                format!("Index {} out of bounds.", idx),
                index.span,
            )) as Box<dyn IError>
        })?;

        self.last_result = Some(element_cell.borrow().clone());

        Ok(())
    }

    pub(in crate::backend::interpreter::interpreter) fn expect_index(&mut self) -> Result<usize, Box<dyn IError>> {
        match self.read_last_result()? {
            Value::I64(i) if i >= 0 => Ok(i as usize),

            other => Err(Box::new(InterpreterError::expected_found(
                ErrorSeverity::HIGH,
                String::from("Array index must be a non-negative i64."),
                String::from("I64"),
                format!("{:?}", other.to_type()),
                self.span,
            ))),
        }
    }

    fn evaluate_binary_op<F>(&mut self, lhs: &'a Box<Node<Expression>>, rhs: &'a Box<Node<Expression>>, op: F) -> Result<(), Box<dyn IError>>
    where
        F: Fn(Value, Value, crate::common::span::Span) -> Result<Value, ComputationError>,
    {
        self.visit_expression(lhs)?;
        let left_value = self.read_last_result()?;

        self.visit_expression(rhs)?;
        let right_value = self.read_last_result()?;

        let value = op(left_value, right_value, self.span)
            .map_err(|err| Box::new(ComputationError::at(ErrorSeverity::HIGH, err.message(), self.span)) as Box<dyn IError>)?;

        self.last_result = Some(value);

        Ok(())
    }

    fn evaluate_unary_op<F>(&mut self, value: &'a Box<Node<Expression>>, op: F) -> Result<(), Box<dyn IError>>
    where
        F: Fn(Value, crate::common::span::Span) -> Result<Value, ComputationError>,
    {
        self.visit_expression(value)?;
        let computed_value = self.read_last_result()?;

        let value = op(computed_value, self.span)
            .map_err(|err| Box::new(ComputationError::at(ErrorSeverity::HIGH, err.message(), self.span)) as Box<dyn IError>)?;

        self.last_result = Some(value);

        Ok(())
    }
}
