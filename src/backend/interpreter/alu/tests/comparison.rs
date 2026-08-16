use std::assert_eq;

use crate::backend::interpreter::alu::ALU;
use crate::common::errors::IError;
use crate::backend::interpreter::alu::value::Value;

#[test]
fn greater() {
    assert_eq!(ALU::greater(Value::I64(1), Value::I64(2)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::greater(Value::I64(2), Value::I64(2)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::greater(Value::I64(3), Value::I64(2)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::greater(Value::F64(1.0), Value::F64(2.0)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::greater(Value::F64(2.0), Value::F64(2.0)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::greater(Value::F64(3.0), Value::F64(2.0)).unwrap(), Value::Bool(true));
    assert_eq!(
        ALU::greater(Value::I64(2), Value::F64(3.0)).err().unwrap().message(),
        String::from("Cannot perform greater between values of type 'i64' and 'f64'.")
    );
}

#[test]
fn greater_or_equal() {
    assert_eq!(ALU::greater_or_equal(Value::I64(1), Value::I64(2)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::greater_or_equal(Value::I64(2), Value::I64(2)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::greater_or_equal(Value::I64(3), Value::I64(2)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::greater_or_equal(Value::F64(1.0), Value::F64(2.0)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::greater_or_equal(Value::F64(2.0), Value::F64(2.0)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::greater_or_equal(Value::F64(3.0), Value::F64(2.0)).unwrap(), Value::Bool(true));
    assert_eq!(
        ALU::greater_or_equal(Value::I64(2), Value::F64(3.0)).err().unwrap().message(),
        String::from("Cannot perform greater or equal between values of type 'i64' and 'f64'.")
    );
}

#[test]
fn less() {
    assert_eq!(ALU::less(Value::I64(1), Value::I64(2)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::less(Value::I64(2), Value::I64(2)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::less(Value::I64(3), Value::I64(2)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::less(Value::F64(1.0), Value::F64(2.0)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::less(Value::F64(2.0), Value::F64(2.0)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::less(Value::F64(3.0), Value::F64(2.0)).unwrap(), Value::Bool(false));
    assert_eq!(
        ALU::less(Value::I64(2), Value::F64(3.0)).err().unwrap().message(),
        String::from("Cannot perform less between values of type 'i64' and 'f64'.")
    );
}

#[test]
fn less_or_equal() {
    assert_eq!(ALU::less_or_equal(Value::I64(1), Value::I64(2)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::less_or_equal(Value::I64(2), Value::I64(2)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::less_or_equal(Value::I64(3), Value::I64(2)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::less_or_equal(Value::F64(1.0), Value::F64(2.0)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::less_or_equal(Value::F64(2.0), Value::F64(2.0)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::less_or_equal(Value::F64(3.0), Value::F64(2.0)).unwrap(), Value::Bool(false));
    assert_eq!(
        ALU::less_or_equal(Value::I64(2), Value::F64(3.0)).err().unwrap().message(),
        String::from("Cannot perform less or equal between values of type 'i64' and 'f64'.")
    );
}

#[test]
fn equal() {
    assert_eq!(ALU::equal(Value::I64(1), Value::I64(2)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::equal(Value::I64(2), Value::I64(2)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::equal(Value::F64(1.0), Value::F64(2.0)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::equal(Value::F64(2.0), Value::F64(2.0)).unwrap(), Value::Bool(true));
    assert_eq!(
        ALU::equal(Value::String(String::from("a")), Value::String(String::from("b"))).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::equal(Value::String(String::from("a")), Value::String(String::from("a"))).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(ALU::equal(Value::Bool(true), Value::Bool(false)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::equal(Value::Bool(true), Value::Bool(true)).unwrap(), Value::Bool(true));
    assert_eq!(
        ALU::equal(Value::Bool(true), Value::I64(1)).err().unwrap().message(),
        String::from("Cannot perform equal between values of type 'bool' and 'i64'.")
    );
}

#[test]
fn not_equal() {
    assert_eq!(ALU::not_equal(Value::I64(1), Value::I64(2)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::not_equal(Value::I64(2), Value::I64(2)).unwrap(), Value::Bool(false));
    assert_eq!(ALU::not_equal(Value::F64(1.0), Value::F64(2.0)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::not_equal(Value::F64(2.0), Value::F64(2.0)).unwrap(), Value::Bool(false));
    assert_eq!(
        ALU::not_equal(Value::String(String::from("a")), Value::String(String::from("b"))).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::not_equal(Value::String(String::from("a")), Value::String(String::from("a"))).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(ALU::not_equal(Value::Bool(true), Value::Bool(false)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::not_equal(Value::Bool(true), Value::Bool(true)).unwrap(), Value::Bool(false));
    assert_eq!(
        ALU::not_equal(Value::Bool(true), Value::I64(1)).err().unwrap().message(),
        String::from("Cannot perform not equal between values of type 'bool' and 'i64'.")
    );
}

#[test]
fn comparisons_unsupported_types_fail() {
    assert_eq!(
        ALU::greater(Value::Bool(true), Value::Bool(false)).err().unwrap().message(),
        String::from("Cannot perform greater between values of type 'bool' and 'bool'.")
    );
    assert_eq!(
        ALU::greater_or_equal(Value::String(String::from("a")), Value::String(String::from("b")))
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform greater or equal between values of type 'str' and 'str'.")
    );
    assert_eq!(
        ALU::less(Value::Bool(true), Value::Bool(false)).err().unwrap().message(),
        String::from("Cannot perform less between values of type 'bool' and 'bool'.")
    );
    assert_eq!(
        ALU::less_or_equal(Value::String(String::from("a")), Value::String(String::from("b")))
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform less or equal between values of type 'str' and 'str'.")
    );
}
