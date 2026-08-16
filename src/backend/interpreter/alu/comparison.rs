use crate::backend::interpreter::alu::ALU;
use crate::{
    backend::interpreter::alu::value::Value,
    common::errors::{ComputationError, ErrorSeverity},
};

impl ALU {
    pub(in crate::backend::interpreter) fn greater(val1: Value, val2: Value) -> Result<Value, ComputationError> {
        match (val1, val2) {
            (Value::I64(val1), Value::I64(val2)) => Ok(Value::Bool(val1 > val2)),
            (Value::F64(val1), Value::F64(val2)) => Ok(Value::Bool(val1 > val2)),
            (a, b) => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!(
                    "Cannot perform greater between values of type '{:?}' and '{:?}'.",
                    a.to_type(),
                    b.to_type()
                ),
            )),
        }
    }

    pub(in crate::backend::interpreter) fn greater_or_equal(val1: Value, val2: Value) -> Result<Value, ComputationError> {
        match (val1, val2) {
            (Value::I64(val1), Value::I64(val2)) => Ok(Value::Bool(val1 >= val2)),
            (Value::F64(val1), Value::F64(val2)) => Ok(Value::Bool(val1 >= val2)),
            (a, b) => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!(
                    "Cannot perform greater or equal between values of type '{:?}' and '{:?}'.",
                    a.to_type(),
                    b.to_type()
                ),
            )),
        }
    }

    pub(in crate::backend::interpreter) fn less(val1: Value, val2: Value) -> Result<Value, ComputationError> {
        match (val1, val2) {
            (Value::I64(val1), Value::I64(val2)) => Ok(Value::Bool(val1 < val2)),
            (Value::F64(val1), Value::F64(val2)) => Ok(Value::Bool(val1 < val2)),
            (a, b) => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform less between values of type '{:?}' and '{:?}'.", a.to_type(), b.to_type()),
            )),
        }
    }

    pub(in crate::backend::interpreter) fn less_or_equal(val1: Value, val2: Value) -> Result<Value, ComputationError> {
        match (val1, val2) {
            (Value::I64(val1), Value::I64(val2)) => Ok(Value::Bool(val1 <= val2)),
            (Value::F64(val1), Value::F64(val2)) => Ok(Value::Bool(val1 <= val2)),
            (a, b) => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!(
                    "Cannot perform less or equal between values of type '{:?}' and '{:?}'.",
                    a.to_type(),
                    b.to_type()
                ),
            )),
        }
    }

    pub(in crate::backend::interpreter) fn equal(val1: Value, val2: Value) -> Result<Value, ComputationError> {
        match (val1, val2) {
            (Value::I64(val1), Value::I64(val2)) => Ok(Value::Bool(val1 == val2)),
            (Value::F64(val1), Value::F64(val2)) => Ok(Value::Bool(val1 == val2)),
            (Value::String(val1), Value::String(val2)) => Ok(Value::Bool(val1 == val2)),
            (Value::Bool(val1), Value::Bool(val2)) => Ok(Value::Bool(val1 == val2)),
            (a, b) => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform equal between values of type '{:?}' and '{:?}'.", a.to_type(), b.to_type()),
            )),
        }
    }

    pub(in crate::backend::interpreter) fn not_equal(val1: Value, val2: Value) -> Result<Value, ComputationError> {
        match (val1, val2) {
            (Value::I64(val1), Value::I64(val2)) => Ok(Value::Bool(val1 != val2)),
            (Value::F64(val1), Value::F64(val2)) => Ok(Value::Bool(val1 != val2)),
            (Value::String(val1), Value::String(val2)) => Ok(Value::Bool(val1 != val2)),
            (Value::Bool(val1), Value::Bool(val2)) => Ok(Value::Bool(val1 != val2)),
            (a, b) => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!(
                    "Cannot perform not equal between values of type '{:?}' and '{:?}'.",
                    a.to_type(),
                    b.to_type()
                ),
            )),
        }
    }
}
