use std::io::BufReader;

use crate::tests::common::helpers::{create_interpreter, setup_program, setup_program_skip_typecheck};

#[test]
fn variable_out_of_scope_after_block() {
    let text = BufReader::new(
        r#"
    if (true) {
      i64 x = 5;
    }
    i64 y = x;
    "#
        .as_bytes(),
    );

    let program = setup_program_skip_typecheck(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}

#[test]
fn for_loop_iterator_not_visible_outside() {
    let text = BufReader::new(
        r#"
    for (i64 i = 0; i < 3; i = i + 1) {}
    i64 x = i;
    "#
        .as_bytes(),
    );

    let program = setup_program_skip_typecheck(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}

#[test]
fn shadowing_not_allowed_in_nested_block() {
    let text = BufReader::new(
        r#"
    i64 x = 1;
    if (true) {
      i64 x = 2;
    }
    "#
        .as_bytes(),
    );

    let program = setup_program_skip_typecheck(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}

#[test]
fn redeclare_variable_in_same_scope_fails() {
    let text = BufReader::new(
        r#"
    i64 x = 1;
    i64 x = 2;
    "#
        .as_bytes(),
    );

    let program = setup_program_skip_typecheck(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}

#[test]
fn assign_to_undeclared_variable_fails() {
    let text = BufReader::new(
        r#"
    x = 5;
    "#
        .as_bytes(),
    );

    let program = setup_program_skip_typecheck(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}

#[test]
fn calling_undeclared_function_fails() {
    let text = BufReader::new(
        r#"
    does_not_exist(1, 2);
    "#
        .as_bytes(),
    );

    let program = setup_program_skip_typecheck(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}

#[test]
fn function_call_with_wrong_arg_type_fails() {
    let text = BufReader::new(
        r#"
    fn takes_i64(i64 x): void {}
    takes_i64("not a number");
    "#
        .as_bytes(),
    );

    let program = setup_program_skip_typecheck(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}

#[test]
fn function_missing_return_for_non_void_fails() {
    let text = BufReader::new(
        r#"
    fn broken(): i64 {
      i64 x = 5;
    }
    i64 y = broken();
    "#
        .as_bytes(),
    );

    let program = setup_program(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}
