use crate::{backend::interpreter::Value, common::types::Type};

pub fn type_accepts_value(ty: &Type, value: &Value) -> bool {
    match (ty, value) {
        (Type::Bool, Value::Bool(_)) => true,

        (Type::F64, Value::F64(_)) => true,

        (Type::I8, Value::I8(_)) => true,
        (Type::I16, Value::I16(_)) => true,
        (Type::I32, Value::I32(_)) => true,
        (Type::I64, Value::I64(_)) => true,

        (Type::U8, Value::U8(_)) => true,
        (Type::U16, Value::U16(_)) => true,
        (Type::U32, Value::U32(_)) => true,
        (Type::U64, Value::U64(_)) => true,

        (Type::Str, Value::String(_)) => true,
        (Type::Char, Value::Char(_)) => true,

        (Type::Vector(_), Value::Vector { kind, .. }) => type_matches(ty, kind),

        (Type::Struct { identifier: expected, .. }, Value::Struct { identifier: actual, .. }) => expected == actual,

        _ => false,
    }
}

fn type_matches(expected: &Type, actual: &Type) -> bool {
    match (expected, actual) {
        (Type::Vector(a), Type::Vector(b)) => type_matches(a, b),
        (Type::Struct { identifier: a, .. }, Type::Struct { identifier: b, .. }) => a == b,
        (a, b) => a == b,
    }
}
