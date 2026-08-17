use std::{cell::RefCell, rc::Rc};

use crate::{
    backend::interpreter::{
        alu::value::Value,
        interpreter::{AbortState, Interpreter},
    },
    common::{
        errors::{ErrorSeverity, IError, InterpreterError},
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::{Block, Expression, Node, Program, Statement, SwitchCase, SwitchExpression},
};

impl<'a> Interpreter<'a> {
    pub(in crate::backend::interpreter::interpreter) fn exec_program(&mut self, program: &'a Program) -> Result<(), Box<dyn IError>> {
        for statement in &program.statements {
            self.visit_statement(statement)?;

            if let Some(abort) = &self.abort_state {
                return match abort {
                    AbortState::Break => Err(Box::new(InterpreterError::at(
                        ErrorSeverity::HIGH,
                        String::from("Break called outside 'for' or 'switch'."),
                        self.position,
                    ))),
                    AbortState::Continue => Err(Box::new(InterpreterError::at(
                        ErrorSeverity::HIGH,
                        String::from("Continue called outside 'for' or 'while'."),
                        self.position,
                    ))),
                    AbortState::Return => Err(Box::new(InterpreterError::at(
                        ErrorSeverity::HIGH,
                        String::from("Return called outside a function."),
                        self.position,
                    ))),
                };
            }
        }

        Ok(())
    }

    pub(in crate::backend::interpreter::interpreter) fn exec_statement(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        match &statement.value {
            Statement::FunctionCall { identifier, arguments } => {
                self.call_function(identifier, arguments)?;
            }

            Statement::Declaration { var_type, identifier, value } => {
                self.visit_type(var_type)?;

                let mut computed_value = match value {
                    Some(val) => {
                        self.visit_expression(val)?;

                        self.read_last_result().map_err(|_| {
                            Box::new(InterpreterError::at(
                                ErrorSeverity::HIGH,
                                format!("Cannot declare variable '{}' with no value.", identifier.value),
                                statement.position,
                            )) as Box<dyn IError>
                        })?
                    }

                    None => Value::default_value(&var_type.value)
                        .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), statement.position)) as Box<dyn IError>)?,
                };

                match (&var_type.value, &computed_value) {
                    (Type::I64, Value::I64(_)) | (Type::F64, Value::F64(_)) | (Type::Str, Value::String(_)) | (Type::Bool, Value::Bool(_)) => {}

                    (Type::Vector(declared_inner), Value::Vector { values, .. }) => {
                        for value in values.borrow().iter() {
                            let actual_type = value.borrow().to_type();

                            if actual_type != *declared_inner.as_ref() {
                                return Err(Box::new(InterpreterError::expected_found(
                                    ErrorSeverity::HIGH,
                                    format!("Cannot assign value to vector '{}'.", identifier.value),
                                    format!("{:?}", declared_inner.as_ref()),
                                    format!("{:?}", actual_type),
                                    statement.position,
                                )));
                            }
                        }

                        if let Value::Vector { kind: _, ref values } = computed_value {
                            if values.borrow().is_empty() {
                                computed_value = Value::Vector {
                                    kind: Box::new(var_type.value.clone()),
                                    values: values.clone(),
                                };
                            }
                        }
                    }

                    (declared_type, computed_value) => {
                        return Err(Box::new(InterpreterError::expected_found(
                            ErrorSeverity::HIGH,
                            format!("Cannot assign value to variable '{}'.", identifier.value),
                            format!("{:?}", declared_type),
                            format!("{:?}", computed_value.to_type()),
                            statement.position,
                        )));
                    }
                }

                self.stack
                    .declare_variable(identifier.value.as_str(), Rc::new(RefCell::new(computed_value)))
                    .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), statement.position)) as Box<dyn IError>)?;
            }

            Statement::Assignment { identifier, value, indices } => {
                if indices.is_empty() {
                    self.visit_expression(value)?;

                    let value = self.read_last_result().map_err(|_| {
                        Box::new(InterpreterError::at(
                            ErrorSeverity::HIGH,
                            format!("Cannot assign no value to variable '{}'.", identifier.value),
                            statement.position,
                        )) as Box<dyn IError>
                    })?;

                    self.stack
                        .assign_variable(identifier.value.as_str(), Rc::new(RefCell::new(value)))
                        .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), statement.position)) as Box<dyn IError>)?;
                } else {
                    self.exec_index_assignment(identifier, indices, value)?;
                }
            }

            Statement::Conditional {
                condition,
                if_block,
                else_block,
            } => {
                self.visit_expression(condition)?;

                let computed_condition = self.read_last_result()?;

                let boolean_value = computed_condition
                    .try_into_bool()
                    .map_err(|_| self.condition_error(computed_condition, "if statement"))?;

                if boolean_value {
                    self.visit_block(if_block)?;
                } else if let Some(else_blk) = else_block {
                    self.visit_block(else_blk)?;
                }
            }

            Statement::WhileLoop { condition, block } => {
                self.visit_expression(condition)?;

                let mut computed_condition = self.read_last_result()?;

                let mut boolean_value = computed_condition
                    .try_into_bool()
                    .map_err(|_| self.condition_error(computed_condition, "while statement"))?;

                while boolean_value {
                    self.visit_block(block)?;

                    if let Some(abort) = &self.abort_state {
                        match abort {
                            AbortState::Break => {
                                self.abort_state = None;
                                break;
                            }
                            AbortState::Return => {
                                break;
                            }
                            AbortState::Continue => {
                                self.abort_state = None;
                            }
                        }
                    }

                    self.visit_expression(condition)?;

                    computed_condition = self.read_last_result()?;

                    boolean_value = computed_condition
                        .try_into_bool()
                        .map_err(|_| self.condition_error(computed_condition, "while statement"))?;
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
                    self.visit_statement(decl)?;
                }

                self.visit_expression(condition)?;

                let mut computed_condition = self.read_last_result()?;

                let mut boolean_value = computed_condition
                    .try_into_bool()
                    .map_err(|_| self.condition_error(computed_condition, "for statement"))?;

                while boolean_value {
                    self.visit_block(block)?;

                    if let Some(abort) = &self.abort_state {
                        match abort {
                            AbortState::Break => {
                                self.abort_state = None;
                                break;
                            }
                            AbortState::Return => {
                                break;
                            }
                            AbortState::Continue => {
                                self.abort_state = None;
                            }
                        }
                    }

                    if let Some(assign) = assignment {
                        self.visit_statement(assign)?;
                    }

                    self.visit_expression(condition)?;

                    computed_condition = self.read_last_result()?;

                    boolean_value = computed_condition
                        .try_into_bool()
                        .map_err(|_| self.condition_error(computed_condition, "for statement"))?;
                }

                self.stack.pop_scope();
            }

            Statement::Switch { expressions, cases } => {
                self.stack.push_scope();

                for expr in expressions {
                    self.visit_switch_expression(expr)?;
                }

                for case in cases {
                    self.visit_switch_case(case)?;

                    if let Some(abort) = &self.abort_state {
                        match abort {
                            AbortState::Break => {
                                self.abort_state = None;
                                break;
                            }
                            AbortState::Return => {
                                break;
                            }
                            AbortState::Continue => {
                                break;
                            }
                        }
                    }
                }

                self.stack.pop_scope();
            }

            Statement::Return(value) => {
                let mut returned_value = None;

                if let Some(val) = value {
                    self.visit_expression(val)?;
                    returned_value = Some(self.read_last_result()?);
                }

                self.abort_state = Some(AbortState::Return);
                self.last_result = returned_value;
            }

            Statement::Break => {
                self.abort_state = Some(AbortState::Break);
            }

            Statement::Continue => {
                self.abort_state = Some(AbortState::Continue);
            }
        }

        Ok(())
    }

    pub(in crate::backend::interpreter::interpreter) fn exec_block(&mut self, block: &'a Node<Block>) -> Result<(), Box<dyn IError>> {
        self.stack.push_scope();

        for statement in &block.value.0 {
            if let Some(_) = self.abort_state {
                break;
            }

            self.visit_statement(statement)?;
        }

        self.stack.pop_scope();

        Ok(())
    }

    pub(in crate::backend::interpreter::interpreter) fn exec_switch_case(
        &mut self,
        switch_case: &'a Node<SwitchCase>,
    ) -> Result<(), Box<dyn IError>> {
        self.visit_expression(&switch_case.value.condition)?;

        let computed_value = self.read_last_result()?;

        let boolean_value = computed_value
            .try_into_bool()
            .map_err(|_| self.condition_error(computed_value, "switch case"))?;

        if boolean_value {
            self.visit_block(&switch_case.value.block)?;
        }

        Ok(())
    }

    pub(in crate::backend::interpreter::interpreter) fn exec_switch_expression(
        &mut self,
        switch_expression: &'a Node<SwitchExpression>,
    ) -> Result<(), Box<dyn IError>> {
        if let Some(alias) = &switch_expression.value.alias {
            self.visit_expression(&switch_expression.value.expression)?;

            let computed_value = self.read_last_result()?;

            self.stack
                .declare_variable(alias.value.as_str(), Rc::new(RefCell::new(computed_value)))
                .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), switch_expression.position)) as Box<dyn IError>)?;
        }

        Ok(())
    }

    pub(in crate::backend::interpreter::interpreter) fn condition_error(&self, value: Value, place: &'a str) -> Box<dyn IError> {
        Box::new(InterpreterError::expected_found(
            ErrorSeverity::HIGH,
            format!("Condition in '{}' has to evaluate to a valid boolean.", place),
            format!("{:?}", Type::Bool),
            format!("{:?}", value.to_type()),
            self.position,
        ))
    }

    pub(in crate::backend::interpreter::interpreter) fn exec_index_assignment(
        &mut self,
        identifier: &'a Node<String>,
        indices: &'a Vec<Node<Expression>>,
        value: &'a Node<Expression>,
    ) -> Result<(), Box<dyn IError>> {
        let var_ref = self
            .stack
            .get_variable(identifier.value.as_str())
            .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), identifier.position)) as Box<dyn IError>)?;

        let (last_index_expr, earlier_indices) = indices.split_last().expect("parser guarantees at least one index in IndexAssignment");

        let mut current_values = match &*var_ref.borrow() {
            Value::Vector { values, .. } => values.clone(),

            other => {
                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    format!("Cannot index into variable '{}'.", identifier.value),
                    String::from("Vector"),
                    format!("{:?}", other.to_type()),
                    identifier.position,
                )));
            }
        };

        for index_expr in earlier_indices {
            self.visit_expression(index_expr)?;

            let idx = self.expect_index()?;

            let borrowed = current_values.borrow();

            let next_cell = borrowed.get(idx).ok_or_else(|| {
                Box::new(InterpreterError::at(
                    ErrorSeverity::HIGH,
                    format!("Index {} out of bounds.", idx),
                    index_expr.position,
                )) as Box<dyn IError>
            })?;

            let next_values = match &*next_cell.borrow() {
                Value::Vector { values, .. } => values.clone(),

                other => {
                    return Err(Box::new(InterpreterError::expected_found(
                        ErrorSeverity::HIGH,
                        String::from("Cannot index into this value."),
                        String::from("Vector"),
                        format!("{:?}", other.to_type()),
                        index_expr.position,
                    )));
                }
            };

            drop(borrowed);

            current_values = next_values;
        }

        self.visit_expression(last_index_expr)?;

        let idx = self.expect_index()?;

        self.visit_expression(value)?;

        let new_value = self.read_last_result()?;

        let mut borrowed = current_values.borrow_mut();

        let target_cell = borrowed.get_mut(idx).ok_or_else(|| {
            Box::new(InterpreterError::at(
                ErrorSeverity::HIGH,
                format!("Index {} out of bounds.", idx),
                last_index_expr.position,
            )) as Box<dyn IError>
        })?;

        *target_cell = Rc::new(RefCell::new(new_value));

        Ok(())
    }

    pub(in crate::backend::interpreter::interpreter) fn resolve_reference(
        &mut self,
        expression: &'a Node<Expression>,
    ) -> Result<Rc<RefCell<Value>>, Box<dyn IError>> {
        match &expression.value {
            Expression::Variable(var_name) => self
                .stack
                .get_variable(var_name.as_str())
                .map(Rc::clone)
                .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), expression.position)) as Box<dyn IError>),

            Expression::Index { collection, index } => {
                let collection_ref = self.resolve_reference(collection)?;

                self.visit_expression(index)?;

                let idx = self.expect_index()?;

                let values = match &*collection_ref.borrow() {
                    Value::Vector { values, .. } => values.clone(),

                    other => {
                        return Err(Box::new(InterpreterError::expected_found(
                            ErrorSeverity::HIGH,
                            String::from("Cannot index into this value."),
                            String::from("Vector"),
                            format!("{:?}", other.to_type()),
                            collection.position,
                        )));
                    }
                };

                let borrowed = values.borrow();

                let element_cell = borrowed.get(idx).ok_or_else(|| {
                    Box::new(InterpreterError::at(
                        ErrorSeverity::HIGH,
                        format!("Index {} out of bounds.", idx),
                        index.position,
                    )) as Box<dyn IError>
                })?;

                Ok(Rc::clone(element_cell))
            }

            _ => Err(Box::new(InterpreterError::at(
                ErrorSeverity::HIGH,
                String::from("Cannot pass this kind of expression by reference — expected a variable or indexed value."),
                expression.position,
            ))),
        }
    }
}
