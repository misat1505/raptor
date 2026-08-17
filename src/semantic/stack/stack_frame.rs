use std::fmt::Debug;

use crate::semantic::stack::scope_manager::StaticCheckerScopeManager;

#[derive(Clone)]
pub(in crate::semantic::stack) struct StaticCheckerStackFrame<'a> {
    pub(in crate::semantic::stack) scope_manager: StaticCheckerScopeManager<'a>,
    pub(in crate::semantic::stack) breakable_count: u64,
    pub(in crate::semantic::stack) continuable_count: u64,
}

impl<'a> Debug for StaticCheckerStackFrame<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(f, "{:?}", self.scope_manager)?)
    }
}

impl<'a> StaticCheckerStackFrame<'a> {
    pub(in crate::semantic::stack) fn new() -> Self {
        StaticCheckerStackFrame {
            scope_manager: StaticCheckerScopeManager::new(),
            breakable_count: 0,
            continuable_count: 0,
        }
    }
}
