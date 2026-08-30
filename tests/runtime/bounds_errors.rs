use std::io::BufReader;

use crate::common::{create_interpreter, setup_program};

#[test]
fn vector_index_out_of_bounds() {
    let text = BufReader::new(
        r#"
    i64[] arr = [1, 2, 3];
    i64 x = arr[10];
    "#
        .as_bytes(),
    );

    let program = setup_program(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}

#[test]
fn negative_index_fails() {
    let text = BufReader::new(
        r#"
    i64[] arr = [1, 2, 3];
    i64 idx = 0 - 1;
    i64 x = arr[idx];
    "#
        .as_bytes(),
    );

    let program = setup_program(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}

#[test]
fn index_assignment_out_of_bounds_fails() {
    let text = BufReader::new(
        r#"
    i64[] arr = [1, 2, 3];
    arr[10] = 99;
    "#
        .as_bytes(),
    );

    let program = setup_program(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}
