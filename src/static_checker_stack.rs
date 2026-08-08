use std::{collections::HashMap, fmt::Debug};

use crate::{
    ast::Type,
    errors::{ErrorSeverity, IError, ScopeManagerError, StackOverflowError},
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_manager_declare_and_get() {
        let mut manager = StaticCheckerScopeManager::new();
        assert!(manager.declare_variable("x", Type::I64).is_ok());
        assert_eq!(manager.get_variable("x").unwrap(), &Type::I64);
    }

    #[test]
    fn scope_manager_redeclare_fails() {
        let mut manager = StaticCheckerScopeManager::new();
        let _ = manager.declare_variable("x", Type::I64);
        assert_eq!(
            manager.declare_variable("x", Type::F64).err().unwrap().message(),
            "Cannot redeclare variable 'x'."
        );
    }

    #[test]
    fn scope_manager_get_undeclared_fails() {
        let manager = StaticCheckerScopeManager::new();
        assert_eq!(
            manager.get_variable("x").err().unwrap().message(),
            "Variable 'x' not declared in this scope."
        );
    }

    #[test]
    fn scope_manager_assign_same_type_ok() {
        let mut manager = StaticCheckerScopeManager::new();
        let _ = manager.declare_variable("x", Type::I64);
        assert!(manager.assign_variable("x", Type::I64).is_ok());
    }

    #[test]
    fn scope_manager_assign_different_type_fails() {
        let mut manager = StaticCheckerScopeManager::new();
        let _ = manager.declare_variable("x", Type::I64);
        assert_eq!(
            manager.assign_variable("x", Type::Str).err().unwrap().message(),
            "Cannot assign 'str' to variable 'x' which was previously declared as 'i64'."
        );
    }

    #[test]
    fn scope_manager_assign_undeclared_fails() {
        let mut manager = StaticCheckerScopeManager::new();
        assert_eq!(
            manager.assign_variable("x", Type::I64).err().unwrap().message(),
            "Variable 'x' not declared in this scope."
        );
    }

    #[test]
    fn scope_manager_nested_scope_sees_parent_variable() {
        let mut manager = StaticCheckerScopeManager::new();
        let _ = manager.declare_variable("x", Type::I64);
        manager.push_scope();
        assert_eq!(manager.get_variable("x").unwrap(), &Type::I64);
    }

    #[test]
    fn scope_manager_pop_scope_removes_inner_variable() {
        let mut manager = StaticCheckerScopeManager::new();
        manager.push_scope();
        let _ = manager.declare_variable("y", Type::I64);
        assert!(manager.get_variable("y").is_ok());
        manager.pop_scope();
        assert!(manager.get_variable("y").is_err());
    }

    #[test]
    fn scope_manager_shadowing_in_nested_scope() {
        let mut manager = StaticCheckerScopeManager::new();
        let _ = manager.declare_variable("x", Type::I64);
        manager.push_scope();
        assert!(manager.declare_variable("x", Type::Str).is_err());
    }

    #[test]
    fn stack_push_and_pop_stack_frame() {
        let mut stack = StaticCheckerStack::new();
        assert_eq!(stack.0.len(), 1);
        assert!(stack.push_stack_frame().is_ok());
        assert_eq!(stack.0.len(), 2);
        stack.pop_stack_frame();
        assert_eq!(stack.0.len(), 1);
    }

    #[test]
    fn stack_declare_and_get_variable() {
        let mut stack = StaticCheckerStack::new();
        assert!(stack.declare_variable("x", Type::Bool).is_ok());
        assert_eq!(stack.get_variable("x").unwrap(), &Type::Bool);
    }

    #[test]
    fn stack_variables_isolated_between_frames() {
        let mut stack = StaticCheckerStack::new();
        let _ = stack.declare_variable("x", Type::I64);
        let _ = stack.push_stack_frame();
        assert!(stack.get_variable("x").is_err());
    }

    #[test]
    fn stack_push_scope_and_pop_scope() {
        let mut stack = StaticCheckerStack::new();
        stack.push_scope();
        let _ = stack.declare_variable("x", Type::I64);
        assert!(stack.get_variable("x").is_ok());
        stack.pop_scope();
        assert!(stack.get_variable("x").is_err());
    }

    #[test]
    fn stack_overflow_after_500_frames() {
        let mut stack = StaticCheckerStack::new();
        for _ in 0..499 {
            assert!(stack.push_stack_frame().is_ok());
        }
        assert_eq!(stack.push_stack_frame().err().unwrap().message(), "Stack overflow.");
    }
}
