use std::collections::HashMap;

use crate::common::{
    errors::{ErrorSeverity, ScopeManagerError},
    types::Type,
};

#[derive(Debug, Clone)]
pub(in crate::semantic::stack) struct StaticCheckerScope<'a> {
    variables: HashMap<&'a str, Type>,
}

impl<'a> StaticCheckerScope<'a> {
    pub(in crate::semantic::stack) fn new() -> Self {
        StaticCheckerScope { variables: HashMap::new() }
    }

    pub(in crate::semantic::stack) fn get_variable(&self, searched: &'a str) -> Option<&Type> {
        self.variables.get(searched)
    }

    pub(in crate::semantic::stack) fn assign_variable(&mut self, name: &'a str, value: Type) -> Result<(), ScopeManagerError> {
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

    pub(in crate::semantic::stack) fn declare_variable(&mut self, name: &'a str, value: Type) -> Result<(), ScopeManagerError> {
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
