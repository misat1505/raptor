use crate::{
    common::{
        errors::{ErrorSeverity, IError, SemanticCheckerError},
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::{Node, Statement, SwitchCase, SwitchExpression, VariableDeclarationKind},
    semantic::semantic_checker::{checker::HoverInfo, SemanticChecker},
};

impl<'a> SemanticChecker<'a> {
    pub(in crate::semantic::semantic_checker) fn check_declaration(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        let Statement::Declaration { identifier, kind } = &statement.value else {
            return Ok(());
        };

        match kind {
            VariableDeclarationKind::TYPE { var_type, value } => {
                let _ = self.visit_type(var_type);

                let resolved_type = match value {
                    Some(value) => {
                        let _ = self.visit_expression(value);

                        match self.read_last_result(value.span) {
                            Ok(actual_type) => {
                                let types_compatible = var_type.value == actual_type
                                    || matches!(
                                        (&var_type.value, &actual_type),
                                        (Type::Vector(_), Type::Vector(inner))
                                            if **inner == Type::Void
                                    );

                                if !types_compatible {
                                    let error = SemanticCheckerError::type_mismatch(
                                        ErrorSeverity::HIGH,
                                        format!("Cannot assign `{:?}` to `{}`.", actual_type, identifier.value),
                                        &var_type.value,
                                        &actual_type,
                                        statement.span,
                                    );

                                    self.errors.push(Box::new(error));
                                }

                                Some(var_type.value.clone())
                            }
                            Err(_) => None,
                        }
                    }
                    None => Some(var_type.value.clone()),
                };

                if let Some(resolved_type) = resolved_type.clone() {
                    if let Err(err) = self.stack.declare_variable(identifier.value.as_str(), resolved_type, statement.span) {
                        self.errors
                            .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), statement.span)));
                    }
                }

                self.hovers.push(HoverInfo {
                    contents: format!("```raptor\n{:?} {}\n```", resolved_type.unwrap_or(Type::Void), identifier.value),
                    span: identifier.span,
                });
            }

            VariableDeclarationKind::LET { var_type, value } => {
                let _ = self.visit_expression(value);

                let resolved_type = self.read_last_result(value.span).ok();

                let final_type = match var_type {
                    Some(var_type) => {
                        if let Some(resolved_type) = &resolved_type {
                            let types_compatible = var_type.value == *resolved_type
                                || matches!(
                                    (&var_type.value, resolved_type),
                                    (Type::Vector(_), Type::Vector(inner))
                                        if **inner == Type::Void
                                );

                            if !types_compatible {
                                let error = SemanticCheckerError::type_mismatch(
                                    ErrorSeverity::HIGH,
                                    format!("Cannot assign `{:?}` to `{}`.", resolved_type, identifier.value),
                                    &var_type.value,
                                    resolved_type,
                                    statement.span,
                                );

                                self.errors.push(Box::new(error));
                            }
                        }

                        if let Err(err) = self
                            .stack
                            .declare_variable(identifier.value.as_str(), var_type.value.clone(), statement.span)
                        {
                            self.errors
                                .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), statement.span)));
                        }

                        var_type.value.clone()
                    }

                    None => match resolved_type {
                        Some(resolved_type) => {
                            if matches!(
                                &resolved_type,
                                Type::Vector(inner) if **inner == Type::Void
                            ) {
                                let error = SemanticCheckerError::at(
                                    ErrorSeverity::HIGH,
                                    format!(
                                        "Cannot infer type of empty vector. Consider adding a type annotation, e.g. `let {}: {:?} = [];`.",
                                        identifier.value,
                                        Type::Vector(Box::new(Type::I64))
                                    ),
                                    statement.span,
                                );

                                self.errors.push(Box::new(error));
                            } else if resolved_type == Type::Void {
                                let error = SemanticCheckerError::at(
                                    ErrorSeverity::HIGH,
                                    format!("Cannot assign `{:?}` to `{}`.", resolved_type, identifier.value),
                                    statement.span,
                                );

                                self.errors.push(Box::new(error));
                            } else if let Err(err) = self
                                .stack
                                .declare_variable(identifier.value.as_str(), resolved_type.clone(), statement.span)
                            {
                                self.errors
                                    .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), statement.span)));
                            }

                            resolved_type
                        }

                        None => Type::Void,
                    },
                };

                self.hovers.push(HoverInfo {
                    contents: format!("```raptor\n{:?} {}\n```", final_type, identifier.value),
                    span: identifier.span,
                });
            }
        }

        Ok(())
    }

    pub(in crate::semantic::semantic_checker) fn check_assignment(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        let Statement::Assignment { indices, value, identifier } = &statement.value else {
            return Ok(());
        };

        if indices.is_empty() {
            self.visit_expression(value)?;

            let value = self.read_last_result(value.span).map_err(|_| {
                let error = SemanticCheckerError::at(
                    ErrorSeverity::HIGH,
                    format!("Cannot assign no value to variable `{}`.", identifier.value),
                    statement.span,
                );

                self.errors.push(Box::new(error.clone()));
                Box::new(error) as Box<dyn IError>
            })?;

            if let Err(err) = self.stack.assign_variable(identifier.value.as_str(), value.clone(), statement.span) {
                self.errors
                    .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), statement.span)));
            }

            self.hovers.push(HoverInfo {
                contents: format!("```raptor\n{:?} {}\n```", value, identifier.value),
                span: identifier.span,
            });
        } else {
            self.check_index_assignment(identifier, indices, value, statement.span);
        }

        Ok(())
    }

    pub(in crate::semantic::semantic_checker) fn check_conditional(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        let Statement::Conditional {
            condition,
            else_block,
            if_block,
        } = &statement.value
        else {
            return Ok(());
        };

        let _ = self.visit_expression(condition);

        if let Ok(resolved_condition) = self.read_last_result(condition.span) {
            if resolved_condition != Type::Bool {
                let error = SemanticCheckerError::type_mismatch(
                    ErrorSeverity::HIGH,
                    String::from("if condition must be `bool`"),
                    &Type::Bool,
                    &resolved_condition,
                    condition.span,
                );

                self.errors.push(Box::new(error));
            }
        }

        let _ = self.visit_block(if_block);

        if let Some(else_blk) = else_block {
            let _ = self.visit_block(else_blk);
        }

        Ok(())
    }

    pub(in crate::semantic::semantic_checker) fn check_while_loop(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        let Statement::WhileLoop { block, condition } = &statement.value else {
            return Ok(());
        };

        let _ = self.visit_expression(condition);

        if let Ok(resolved_condition) = self.read_last_result(condition.span) {
            if resolved_condition != Type::Bool {
                let error = SemanticCheckerError::type_mismatch(
                    ErrorSeverity::HIGH,
                    String::from("while condition must be `bool`"),
                    &Type::Bool,
                    &resolved_condition,
                    condition.span,
                );

                self.errors.push(Box::new(error));
            }
        }

        self.stack.enter_breakable();
        self.stack.enter_continuable();

        let _ = self.visit_block(block);

        self.stack.exit_breakable();
        self.stack.exit_continuable();

        Ok(())
    }

    pub(in crate::semantic::semantic_checker) fn check_for_loop(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        let Statement::ForLoop {
            assignment,
            block,
            condition,
            declaration,
        } = &statement.value
        else {
            return Ok(());
        };

        self.stack.push_scope();

        if let Some(decl) = declaration {
            let _ = self.visit_statement(decl);
        }

        let _ = self.visit_expression(condition);

        if let Ok(resolved_condition) = self.read_last_result(condition.span) {
            if resolved_condition != Type::Bool {
                let error = SemanticCheckerError::type_mismatch(
                    ErrorSeverity::HIGH,
                    String::from("for loop condition must be `bool`"),
                    &Type::Bool,
                    &resolved_condition,
                    condition.span,
                );

                self.errors.push(Box::new(error));
            }
        }

        if let Some(assign) = assignment {
            let _ = self.visit_statement(assign);
        }

        self.stack.enter_breakable();
        self.stack.enter_continuable();

        let _ = self.visit_block(block);

        self.stack.exit_breakable();
        self.stack.exit_continuable();

        self.stack.pop_scope();

        Ok(())
    }

    pub(in crate::semantic::semantic_checker) fn check_switch(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        let Statement::Switch { cases, expressions } = &statement.value else {
            return Ok(());
        };

        for expr in expressions {
            let _ = self.check_switch_expression(expr);
        }

        for case in cases {
            let _ = self.check_switch_case(case);
        }

        Ok(())
    }

    pub(in crate::semantic::semantic_checker) fn check_return(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        let Statement::Return(value) = &statement.value else {
            return Ok(());
        };

        if self.stack.size() <= 1 {
            self.errors.push(Box::new(SemanticCheckerError::at(
                ErrorSeverity::HIGH,
                String::from("return statement is not inside a function"),
                statement.span,
            )));
        }

        let actual_type = match value {
            Some(val) => {
                let _ = self.visit_expression(val);
                self.read_last_result(val.span).ok()
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
                    statement.span,
                )));
            }
        }

        Ok(())
    }

    pub(in crate::semantic::semantic_checker) fn check_break(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        if !self.stack.is_in_breakable() {
            self.errors.push(Box::new(SemanticCheckerError::at(
                ErrorSeverity::HIGH,
                String::from("Break statement is not inside a loop nor inside a switch case."),
                statement.span,
            )));
        }

        Ok(())
    }

    pub(in crate::semantic::semantic_checker) fn check_continue(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        if !self.stack.is_in_continuable() {
            self.errors.push(Box::new(SemanticCheckerError::at(
                ErrorSeverity::HIGH,
                String::from("Continue statement is not inside a loop."),
                statement.span,
            )));
        }

        Ok(())
    }

    pub(in crate::semantic::semantic_checker) fn check_switch_case(&mut self, switch_case: &'a Node<SwitchCase>) -> Result<(), Box<dyn IError>> {
        let _ = self.visit_expression(&switch_case.value.condition);

        if let Ok(resolved_condition) = self.read_last_result(switch_case.value.condition.span) {
            if resolved_condition != Type::Bool {
                let error = SemanticCheckerError::type_mismatch(
                    ErrorSeverity::HIGH,
                    String::from("switch case condition must be `bool`"),
                    &Type::Bool,
                    &resolved_condition,
                    switch_case.value.condition.span,
                );

                self.errors.push(Box::new(error));
            }
        }

        self.stack.enter_breakable();

        let _ = self.visit_block(&switch_case.value.block);

        self.stack.exit_breakable();

        Ok(())
    }

    pub(in crate::semantic::semantic_checker) fn check_switch_expression(
        &mut self,
        switch_expression: &'a Node<SwitchExpression>,
    ) -> Result<(), Box<dyn IError>> {
        let expression = &switch_expression.value.expression;

        let _ = self.visit_expression(expression);

        match self.read_last_result(expression.span) {
            Ok(resolved_type) => match &switch_expression.value.alias {
                None => {}

                Some(id) => {
                    if let Err(err) = self.stack.declare_variable(id.value.as_str(), resolved_type, expression.span) {
                        self.errors.push(Box::new(SemanticCheckerError::at(
                            ErrorSeverity::HIGH,
                            err.message(),
                            switch_expression.span,
                        )));
                    }
                }
            },

            Err(_) => {}
        }

        Ok(())
    }
}
