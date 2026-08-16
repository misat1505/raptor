use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    backend::interpreter::{stack::scope::Scope, Value},
    common::errors::{ErrorSeverity, ScopeManagerError},
};

#[derive(Debug, Clone)]
pub(in crate::backend::interpreter::stack) struct ScopeManager<'a> {
    // always has at least 1 scope
    pub(in crate::backend::interpreter::stack) scopes: Vec<Scope<'a>>,
}

impl<'a> ScopeManager<'a> {
    pub(in crate::backend::interpreter::stack) fn new() -> Self {
        let root_scope = Scope::new();
        ScopeManager { scopes: vec![root_scope] }
    }

    pub(in crate::backend::interpreter::stack) fn push_scope(&mut self) {
        let new_scope = Scope::new();
        self.scopes.push(new_scope);
    }

    pub(in crate::backend::interpreter::stack) fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub(in crate::backend::interpreter::stack) fn get_variable(&self, searched: &'a str) -> Result<&Rc<RefCell<Value>>, ScopeManagerError> {
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

    pub(in crate::backend::interpreter::stack) fn assign_variable(
        &mut self,
        name: &'a str,
        value: Rc<RefCell<Value>>,
    ) -> Result<(), ScopeManagerError> {
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

    pub(in crate::backend::interpreter::stack) fn declare_variable(
        &mut self,
        name: &'a str,
        value: Rc<RefCell<Value>>,
    ) -> Result<(), ScopeManagerError> {
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
    pub(in crate::backend::interpreter::stack) fn len(&self) -> u32 {
        self.scopes.len() as u32
    }
}
