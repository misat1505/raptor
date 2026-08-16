use crate::{
    common::{errors::IError, types::Type},
    semantic::stack::scope_manager::StaticCheckerScopeManager,
};

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
fn scope_manager_len_changes_with_nested_scopes() {
    let mut manager = StaticCheckerScopeManager::new();

    assert_eq!(manager.len(), 1);

    manager.push_scope();
    assert_eq!(manager.len(), 2);

    manager.push_scope();
    assert_eq!(manager.len(), 3);

    manager.pop_scope();
    assert_eq!(manager.len(), 2);

    manager.pop_scope();
    assert_eq!(manager.len(), 1);
}

#[test]
fn scope_manager_nested_scope_variable_disappears_after_pop() {
    let mut manager = StaticCheckerScopeManager::new();

    manager.push_scope();
    manager.declare_variable("inner", Type::Bool).unwrap();

    assert_eq!(manager.get_variable("inner").unwrap(), &Type::Bool);

    manager.pop_scope();

    assert!(manager.get_variable("inner").is_err());
}

#[test]
fn scope_manager_cannot_redeclare_variable_from_outer_scope() {
    let mut manager = StaticCheckerScopeManager::new();

    manager.declare_variable("x", Type::I64).unwrap();
    manager.push_scope();

    let result = manager.declare_variable("x", Type::F64);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().message(), "Cannot redeclare variable 'x'.");
}

#[test]
fn scope_manager_assignment_keeps_original_type() {
    let mut manager = StaticCheckerScopeManager::new();

    manager.declare_variable("x", Type::I64).unwrap();

    assert!(manager.assign_variable("x", Type::I64).is_ok());
    assert_eq!(manager.get_variable("x").unwrap(), &Type::I64);

    assert!(manager.assign_variable("x", Type::F64).is_err());
    assert_eq!(manager.get_variable("x").unwrap(), &Type::I64);
}
