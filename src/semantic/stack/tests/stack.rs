use crate::{
    common::{errors::IError, types::Type},
    semantic::stack::stack::StaticCheckerStack,
};

#[test]
fn stack_push_and_pop_stack_frame() {
    let mut stack = StaticCheckerStack::new();
    assert_eq!(stack.0.len(), 1);
    assert!(stack.push_stack_frame().is_ok());
    assert_eq!(stack.0.len(), 2);
    stack.pop_stack_frame();
    assert_eq!(stack.0.len(), 1);
}

#[test]
fn stack_declare_and_get_variable() {
    let mut stack = StaticCheckerStack::new();
    assert!(stack.declare_variable("x", Type::Bool).is_ok());
    assert_eq!(stack.get_variable("x").unwrap(), &Type::Bool);
}

#[test]
fn stack_variables_isolated_between_frames() {
    let mut stack = StaticCheckerStack::new();
    let _ = stack.declare_variable("x", Type::I64);
    let _ = stack.push_stack_frame();
    assert!(stack.get_variable("x").is_err());
}

#[test]
fn stack_push_scope_and_pop_scope() {
    let mut stack = StaticCheckerStack::new();
    stack.push_scope();
    let _ = stack.declare_variable("x", Type::I64);
    assert!(stack.get_variable("x").is_ok());
    stack.pop_scope();
    assert!(stack.get_variable("x").is_err());
}

#[test]
fn stack_overflow_after_500_frames() {
    let mut stack = StaticCheckerStack::new();
    for _ in 0..499 {
        assert!(stack.push_stack_frame().is_ok());
    }
    assert_eq!(stack.push_stack_frame().err().unwrap().message(), "Stack overflow.");
}

#[test]
fn stack_is_in_breakable_false_by_default() {
    let stack = StaticCheckerStack::new();
    assert!(!stack.is_in_breakable());
}

#[test]
fn stack_enter_breakable_sets_flag() {
    let mut stack = StaticCheckerStack::new();
    stack.enter_breakable();
    assert!(stack.is_in_breakable());
}

#[test]
fn stack_exit_breakable_clears_flag() {
    let mut stack = StaticCheckerStack::new();
    stack.enter_breakable();
    stack.exit_breakable();
    assert!(!stack.is_in_breakable());
}

#[test]
fn stack_nested_breakable_counts_correctly() {
    let mut stack = StaticCheckerStack::new();
    stack.enter_breakable(); // fe. for
    stack.enter_breakable(); // fe. switch inside for
    assert!(stack.is_in_breakable());
    stack.exit_breakable(); // exit switch
    assert!(stack.is_in_breakable()); // still inside for
    stack.exit_breakable(); // exit for
    assert!(!stack.is_in_breakable());
}

#[test]
fn stack_breakable_is_per_frame() {
    let mut stack = StaticCheckerStack::new();
    stack.enter_breakable();
    assert!(stack.is_in_breakable());

    let _ = stack.push_stack_frame();
    assert!(!stack.is_in_breakable());

    stack.pop_stack_frame();
    assert!(stack.is_in_breakable());
}

#[test]
fn stack_size_reflects_frame_count() {
    let mut stack = StaticCheckerStack::new();
    assert_eq!(stack.size(), 1);
    let _ = stack.push_stack_frame();
    assert_eq!(stack.size(), 2);
    stack.pop_stack_frame();
    assert_eq!(stack.size(), 1);
}

#[test]
fn stack_is_in_continuable_false_by_default() {
    let stack = StaticCheckerStack::new();

    assert!(!stack.is_in_continuable());
}

#[test]
fn stack_enter_continuable_sets_flag() {
    let mut stack = StaticCheckerStack::new();

    stack.enter_continuable();

    assert!(stack.is_in_continuable());
}

#[test]
fn stack_exit_continuable_clears_flag() {
    let mut stack = StaticCheckerStack::new();

    stack.enter_continuable();
    stack.exit_continuable();

    assert!(!stack.is_in_continuable());
}

#[test]
fn stack_nested_continuable_counts_correctly() {
    let mut stack = StaticCheckerStack::new();

    stack.enter_continuable();
    stack.enter_continuable();

    assert!(stack.is_in_continuable());

    stack.exit_continuable();

    assert!(stack.is_in_continuable());

    stack.exit_continuable();

    assert!(!stack.is_in_continuable());
}

#[test]
fn stack_continuable_is_per_frame() {
    let mut stack = StaticCheckerStack::new();

    stack.enter_continuable();

    assert!(stack.is_in_continuable());

    stack.push_stack_frame().unwrap();

    assert!(!stack.is_in_continuable());

    stack.enter_continuable();

    assert!(stack.is_in_continuable());

    stack.pop_stack_frame();

    assert!(stack.is_in_continuable());
}

#[test]
fn stack_breakable_and_continuable_are_independent() {
    let mut stack = StaticCheckerStack::new();

    stack.enter_breakable();

    assert!(stack.is_in_breakable());
    assert!(!stack.is_in_continuable());

    stack.enter_continuable();

    assert!(stack.is_in_breakable());
    assert!(stack.is_in_continuable());

    stack.exit_breakable();

    assert!(!stack.is_in_breakable());
    assert!(stack.is_in_continuable());

    stack.exit_continuable();

    assert!(!stack.is_in_breakable());
    assert!(!stack.is_in_continuable());
}

#[test]
fn stack_new_frame_resets_breakable_and_continuable_state() {
    let mut stack = StaticCheckerStack::new();

    stack.enter_breakable();
    stack.enter_continuable();

    assert!(stack.is_in_breakable());
    assert!(stack.is_in_continuable());

    stack.push_stack_frame().unwrap();

    assert!(!stack.is_in_breakable());
    assert!(!stack.is_in_continuable());
}

#[test]
fn stack_frame_scope_and_loop_state_are_independent() {
    let mut stack = StaticCheckerStack::new();

    stack.push_scope();
    stack.declare_variable("x", Type::I64).unwrap();
    stack.enter_breakable();
    stack.enter_continuable();

    assert!(stack.get_variable("x").is_ok());
    assert!(stack.is_in_breakable());
    assert!(stack.is_in_continuable());

    stack.pop_scope();

    assert!(stack.get_variable("x").is_err());
    assert!(stack.is_in_breakable());
    assert!(stack.is_in_continuable());
}

#[test]
fn stack_assignment_uses_current_frame_only() {
    let mut stack = StaticCheckerStack::new();

    stack.declare_variable("x", Type::I64).unwrap();
    stack.push_stack_frame().unwrap();

    assert!(stack.assign_variable("x", Type::I64).is_err());
}

#[test]
fn stack_new_frame_has_empty_scope_manager() {
    let mut stack = StaticCheckerStack::new();

    stack.push_scope();
    stack.declare_variable("x", Type::I64).unwrap();

    stack.push_stack_frame().unwrap();

    assert_eq!(stack.0.last().unwrap().scope_manager.len(), 1);
    assert!(stack.get_variable("x").is_err());
}

#[test]
fn stack_overflow_does_not_increase_size() {
    let mut stack = StaticCheckerStack::new();

    for _ in 0..499 {
        stack.push_stack_frame().unwrap();
    }

    assert_eq!(stack.size(), 500);

    let result = stack.push_stack_frame();

    assert!(result.is_err());
    assert_eq!(stack.size(), 500);
}

#[test]
fn stack_pop_frame_restores_previous_loop_state() {
    let mut stack = StaticCheckerStack::new();

    stack.enter_breakable();
    stack.enter_continuable();

    stack.push_stack_frame().unwrap();

    stack.enter_breakable();

    assert!(stack.is_in_breakable());
    assert!(!stack.is_in_continuable());

    stack.pop_stack_frame();

    assert!(stack.is_in_breakable());
    assert!(stack.is_in_continuable());
}
