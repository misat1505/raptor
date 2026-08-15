use std::io::BufReader;

use crate::tests::common::helpers::{create_interpreter, setup_program};

#[test]
fn division_by_zero_i64() {
    let text = BufReader::new(
        r#"
    i64 a = 10;
    i64 b = 0;
    i64 c = a / b;
    "#
        .as_bytes(),
    );

    let program = setup_program(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}

#[test]
fn division_by_zero_f64() {
    let text = BufReader::new(
        r#"
    f64 a = 10.0;
    f64 b = 0.0;
    f64 c = a / b;
    "#
        .as_bytes(),
    );

    let program = setup_program(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}

#[test]
fn i64_overflow() {
    let text = BufReader::new(
        r#"
    i64 a = 9223372036854775807;
    i64 b = a + 1;
    "#
        .as_bytes(),
    );

    let program = setup_program(text);
    let mut interpreter = create_interpreter(&program);
    assert!(interpreter.interpret().is_err());
}
