use std::{assert_eq, cell::RefCell, rc::Rc};

use crate::{
    backend::interpreter::{stack::scope::Scope, Value},
    common::errors::IError,
};

#[test]
fn initializes_scope() {
    let scope = Scope::new();
    assert!(scope.variables.is_empty());
}

#[test]
fn scope_variables() {
    let mut scope = Scope::new();
    let name = "x";
    let value = Rc::new(RefCell::new(Value::I64(5)));

    let _ = scope.declare_variable(name, value.clone());
    assert_eq!(scope.get_variable(name).unwrap().clone(), value);
    assert!(scope.get_variable("non-existent").is_none());

    let new_value = Rc::new(RefCell::new(Value::I64(0)));
    let _ = scope.assign_variable(name, new_value.clone());
    assert_eq!(scope.get_variable(name).unwrap().clone(), new_value);

    assert_eq!(
        scope
            .assign_variable("y", Rc::new(RefCell::new(Value::Bool(true))))
            .err()
            .unwrap()
            .message(),
        String::from("Variable 'y' not declared.")
    );
}
