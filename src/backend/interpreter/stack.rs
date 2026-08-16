use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    backend::interpreter::{alu::value::Value, scope_manager::ScopeManager},
    common::errors::{ErrorSeverity, ScopeManagerError, StackOverflowError},
};

#[derive(Debug, Clone)]
pub struct Stack<'a>(pub Vec<StackFrame<'a>>);

#[derive(Clone)]
pub struct StackFrame<'a> {
    pub scope_manager: ScopeManager<'a>,
}

impl<'a> Debug for StackFrame<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(f, "{:?}", self.scope_manager)?)
    }
}

impl<'a> StackFrame<'a> {
    pub fn new() -> Self {
        StackFrame {
            scope_manager: ScopeManager::new(),
        }
    }
}

impl<'a> Stack<'a> {
    pub fn new() -> Self {
        Stack(vec![StackFrame::new()])
    }

    pub fn push_stack_frame(&mut self) -> Result<(), StackOverflowError> {
        if self.0.len() == 500 {
            return Err(StackOverflowError::new(ErrorSeverity::HIGH, String::from("Stack overflow.")));
        }
        self.0.push(StackFrame::new());
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

    pub fn get_variable(&mut self, name: &'a str) -> Result<&Rc<RefCell<Value>>, ScopeManagerError> {
        match self.0.last_mut() {
            Some(last_frame) => last_frame.scope_manager.get_variable(name),
            None => unreachable!("Scope stack is empty"),
        }
    }

    pub fn assign_variable(&mut self, name: &'a str, value: Rc<RefCell<Value>>) -> Result<(), ScopeManagerError> {
        if let Some(last_frame) = self.0.last_mut() {
            last_frame.scope_manager.assign_variable(name, value)?;
        }
        Ok(())
    }

    pub fn declare_variable(&mut self, name: &'a str, value: Rc<RefCell<Value>>) -> Result<(), ScopeManagerError> {
        if let Some(last_frame) = self.0.last_mut() {
            last_frame.scope_manager.declare_variable(name, value)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{backend::interpreter::alu::value::Value, common::errors::IError};

    #[test]
    fn test_stack_push_pop_frame() {
        let mut stack = Stack::new();

        assert_eq!(stack.0.len(), 1);

        stack.push_stack_frame().unwrap();
        assert_eq!(stack.0.len(), 2);

        stack.pop_stack_frame();
        assert_eq!(stack.0.len(), 1);
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::new();

        for _ in 0..499 {
            stack.push_stack_frame().unwrap();
        }

        assert_eq!(stack.0.len(), 500);
        let result = stack.push_stack_frame();
        if let Err(e) = result {
            assert_eq!(e.message(), "Stack overflow.");
        }
    }

    #[test]
    fn test_scope_push_pop() {
        let mut stack = Stack::new();

        stack.push_scope();
        if let Some(last_frame) = stack.0.last() {
            assert_eq!(last_frame.scope_manager.len(), 2);
        }

        stack.pop_scope();
        if let Some(last_frame) = stack.0.last() {
            assert_eq!(last_frame.scope_manager.len(), 1);
        }
    }

    #[test]
    fn test_variable_operations() {
        let mut stack = Stack::new();

        let var_name = "x";
        let var_value = Rc::new(RefCell::new(Value::I64(42)));

        stack.declare_variable(var_name, var_value.clone()).unwrap();
        let retrieved_value = stack.get_variable(var_name).unwrap();
        assert_eq!(retrieved_value, &var_value);

        let new_value = Rc::new(RefCell::new(Value::I64(43)));
        stack.assign_variable(var_name, new_value.clone()).unwrap();
        let updated_value = stack.get_variable(var_name).unwrap();
        assert_eq!(updated_value, &new_value);
    }

    #[test]
    fn test_pop_stack_frame_removes_current_frame() {
        let mut stack = Stack::new();

        stack.push_stack_frame().unwrap();
        stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(42)))).unwrap();

        assert_eq!(stack.0.len(), 2);

        stack.pop_stack_frame();

        assert_eq!(stack.0.len(), 1);
        assert!(stack.get_variable("x").is_err());
    }

    #[test]
    fn test_pop_stack_frame_on_empty_stack_is_safe() {
        let mut stack = Stack::new();

        stack.pop_stack_frame();
        assert!(stack.0.is_empty());

        stack.pop_stack_frame();
        assert!(stack.0.is_empty());
    }

    #[test]
    fn test_variables_are_isolated_between_stack_frames() {
        let mut stack = Stack::new();

        let first_value = Rc::new(RefCell::new(Value::I64(1)));
        stack.declare_variable("x", first_value.clone()).unwrap();

        stack.push_stack_frame().unwrap();

        assert!(stack.get_variable("x").is_err());

        let second_value = Rc::new(RefCell::new(Value::I64(2)));
        stack.declare_variable("x", second_value.clone()).unwrap();

        assert_eq!(stack.get_variable("x").unwrap(), &second_value);

        stack.pop_stack_frame();

        assert_eq!(stack.get_variable("x").unwrap(), &first_value);
    }

    #[test]
    fn test_scope_does_not_leak_after_pop() {
        let mut stack = Stack::new();

        stack.push_scope();

        let value = Rc::new(RefCell::new(Value::I64(42)));
        stack.declare_variable("x", value).unwrap();

        assert!(stack.get_variable("x").is_ok());

        stack.pop_scope();

        assert!(stack.get_variable("x").is_err());
    }

    #[test]
    fn test_nested_scope_can_access_outer_variable() {
        let mut stack = Stack::new();

        let value = Rc::new(RefCell::new(Value::I64(42)));
        stack.declare_variable("x", value.clone()).unwrap();

        stack.push_scope();

        assert_eq!(stack.get_variable("x").unwrap(), &value);
    }

    #[test]
    fn test_variable_declaration_is_local_to_current_scope() {
        let mut stack = Stack::new();

        stack.push_scope();

        let value = Rc::new(RefCell::new(Value::I64(42)));
        stack.declare_variable("x", value.clone()).unwrap();

        stack.push_scope();

        assert_eq!(stack.get_variable("x").unwrap(), &value);

        stack.pop_scope();

        assert_eq!(stack.get_variable("x").unwrap(), &value);

        stack.pop_scope();

        assert!(stack.get_variable("x").is_err());
    }

    #[test]
    fn test_assign_variable_updates_existing_variable() {
        let mut stack = Stack::new();

        let original = Rc::new(RefCell::new(Value::I64(10)));
        stack.declare_variable("x", original).unwrap();

        let updated = Rc::new(RefCell::new(Value::I64(20)));
        stack.assign_variable("x", updated.clone()).unwrap();

        assert_eq!(stack.get_variable("x").unwrap(), &updated);
    }

    #[test]
    fn test_get_undeclared_variable_returns_error() {
        let mut stack = Stack::new();

        let result = stack.get_variable("missing");

        assert!(result.is_err());
    }

    #[test]
    fn test_assign_undeclared_variable_returns_error() {
        let mut stack = Stack::new();

        let value = Rc::new(RefCell::new(Value::I64(42)));
        let result = stack.assign_variable("missing", value);

        assert!(result.is_err());
    }

    #[test]
    fn test_declare_duplicate_variable_returns_error() {
        let mut stack = Stack::new();

        let first = Rc::new(RefCell::new(Value::I64(1)));
        stack.declare_variable("x", first).unwrap();

        let second = Rc::new(RefCell::new(Value::I64(2)));
        let result = stack.declare_variable("x", second);

        assert!(result.is_err());
    }

    #[test]
    fn test_same_variable_name_can_be_declared_in_different_frames() {
        let mut stack = Stack::new();

        let first = Rc::new(RefCell::new(Value::I64(1)));
        stack.declare_variable("x", first.clone()).unwrap();

        stack.push_stack_frame().unwrap();

        let second = Rc::new(RefCell::new(Value::I64(2)));
        stack.declare_variable("x", second.clone()).unwrap();

        assert_eq!(stack.get_variable("x").unwrap(), &second);

        stack.pop_stack_frame();

        assert_eq!(stack.get_variable("x").unwrap(), &first);
    }

    #[test]
    fn test_assign_variable_does_not_modify_outer_frame() {
        let mut stack = Stack::new();

        let outer = Rc::new(RefCell::new(Value::I64(1)));
        stack.declare_variable("x", outer.clone()).unwrap();

        stack.push_stack_frame().unwrap();

        let inner = Rc::new(RefCell::new(Value::I64(2)));
        stack.declare_variable("x", inner.clone()).unwrap();

        let replacement = Rc::new(RefCell::new(Value::I64(3)));
        stack.assign_variable("x", replacement.clone()).unwrap();

        assert_eq!(stack.get_variable("x").unwrap(), &replacement);

        stack.pop_stack_frame();

        assert_eq!(stack.get_variable("x").unwrap(), &outer);
    }

    #[test]
    fn test_scope_operations_on_new_stack_frame() {
        let mut stack = Stack::new();

        stack.push_stack_frame().unwrap();
        assert_eq!(stack.0.len(), 2);

        stack.push_scope();

        assert_eq!(stack.0.last().unwrap().scope_manager.len(), 2);

        stack.pop_scope();

        assert_eq!(stack.0.last().unwrap().scope_manager.len(), 1);
    }

    #[test]
    fn test_stack_can_reach_maximum_frame_count() {
        let mut stack = Stack::new();

        for _ in 0..499 {
            assert!(stack.push_stack_frame().is_ok());
        }

        assert_eq!(stack.0.len(), 500);
    }

    #[test]
    fn test_push_after_overflow_does_not_change_stack_size() {
        let mut stack = Stack::new();

        for _ in 0..499 {
            stack.push_stack_frame().unwrap();
        }

        assert_eq!(stack.0.len(), 500);

        let result = stack.push_stack_frame();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message(), "Stack overflow.");
        assert_eq!(stack.0.len(), 500);
    }

    #[test]
    fn test_pop_then_push_frame_works() {
        let mut stack = Stack::new();

        stack.push_stack_frame().unwrap();
        assert_eq!(stack.0.len(), 2);

        stack.pop_stack_frame();
        assert_eq!(stack.0.len(), 1);

        assert!(stack.push_stack_frame().is_ok());
        assert_eq!(stack.0.len(), 2);
    }

    #[test]
    fn test_variable_value_can_be_mutated_through_shared_reference() {
        let mut stack = Stack::new();

        let value = Rc::new(RefCell::new(Value::I64(42)));
        stack.declare_variable("x", value.clone()).unwrap();

        let retrieved = stack.get_variable("x").unwrap();

        *retrieved.borrow_mut() = Value::I64(100);

        assert_eq!(*value.borrow(), Value::I64(100));
    }
}
