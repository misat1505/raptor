use crate::backend::interpreter::alu::ALU;
use crate::{
    backend::interpreter::alu::value::Value,
    common::{
        errors::{ComputationError, ErrorSeverity},
        types::Type,
    },
};

impl ALU {
    pub(in crate::backend::interpreter) fn cast_to_type(val: Value, to_type: &Type) -> Result<Value, ComputationError> {
        match (val, to_type) {
            (Value::I64(i64), Type::Str) => Ok(Value::String(i64.to_string())),
            (Value::F64(f64), Type::Str) => Ok(Value::String(f64.to_string())),
            (Value::I64(i64), Type::F64) => Ok(Value::F64(i64 as f64)),
            (Value::F64(f64), Type::I64) => Ok(Value::I64(f64 as i64)),
            (Value::I64(i64), Type::Bool) => Ok(Value::Bool(i64 > 0)),
            (Value::F64(f64), Type::Bool) => Ok(Value::Bool(f64 > 0.0)),
            (Value::String(string), Type::I64) => match string.parse::<i64>() {
                Ok(i64) => Ok(Value::I64(i64)),
                Err(_) => Err(ComputationError::new(
                    ErrorSeverity::HIGH,
                    format!("Cannot cast String '{}' to 'i64'.", string),
                )),
            },
            (Value::String(string), Type::F64) => match string.parse::<f64>() {
                Ok(f64) => Ok(Value::F64(f64)),
                Err(_) => Err(ComputationError::new(
                    ErrorSeverity::HIGH,
                    format!("Cannot cast String '{}' to 'f64'.", string),
                )),
            },
            (Value::String(string), Type::Bool) => Ok(Value::Bool(string.as_str() != "")),
            (Value::Bool(bool), Type::Str) => return Ok(Value::String(String::from(if bool { "true" } else { "false" }))),
            (Value::Bool(bool), Type::I64) => return Ok(Value::I64(if bool { 1 } else { 0 })),
            (Value::Bool(bool), Type::F64) => return Ok(Value::F64(if bool { 1.0 } else { 0.0 })),
            (value, target_type) => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!("Cannot cast '{:?}' to '{:?}'.", value, target_type),
            )),
        }
    }
}
