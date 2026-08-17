use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub enum Type {
    Bool,
    Str,
    I64,
    F64,
    Void,
    Vector(Box<Type>),
    #[allow(dead_code)]
    Any, // internal, not available for the user
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Bool => Ok(write!(f, "bool")?),
            Type::F64 => Ok(write!(f, "f64")?),
            Type::I64 => Ok(write!(f, "i64")?),
            Type::Str => Ok(write!(f, "str")?),
            Type::Void => Ok(write!(f, "void")?),
            Type::Vector(inner) => write!(f, "{:?}[]", inner),
            Type::Any => Ok(write!(f, "any")?),
        }
    }
}

impl Type {
    pub fn is_compatible(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Any, _) | (_, Type::Any) => true,
            (Type::Vector(a), Type::Vector(b)) => a.is_compatible(b),
            (a, b) => a == b,
        }
    }
}
