use std::assert_eq;

use crate::backend::interpreter::alu::value::Value;
use crate::backend::interpreter::alu::ALU;
use crate::common::errors::IError;
use crate::common::span::Span;

#[test]
fn boolean_negation() {
    assert_eq!(ALU::boolean_negate(Value::Bool(false), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::boolean_negate(Value::Bool(true), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(
        ALU::boolean_negate(Value::I64(1), Span::default()).err().unwrap().message(),
        String::from("Cannot perform boolean negation on type 'i64'.")
    );
}

#[test]
fn arithmetic_negation() {
    assert_eq!(ALU::arithmetic_negate(Value::I64(1), Span::default()).unwrap(), Value::I64(-1));
    assert_eq!(ALU::arithmetic_negate(Value::F64(-21.37), Span::default()).unwrap(), Value::F64(21.37));
    assert_eq!(
        ALU::arithmetic_negate(Value::String(String::from("abc")), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform arithmetic negation on type 'str'.")
    );
}
