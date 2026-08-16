use std::{cell::RefCell, rc::Rc};

use crate::{
    backend::interpreter::{stack::stack_frame::StackFrame, Value},
    common::errors::{ErrorSeverity, ScopeManagerError, StackOverflowError},
};

#[derive(Debug, Clone)]
pub(in crate::backend::interpreter) struct Stack<'a>(pub(in crate::backend::interpreter::stack) Vec<StackFrame<'a>>);

impl<'a> Stack<'a> {
    pub(in crate::backend::interpreter) fn new() -> Self {
        Stack(vec![StackFrame::new()])
    }

    pub(in crate::backend::interpreter) fn push_stack_frame(&mut self) -> Result<(), StackOverflowError> {
        if self.0.len() == 500 {
            return Err(StackOverflowError::new(ErrorSeverity::HIGH, String::from("Stack overflow.")));
        }
        self.0.push(StackFrame::new());
        Ok(())
    }

    pub(in crate::backend::interpreter) fn pop_stack_frame(&mut self) {
        self.0.pop();
    }

    pub(in crate::backend::interpreter) fn push_scope(&mut self) {
        if let Some(last_frame) = self.0.last_mut() {
            last_frame.scope_manager.push_scope();
        }
    }

    pub(in crate::backend::interpreter) fn pop_scope(&mut self) {
        if let Some(last_frame) = self.0.last_mut() {
            last_frame.scope_manager.pop_scope();
        }
    }

    pub(in crate::backend::interpreter) fn get_variable(&mut self, name: &'a str) -> Result<&Rc<RefCell<Value>>, ScopeManagerError> {
        match self.0.last_mut() {
            Some(last_frame) => last_frame.scope_manager.get_variable(name),
            None => unreachable!("Scope stack is empty"),
        }
    }

    pub(in crate::backend::interpreter) fn assign_variable(&mut self, name: &'a str, value: Rc<RefCell<Value>>) -> Result<(), ScopeManagerError> {
        if let Some(last_frame) = self.0.last_mut() {
            last_frame.scope_manager.assign_variable(name, value)?;
        }
        Ok(())
    }

    pub(in crate::backend::interpreter) fn declare_variable(&mut self, name: &'a str, value: Rc<RefCell<Value>>) -> Result<(), ScopeManagerError> {
        if let Some(last_frame) = self.0.last_mut() {
            last_frame.scope_manager.declare_variable(name, value)?;
        }
        Ok(())
    }
}
