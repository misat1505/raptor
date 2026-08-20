use std::assert_eq;

use crate::backend::interpreter::alu::ALU;
use crate::common::errors::IError;
use crate::common::span::Span;
use crate::{backend::interpreter::alu::value::Value, common::types::Type};

#[test]
fn cast_to_type() {
    let data = [
        (Value::I64(1), Type::Str),
        (Value::F64(1.2), Type::Str),
        (Value::I64(1), Type::F64),
        (Value::F64(1.2), Type::I64),
        (Value::I64(1), Type::Bool),
        (Value::I64(0), Type::Bool),
        (Value::F64(1.2), Type::Bool),
        (Value::F64(0.0), Type::Bool),
        (Value::String(String::from("1")), Type::I64),
        (Value::String(String::from("1.2")), Type::F64),
        (Value::String(String::from("some string")), Type::Bool),
        (Value::String(String::from("")), Type::Bool),
    ];

    let expected = [
        Value::String(String::from("1")),
        Value::String(String::from("1.2")),
        Value::F64(1.0),
        Value::I64(1),
        Value::Bool(true),
        Value::Bool(false),
        Value::Bool(true),
        Value::Bool(false),
        Value::I64(1),
        Value::F64(1.2),
        Value::Bool(true),
        Value::Bool(false),
    ];

    for idx in 0..data.len() {
        let (init, to_type) = &data[idx];
        let exp = &expected[idx];
        assert_eq!(ALU::cast_to_type(init.clone(), to_type, Span::default()).unwrap(), *exp);
    }
}

#[test]
fn cast_to_type_fail() {
    let data = [
        (Value::String(String::from("abc")), Type::I64),
        (Value::String(String::from("abc")), Type::F64),
    ];

    for (val, to_type) in data {
        assert_eq!(
            ALU::cast_to_type(val, &to_type, Span::default()).err().unwrap().message(),
            format!("Cannot cast String 'abc' to '{:?}'.", to_type)
        );
    }
}

#[test]
fn cast_bool_to_other_types() {
    assert_eq!(
        ALU::cast_to_type(Value::Bool(true), &Type::Str, Span::default()).unwrap(),
        Value::String(String::from("true"))
    );
    assert_eq!(
        ALU::cast_to_type(Value::Bool(false), &Type::Str, Span::default()).unwrap(),
        Value::String(String::from("false"))
    );
    assert_eq!(ALU::cast_to_type(Value::Bool(true), &Type::I64, Span::default()).unwrap(), Value::I64(1));
    assert_eq!(ALU::cast_to_type(Value::Bool(false), &Type::I64, Span::default()).unwrap(), Value::I64(0));
    assert_eq!(
        ALU::cast_to_type(Value::Bool(true), &Type::F64, Span::default()).unwrap(),
        Value::F64(1.0)
    );
    assert_eq!(
        ALU::cast_to_type(Value::Bool(false), &Type::F64, Span::default()).unwrap(),
        Value::F64(0.0)
    );
}

#[test]
fn cast_to_type_unsupported_combination_fails() {
    let result = ALU::cast_to_type(Value::I64(1), &Type::Char, Span::default());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().message(), String::from("Cannot cast '1' to 'char'."));
}
