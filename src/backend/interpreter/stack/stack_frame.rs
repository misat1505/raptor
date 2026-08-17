use std::fmt::Debug;

use crate::backend::interpreter::stack::scope_manager::ScopeManager;

#[derive(Clone)]
pub(in crate::backend::interpreter::stack) struct StackFrame<'a> {
    pub(in crate::backend::interpreter::stack) scope_manager: ScopeManager<'a>,
}

impl<'a> Debug for StackFrame<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(f, "{:?}", self.scope_manager)?)
    }
}

impl<'a> StackFrame<'a> {
    pub(in crate::backend::interpreter::stack) fn new() -> Self {
        StackFrame {
            scope_manager: ScopeManager::new(),
        }
    }
}
