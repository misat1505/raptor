use std::{cell::RefCell, rc::Rc, vec};

use crate::common::{
    errors::{ComputationError, ErrorSeverity},
    types::Type,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    I64(i64),
    F64(f64),
    String(String),
    Bool(bool),
    Vector {
        kind: Box<Type>,
        values: Rc<RefCell<Vec<Rc<RefCell<Value>>>>>,
    },
}

impl Value {
    pub(in crate::backend) fn default_value(var_type: &Type) -> Result<Value, ComputationError> {
        match var_type {
            Type::Bool => Ok(Value::Bool(false)),
            Type::I64 => Ok(Value::I64(0)),
            Type::F64 => Ok(Value::F64(0.0)),
            Type::Str => Ok(Value::String("".to_owned())),
            Type::Vector(inner) => Ok(Value::Vector {
                kind: Box::new((**inner).clone()),
                values: Rc::new(RefCell::new(vec![])),
            }),
            other => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!("Cannot create default value for type '{:?}'.", other),
            )),
        }
    }

    pub(in crate::backend) fn to_type(&self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::F64(_) => Type::F64,
            Value::I64(_) => Type::I64,
            Value::String(_) => Type::Str,
            Value::Vector { kind, .. } => kind.as_ref().clone(),
        }
    }

    pub(in crate::backend) fn try_into_bool(&self) -> Result<bool, ComputationError> {
        match self {
            Value::Bool(bool) => Ok(*bool),
            _ => Err(ComputationError::new(ErrorSeverity::HIGH, String::from("Given value is not a boolean."))),
        }
    }
}
