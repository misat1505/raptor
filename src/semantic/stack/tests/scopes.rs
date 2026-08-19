use crate::{
    common::{errors::IError, span::Span, types::Type},
    semantic::stack::scope_manager::StaticCheckerScopeManager,
};

#[test]
fn scope_manager_declare_and_get() {
    let mut manager = StaticCheckerScopeManager::new();
    assert!(manager.declare_variable("x", Type::I64, Span::default()).is_ok());
    assert_eq!(manager.get_variable("x", Span::default()).unwrap(), &Type::I64);
}

#[test]
fn scope_manager_redeclare_fails() {
    let mut manager = StaticCheckerScopeManager::new();
    let _ = manager.declare_variable("x", Type::I64, Span::default());
    assert_eq!(
        manager.declare_variable("x", Type::F64, Span::default()).err().unwrap().message(),
        "Cannot redeclare variable 'x'."
    );
}

#[test]
fn scope_manager_get_undeclared_fails() {
    let manager = StaticCheckerScopeManager::new();
    assert_eq!(
        manager.get_variable("x", Span::default()).err().unwrap().message(),
        "Variable 'x' not declared in this scope."
    );
}

#[test]
fn scope_manager_assign_same_type_ok() {
    let mut manager = StaticCheckerScopeManager::new();
    let _ = manager.declare_variable("x", Type::I64, Span::default());
    assert!(manager.assign_variable("x", Type::I64, Span::default()).is_ok());
}

#[test]
fn scope_manager_assign_different_type_fails() {
    let mut manager = StaticCheckerScopeManager::new();
    let _ = manager.declare_variable("x", Type::I64, Span::default());
    assert_eq!(
        manager.assign_variable("x", Type::Str, Span::default()).err().unwrap().message(),
        "Cannot assign 'str' to variable 'x' which was previously declared as 'i64'."
    );
}

#[test]
fn scope_manager_assign_undeclared_fails() {
    let mut manager = StaticCheckerScopeManager::new();
    assert_eq!(
        manager.assign_variable("x", Type::I64, Span::default()).err().unwrap().message(),
        "Variable 'x' not declared in this scope."
    );
}

#[test]
fn scope_manager_nested_scope_sees_parent_variable() {
    let mut manager = StaticCheckerScopeManager::new();
    let _ = manager.declare_variable("x", Type::I64, Span::default());
    manager.push_scope();
    assert_eq!(manager.get_variable("x", Span::default()).unwrap(), &Type::I64);
}

#[test]
fn scope_manager_pop_scope_removes_inner_variable() {
    let mut manager = StaticCheckerScopeManager::new();
    manager.push_scope();
    let _ = manager.declare_variable("y", Type::I64, Span::default());
    assert!(manager.get_variable("y", Span::default()).is_ok());
    manager.pop_scope();
    assert!(manager.get_variable("y", Span::default()).is_err());
}

#[test]
fn scope_manager_shadowing_in_nested_scope() {
    let mut manager = StaticCheckerScopeManager::new();
    let _ = manager.declare_variable("x", Type::I64, Span::default());
    manager.push_scope();
    assert!(manager.declare_variable("x", Type::Str, Span::default()).is_err());
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
    manager.declare_variable("inner", Type::Bool, Span::default()).unwrap();

    assert_eq!(manager.get_variable("inner", Span::default()).unwrap(), &Type::Bool);

    manager.pop_scope();

    assert!(manager.get_variable("inner", Span::default()).is_err());
}

#[test]
fn scope_manager_cannot_redeclare_variable_from_outer_scope() {
    let mut manager = StaticCheckerScopeManager::new();

    manager.declare_variable("x", Type::I64, Span::default()).unwrap();
    manager.push_scope();

    let result = manager.declare_variable("x", Type::F64, Span::default());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().message(), "Cannot redeclare variable 'x'.");
}

#[test]
fn scope_manager_assignment_keeps_original_type() {
    let mut manager = StaticCheckerScopeManager::new();

    manager.declare_variable("x", Type::I64, Span::default()).unwrap();

    assert!(manager.assign_variable("x", Type::I64, Span::default()).is_ok());
    assert_eq!(manager.get_variable("x", Span::default()).unwrap(), &Type::I64);

    assert!(manager.assign_variable("x", Type::F64, Span::default()).is_err());
    assert_eq!(manager.get_variable("x", Span::default()).unwrap(), &Type::I64);
}
