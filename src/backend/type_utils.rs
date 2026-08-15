use crate::{backend::interpreter::Value, common::types::Type};

pub fn type_accepts_value(ty: &Type, value: &Value) -> bool {
    match (ty, value) {
        (Type::Bool, Value::Bool(_)) => true,
        (Type::F64, Value::F64(_)) => true,
        (Type::I64, Value::I64(_)) => true,
        (Type::Str, Value::String(_)) => true,

        (Type::Vector(_), Value::Vector { kind, .. }) => *ty == **kind,

        _ => false,
    }
}
