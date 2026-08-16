use std::{assert_eq, cell::RefCell, rc::Rc};

use crate::{
    backend::interpreter::{
        stack::scope_manager::ScopeManager,
        Value,
    },
    common::errors::IError,
};

#[test]
fn initializes_scope_manager() {
    let manager = ScopeManager::new();
    assert_eq!(manager.scopes.len(), 1);
}

#[test]
fn manages_scopes() {
    let mut manager = ScopeManager::new();
    assert_eq!(manager.scopes.len(), 1);

    manager.push_scope();
    assert_eq!(manager.scopes.len(), 2);

    manager.pop_scope();
    assert_eq!(manager.scopes.len(), 1);
}

#[test]
fn manages_variables() {
    // i64 x = 1;
    // {x = 5; i64 y = 2;}
    // {y; i64 y = 3;}

    let mut manager = ScopeManager::new();

    let _ = manager.declare_variable("x", Rc::new(RefCell::new(Value::I64(1))));
    assert_eq!(manager.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(1))));

    manager.push_scope();
    assert_eq!(manager.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(1))));

    let _ = manager.assign_variable("x", Rc::new(RefCell::new(Value::I64(5))));
    assert_eq!(manager.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(5))));

    let _ = manager.declare_variable("y", Rc::new(RefCell::new(Value::I64(2))));
    assert_eq!(manager.get_variable("y").unwrap().clone(), Rc::new(RefCell::new(Value::I64(2))));

    manager.pop_scope();
    assert_eq!(manager.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(5))));
    assert_eq!(
        manager.get_variable("y").err().unwrap().message(),
        String::from("Variable 'y' not declared in this scope.")
    );

    manager.push_scope();
    assert_eq!(
        manager.get_variable("y").err().unwrap().message(),
        String::from("Variable 'y' not declared in this scope.")
    );

    let _ = manager.declare_variable("y", Rc::new(RefCell::new(Value::I64(3))));
    assert_eq!(manager.get_variable("y").unwrap().clone(), Rc::new(RefCell::new(Value::I64(3))));

    manager.pop_scope();
}

#[test]
fn bad_assign_type() {
    let mut manager = ScopeManager::new();

    let _ = manager.declare_variable("x", Rc::new(RefCell::new(Value::I64(1))));
    assert_eq!(
        manager
            .assign_variable("x", Rc::new(RefCell::new(Value::Bool(true))))
            .err()
            .unwrap()
            .message(),
        String::from("Cannot assign 'bool' to variable 'x' which was previously declared as 'i64'.")
    );
}

#[test]
fn doesnt_allow_redeclare() {
    let mut manager = ScopeManager::new();

    let _ = manager.declare_variable("x", Rc::new(RefCell::new(Value::I64(1))));
    assert_eq!(
        manager
            .declare_variable("x", Rc::new(RefCell::new(Value::I64(6))))
            .err()
            .unwrap()
            .message(),
        String::from("Cannot redeclare variable 'x'.")
    );
}

#[test]
fn scope_manager_len() {
    let mut manager = ScopeManager::new();
    assert_eq!(manager.len(), 1);

    manager.push_scope();
    assert_eq!(manager.len(), 2);

    manager.push_scope();
    assert_eq!(manager.len(), 3);

    manager.pop_scope();
    assert_eq!(manager.len(), 2);
}

#[test]
fn scope_manager_assign_undeclared_variable_fails() {
    let mut manager = ScopeManager::new();

    assert_eq!(
        manager
            .assign_variable("x", Rc::new(RefCell::new(Value::I64(1))))
            .err()
            .unwrap()
            .message(),
        String::from("Variable 'x' not declared in this scope.")
    );
}

#[test]
fn disallows_shadowing_variable_from_outer_scope() {
    // i64 x = 1;
    // { i64 x = 2; } <- should fail, shadowing not allowed
    let mut manager = ScopeManager::new();

    let _ = manager.declare_variable("x", Rc::new(RefCell::new(Value::I64(1))));

    manager.push_scope();
    assert_eq!(
        manager
            .declare_variable("x", Rc::new(RefCell::new(Value::I64(2))))
            .err()
            .unwrap()
            .message(),
        String::from("Cannot redeclare variable 'x'.")
    );

    assert_eq!(manager.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(1))));
}

#[test]
fn popping_last_remaining_scope_breaks_declare_invariant() {
    let mut manager = ScopeManager::new();
    assert_eq!(manager.len(), 1);

    manager.pop_scope();
    assert_eq!(manager.len(), 0);

    assert_eq!(
        manager
            .declare_variable("x", Rc::new(RefCell::new(Value::I64(1))))
            .err()
            .unwrap()
            .message(),
        String::from("No scope available to set the variable.")
    );
}

#[test]
fn popping_last_remaining_scope_makes_get_variable_fail() {
    let mut manager = ScopeManager::new();
    let _ = manager.declare_variable("x", Rc::new(RefCell::new(Value::I64(1))));

    manager.pop_scope();

    assert_eq!(
        manager.get_variable("x").err().unwrap().message(),
        String::from("Variable 'x' not declared in this scope.")
    );
}
