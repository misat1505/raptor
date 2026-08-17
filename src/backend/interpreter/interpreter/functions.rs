use std::{cell::RefCell, rc::Rc};

use crate::{
    backend::{
        interpreter::{
            interpreter::{AbortState, Interpreter},
            Value,
        },
        std_functions::std_functions::StdFunction,
        type_utils::type_accepts_value,
    },
    common::{
        errors::{ErrorSeverity, IError, InterpreterError},
        position::Position,
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::{Argument, FunctionDeclaration, Node, PassedBy},
};

impl<'a> Interpreter<'a> {
    pub(in crate::backend::interpreter::interpreter) fn execute_std_function(
        std_function: &StdFunction,
        arguments: &Vec<Rc<RefCell<Value>>>,
        position: Position,
    ) -> Result<Option<Value>, Box<dyn IError>> {
        (std_function.execute)(arguments)
            .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), position)) as Box<dyn IError>)
    }

    pub(in crate::backend::interpreter::interpreter) fn call_function(
        &mut self,
        identifier: &Node<String>,
        arguments: &'a Vec<Box<Node<Argument>>>,
    ) -> Result<(), Box<dyn IError>> {
        let name = identifier.value.as_str();

        let mut args: Vec<Rc<RefCell<Value>>> = vec![];

        for arg in arguments {
            self.position = arg.position;

            match arg.value.passed_by {
                PassedBy::Value => {
                    self.visit_expression(&arg.value.value)?;

                    let value = self.read_last_result()?;

                    match value {
                        Value::Vector { ref kind, ref values } => {
                            let shallow_copy = Rc::new(RefCell::new(values.borrow().iter().map(Rc::clone).collect::<Vec<_>>()));
                            let shallow_copy_vector = Value::Vector {
                                kind: kind.clone(),
                                values: shallow_copy,
                            };
                            args.push(Rc::new(RefCell::new(shallow_copy_vector)));
                        }
                        _ => args.push(Rc::new(RefCell::new(value))),
                    }
                }

                PassedBy::Reference => {
                    let reference = self.resolve_reference(&arg.value.value)?;
                    args.push(reference);
                }
            }
        }

        self.last_arguments = args;

        if let Some(std_function) = self.program.std_functions.get(name) {
            if arguments.len() != std_function.params.len() {
                self.last_arguments.clear();

                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    format!("invalid number of arguments for function `{}`", name),
                    std_function.params.len().to_string(),
                    arguments.len().to_string(),
                    identifier.position,
                )));
            }

            if let Some(return_value) = Self::execute_std_function(std_function, &self.last_arguments, identifier.position)? {
                self.last_result = Some(return_value);
            }

            self.last_arguments.clear();

            return Ok(());
        }

        if let Some(function_declaration) = self.program.functions.get(name) {
            let expected_arguments = function_declaration.value.parameters.len();

            if arguments.len() != expected_arguments {
                self.last_arguments.clear();

                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    format!("invalid number of arguments for function `{}`", name),
                    expected_arguments.to_string(),
                    arguments.len().to_string(),
                    identifier.position,
                )));
            }

            self.execute_function(&function_declaration.value)?;

            if let Some(AbortState::Return) = self.abort_state {
                self.abort_state = None;
            }

            self.last_arguments.clear();

            return Ok(());
        }

        self.last_arguments.clear();

        Err(Box::new(InterpreterError::at(
            ErrorSeverity::HIGH,
            format!("use of undeclared function `{}`", name),
            identifier.position,
        )))
    }

    pub(in crate::backend::interpreter::interpreter) fn execute_function(
        &mut self,
        function_declaration: &'a FunctionDeclaration,
    ) -> Result<(), Box<dyn IError>> {
        let name = function_declaration.identifier.value.as_str();

        let statements = &function_declaration.block.value.0;

        self.stack.push_stack_frame().map_err(|err| {
            Box::new(InterpreterError::at(
                ErrorSeverity::HIGH,
                err.message(),
                function_declaration.identifier.position,
            )) as Box<dyn IError>
        })?;

        for idx in 0..self.last_arguments.len() {
            let parameter = function_declaration.parameters.get(idx).ok_or_else(|| {
                Box::new(InterpreterError::at(
                    ErrorSeverity::HIGH,
                    format!("Invalid parameter index {} while calling function '{}'.", idx, name),
                    function_declaration.identifier.position,
                )) as Box<dyn IError>
            })?;

            let desired_type = &parameter.value.parameter_type.value;
            let param_name = &parameter.value.identifier.value;
            let value = &self.last_arguments[idx];

            if !type_accepts_value(desired_type, &value.borrow()) {
                self.stack.pop_stack_frame();

                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    format!("Function '{}' parameter '{}': wrong argument type.", name, param_name),
                    format!("{:?}", desired_type),
                    format!("{:?}", value.borrow().to_type()),
                    parameter.position,
                )));
            }

            self.stack
                .declare_variable(param_name.as_str(), Rc::clone(value))
                .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), parameter.position)) as Box<dyn IError>)?;
        }

        for statement in statements {
            if let Some(AbortState::Return) = self.abort_state {
                self.abort_state = None;
                break;
            }

            self.visit_statement(statement)?;

            if let Some(abort) = &self.abort_state {
                match abort {
                    AbortState::Break => {
                        self.stack.pop_stack_frame();

                        return Err(Box::new(InterpreterError::at(
                            ErrorSeverity::HIGH,
                            String::from("Break called outside 'for' or 'switch'."),
                            statement.position,
                        )));
                    }
                    AbortState::Continue => {
                        self.stack.pop_stack_frame();

                        return Err(Box::new(InterpreterError::at(
                            ErrorSeverity::HIGH,
                            String::from("Continue called outside 'for' or 'while'."),
                            statement.position,
                        )));
                    }
                    _ => {}
                }
            }
        }

        match &self.last_result {
            None if function_declaration.return_type.value == Type::Void => {}

            Some(value) if type_accepts_value(&function_declaration.return_type.value, value) => {}

            result => {
                let result_type = match result {
                    None => Type::Void,
                    Some(value) => value.to_type(),
                };

                self.stack.pop_stack_frame();

                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    format!("Bad return type from function '{}'.", name),
                    format!("{:?}", function_declaration.return_type.value),
                    format!("{:?}", result_type),
                    function_declaration.return_type.position,
                )));
            }
        }

        self.stack.pop_stack_frame();

        Ok(())
    }
}
