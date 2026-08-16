use crate::{
    common::{
        errors::{ErrorSeverity, IError, SemanticCheckerError},
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::{Expression, Node, PassedBy, Statement},
    semantic::semantic_checker::SemanticChecker,
};

pub(in crate::semantic::semantic_checker) enum FunctionCallType<'a> {
    Statement(&'a Node<Statement>),
    Expression(&'a Node<Expression>),
}

impl<'a> SemanticChecker<'a> {
    pub(in crate::semantic::semantic_checker) fn is_valid_reference_expression(expression: &Expression) -> bool {
        match expression {
            Expression::Variable(_) => true,
            Expression::Index { collection, .. } => Self::is_valid_reference_expression(&collection.value),
            _ => false,
        }
    }

    pub(in crate::semantic::semantic_checker) fn check_function_call(&mut self, function: FunctionCallType<'a>) {
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

                        let _ = self.visit_expression(&argument.value.value);
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

                // extern function
                if let Some(function_declaration) = self.program.extern_functions.get(name) {
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

                        let _ = self.visit_expression(&argument.value.value);
                        let actual_type = self.read_last_result().ok();

                        if let Some(parameter) = parameters.get(idx) {
                            if argument.value.passed_by != parameter.value.passed_by {
                                self.errors.push(Box::new(SemanticCheckerError::expected_found(
                                    ErrorSeverity::HIGH,
                                    format!(
                                        "parameter `{}` in extern function `{}` passed by the wrong mode",
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
                                        "parameter `{}` in extern function `{}` must be a variable or index expression when passed by reference",
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
                                            "parameter `{}` in extern function `{}` has the wrong type",
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

                        let _ = self.visit_expression(&argument.value.value);
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
}
