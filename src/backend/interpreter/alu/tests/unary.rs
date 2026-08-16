use std::assert_eq;

use crate::backend::interpreter::alu::ALU;
use crate::common::errors::IError;
use crate::backend::interpreter::alu::value::Value;

#[test]
fn boolean_negation() {
    assert_eq!(ALU::boolean_negate(Value::Bool(false)).unwrap(), Value::Bool(true));
    assert_eq!(ALU::boolean_negate(Value::Bool(true)).unwrap(), Value::Bool(false));
    assert_eq!(
        ALU::boolean_negate(Value::I64(1)).err().unwrap().message(),
        String::from("Cannot perform boolean negation on type 'i64'.")
    );
}

#[test]
fn arithmetic_negation() {
    assert_eq!(ALU::arithmetic_negate(Value::I64(1)).unwrap(), Value::I64(-1));
    assert_eq!(ALU::arithmetic_negate(Value::F64(-21.37)).unwrap(), Value::F64(21.37));
    assert_eq!(
        ALU::arithmetic_negate(Value::String(String::from("abc"))).err().unwrap().message(),
        String::from("Cannot perform arithmetic negation on type 'str'.")
    );
}
