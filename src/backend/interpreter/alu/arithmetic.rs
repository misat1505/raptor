use crate::backend::interpreter::alu::value::Value;
use crate::backend::interpreter::alu::ALU;
use crate::common::errors::{ComputationError, ErrorSeverity};

impl ALU {
    fn check_int_operation<F>(val1: &Value, val2: &Value, op: F, op_name: &str) -> Result<Value, ComputationError>
    where
        F: Fn(i64, i64) -> Option<i64>,
    {
        match (val1, val2) {
            (Value::I64(a), Value::I64(b)) => match op(*a, *b) {
                Some(result) => Ok(Value::I64(result)),
                None => Err(ComputationError::new(
                    ErrorSeverity::HIGH,
                    format!("Overflow occurred when performing {} on i64s.", op_name),
                )),
            },
            _ => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!(
                    "Cannot perform {} between values of type '{:?}' and '{:?}'.",
                    op_name,
                    val1.to_type(),
                    val2.to_type()
                ),
            )),
        }
    }

    fn check_float_operation<F>(val1: &Value, val2: &Value, op: F, op_name: &str) -> Result<Value, ComputationError>
    where
        F: Fn(f64, f64) -> f64,
    {
        match (val1, val2) {
            (Value::F64(a), Value::F64(b)) => {
                let result = op(*a, *b);
                if result.is_infinite() || result.is_nan() {
                    Err(ComputationError::new(
                        ErrorSeverity::HIGH,
                        format!("Invalid result when performing {} on f64s.", op_name),
                    ))
                } else {
                    Ok(Value::F64(result))
                }
            }
            _ => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!(
                    "Cannot perform {} between values of type '{:?}' and '{:?}'.",
                    op_name,
                    val1.to_type(),
                    val2.to_type()
                ),
            )),
        }
    }

    pub(in crate::backend::interpreter) fn add(val1: Value, val2: Value) -> Result<Value, ComputationError> {
        match (&val1, &val2) {
            (Value::I64(_), Value::I64(_)) => Self::check_int_operation(&val1, &val2, i64::checked_add, "addition"),
            (Value::F64(_), Value::F64(_)) => Self::check_float_operation(&val1, &val2, |a, b| a + b, "addition"),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a.clone() + b)),
            (a, b) => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!(
                    "Cannot perform addition between values of type '{:?}' and '{:?}'.",
                    a.to_type(),
                    b.to_type()
                ),
            )),
        }
    }

    pub(in crate::backend::interpreter) fn subtract(val1: Value, val2: Value) -> Result<Value, ComputationError> {
        match (&val1, &val2) {
            (Value::I64(_), Value::I64(_)) => Self::check_int_operation(&val1, &val2, i64::checked_sub, "subtraction"),
            (Value::F64(_), Value::F64(_)) => Self::check_float_operation(&val1, &val2, |a, b| a - b, "subtraction"),
            (a, b) => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!(
                    "Cannot perform subtraction between values of type '{:?}' and '{:?}'.",
                    a.to_type(),
                    b.to_type()
                ),
            )),
        }
    }

    pub(in crate::backend::interpreter) fn multiplication(val1: Value, val2: Value) -> Result<Value, ComputationError> {
        match (&val1, &val2) {
            (Value::I64(_), Value::I64(_)) => Self::check_int_operation(&val1, &val2, i64::checked_mul, "multiplication"),
            (Value::F64(_), Value::F64(_)) => Self::check_float_operation(&val1, &val2, |a, b| a * b, "multiplication"),
            (a, b) => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!(
                    "Cannot perform multiplication between values of type '{:?}' and '{:?}'.",
                    a.to_type(),
                    b.to_type()
                ),
            )),
        }
    }

    pub(in crate::backend::interpreter) fn division(val1: Value, val2: Value) -> Result<Value, ComputationError> {
        match (&val1, &val2) {
            (Value::I64(_), Value::I64(_)) => Self::check_int_operation(&val1, &val2, i64::checked_div, "division"),
            (Value::F64(_), Value::F64(_)) => Self::check_float_operation(&val1, &val2, |a, b| a / b, "division"),
            (a, b) => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!(
                    "Cannot perform division between values of type '{:?}' and '{:?}'.",
                    a.to_type(),
                    b.to_type()
                ),
            )),
        }
    }

    pub(in crate::backend::interpreter) fn modulo(val1: Value, val2: Value) -> Result<Value, ComputationError> {
        match (&val1, &val2) {
            (Value::I64(_), Value::I64(_)) => Self::check_int_operation(&val1, &val2, i64::checked_rem, "modulo"),
            (a, b) => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!(
                    "Cannot perform modulo between values of type '{:?}' and '{:?}'.",
                    a.to_type(),
                    b.to_type()
                ),
            )),
        }
    }
}
