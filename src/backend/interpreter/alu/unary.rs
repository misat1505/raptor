use crate::backend::interpreter::alu::ALU;
use crate::{
    backend::interpreter::alu::value::Value,
    common::errors::{ComputationError, ErrorSeverity},
};

impl ALU {
    pub(in crate::backend::interpreter) fn boolean_negate(val: Value) -> Result<Value, ComputationError> {
        match val {
            Value::Bool(bool) => Ok(Value::Bool(!bool)),
            val => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform boolean negation on type '{:?}'.", val.to_type()),
            )),
        }
    }

    pub(in crate::backend::interpreter) fn arithmetic_negate(val: Value) -> Result<Value, ComputationError> {
        match val {
            Value::I64(i64) => Ok(Value::I64(-i64)),
            Value::F64(f64) => Ok(Value::F64(-f64)),
            val => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform arithmetic negation on type '{:?}'.", val.to_type()),
            )),
        }
    }
}
