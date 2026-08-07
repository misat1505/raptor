use std::{collections::HashMap, fmt::Debug};

use crate::{
    ast::Type,
    errors::{ErrorSeverity, ScopeManagerError, StackOverflowError},
};

#[derive(Debug, Clone)]
pub struct StaticCheckerScopeManager<'a> {
    // always has at least 1 scope
    scopes: Vec<StaticCheckerScope<'a>>,
}

impl<'a> StaticCheckerScopeManager<'a> {
    pub fn new() -> Self {
        let root_scope = StaticCheckerScope::new();
        StaticCheckerScopeManager { scopes: vec![root_scope] }
    }

    pub fn push_scope(&mut self) {
        let new_scope = StaticCheckerScope::new();
        self.scopes.push(new_scope);
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn get_variable(&self, searched: &'a str) -> Result<&Type, ScopeManagerError> {
        for scope in &self.scopes {
            if let Some(var) = scope.get_variable(searched) {
                return Ok(var);
            }
        }

        Err(ScopeManagerError::new(
            ErrorSeverity::HIGH,
            format!("Variable '{}' not declared in this scope.", searched),
        ))
    }

    pub fn assign_variable(&mut self, name: &'a str, value: Type) -> Result<(), ScopeManagerError> {
        for scope in &mut self.scopes {
            if let Some(_) = scope.get_variable(name) {
                return scope.assign_variable(name, value);
            }
        }

        Err(ScopeManagerError::new(
            ErrorSeverity::HIGH,
            format!("Variable '{}' not declared in this scope.", name),
        ))
    }

    pub fn declare_variable(&mut self, name: &'a str, value: Type) -> Result<(), ScopeManagerError> {
        if self.get_variable(name).is_ok() {
            return Err(ScopeManagerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot redeclare variable '{}'.", name),
            ));
        }

        if let Some(last_scope) = self.scopes.last_mut() {
            let _ = last_scope.declare_variable(name, value);
            Ok(())
        } else {
            Err(ScopeManagerError::new(
                ErrorSeverity::HIGH,
                String::from("No scope available to set the variable."),
            ))
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> u32 {
        self.scopes.len() as u32
    }
}

#[derive(Debug, Clone)]
pub struct StaticCheckerScope<'a> {
    variables: HashMap<&'a str, Type>,
}

impl<'a> StaticCheckerScope<'a> {
    fn new() -> Self {
        StaticCheckerScope { variables: HashMap::new() }
    }

    fn get_variable(&self, searched: &'a str) -> Option<&Type> {
        self.variables.get(searched)
    }

    fn assign_variable(&mut self, name: &'a str, value: Type) -> Result<(), ScopeManagerError> {
        let current_value_option = self.get_variable(name);
        match current_value_option {
            None => Err(ScopeManagerError::new(ErrorSeverity::HIGH, format!("Variable '{}' not declared.", name))),

            Some(prev_val) => {
                if *prev_val == value {
                    Ok(())
                } else {
                    Err(ScopeManagerError::new(
                        ErrorSeverity::HIGH,
                        format!(
                            "Cannot assign '{:?}' to variable '{}' which was previously declared as '{:?}'.",
                            value, name, prev_val
                        ),
                    ))
                }
            }
        }
    }

    fn declare_variable(&mut self, name: &'a str, value: Type) -> Result<(), ScopeManagerError> {
        match self.get_variable(name) {
            Some(_) => Err(ScopeManagerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot redeclare variable '{}'.", name),
            )),
            None => {
                self.variables.insert(name, value);
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct StaticCheckerStack<'a>(pub Vec<StaticCheckerStackFrame<'a>>);

#[derive(Clone)]
pub struct StaticCheckerStackFrame<'a> {
    pub scope_manager: StaticCheckerScopeManager<'a>,
}

impl<'a> Debug for StaticCheckerStackFrame<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(f, "{:?}", self.scope_manager)?)
    }
}

impl<'a> StaticCheckerStackFrame<'a> {
    pub fn new() -> Self {
        StaticCheckerStackFrame {
            scope_manager: StaticCheckerScopeManager::new(),
        }
    }
}

impl<'a> StaticCheckerStack<'a> {
    pub fn new() -> Self {
        StaticCheckerStack(vec![StaticCheckerStackFrame::new()])
    }

    pub fn push_stack_frame(&mut self) -> Result<(), StackOverflowError> {
        if self.0.len() == 500 {
            return Err(StackOverflowError::new(ErrorSeverity::HIGH, String::from("Stack overflow.")));
        }
        self.0.push(StaticCheckerStackFrame::new());
        Ok(())
    }

    pub fn pop_stack_frame(&mut self) {
        self.0.pop();
    }

    pub fn push_scope(&mut self) {
        if let Some(last_frame) = self.0.last_mut() {
            last_frame.scope_manager.push_scope();
        }
    }

    pub fn pop_scope(&mut self) {
        if let Some(last_frame) = self.0.last_mut() {
            last_frame.scope_manager.pop_scope();
        }
    }

    pub fn get_variable(&mut self, name: &'a str) -> Result<&Type, ScopeManagerError> {
        match self.0.last_mut() {
            Some(last_frame) => last_frame.scope_manager.get_variable(name),
            None => unreachable!("Scope stack is empty"),
        }
    }

    pub fn assign_variable(&mut self, name: &'a str, value: Type) -> Result<(), ScopeManagerError> {
        if let Some(last_frame) = self.0.last_mut() {
            last_frame.scope_manager.assign_variable(name, value)?;
        }
        Ok(())
    }

    pub fn declare_variable(&mut self, name: &'a str, value: Type) -> Result<(), ScopeManagerError> {
        if let Some(last_frame) = self.0.last_mut() {
            last_frame.scope_manager.declare_variable(name, value)?;
        }
        Ok(())
    }
}
