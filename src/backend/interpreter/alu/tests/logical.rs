use std::assert_eq;

use crate::backend::interpreter::alu::value::Value;
use crate::backend::interpreter::alu::ALU;
use crate::common::errors::IError;
use crate::common::span::Span;

#[test]
fn concatenation() {
    assert_eq!(
        ALU::concatenation(Value::Bool(true), Value::Bool(true), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::concatenation(Value::Bool(false), Value::Bool(true), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::concatenation(Value::Bool(true), Value::Bool(false), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::concatenation(Value::Bool(false), Value::Bool(false), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::concatenation(Value::Bool(true), Value::I64(1), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform concatenation between values of type 'bool' and 'i64'.")
    );
}

#[test]
fn alternative() {
    assert_eq!(
        ALU::alternative(Value::Bool(true), Value::Bool(true), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::alternative(Value::Bool(false), Value::Bool(true), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::alternative(Value::Bool(true), Value::Bool(false), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::alternative(Value::Bool(false), Value::Bool(false), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::alternative(Value::Bool(true), Value::I64(1), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform alternative between values of type 'bool' and 'i64'.")
    );
}
