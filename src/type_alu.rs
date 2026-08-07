use crate::{
    ast::Type,
    errors::{ErrorSeverity, SemanticCheckerError},
};

pub struct TypeALU;

impl TypeALU {
    fn check_numeric_operation(t1: Type, t2: Type, op_name: &str) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::I64, Type::I64) => Ok(Type::I64),
            (Type::F64, Type::F64) => Ok(Type::F64),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform {} between values of type '{:?}' and '{:?}'.", op_name, a, b),
            )),
        }
    }
}

impl TypeALU {
    pub fn cast_to_type(from: Type, to_type: &Type) -> Result<Type, SemanticCheckerError> {
        match (from, to_type) {
            (Type::I64, Type::Str) => Ok(Type::Str),
            (Type::F64, Type::Str) => Ok(Type::Str),
            (Type::I64, Type::F64) => Ok(Type::F64),
            (Type::F64, Type::I64) => Ok(Type::I64),
            (Type::I64, Type::Bool) => Ok(Type::Bool),
            (Type::F64, Type::Bool) => Ok(Type::Bool),
            (Type::Str, Type::I64) => Ok(Type::I64),
            (Type::Str, Type::F64) => Ok(Type::F64),
            (Type::Str, Type::Bool) => Ok(Type::Bool),
            (Type::Bool, Type::Str) => Ok(Type::Str),
            (Type::Bool, Type::I64) => Ok(Type::I64),
            (Type::Bool, Type::F64) => Ok(Type::F64),
            (value, target_type) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot cast '{:?}' to '{:?}'.", value, target_type),
            )),
        }
    }

    pub fn boolean_negate(t: Type) -> Result<Type, SemanticCheckerError> {
        match t {
            Type::Bool => Ok(Type::Bool),
            t => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform boolean negation on type '{:?}'.", t),
            )),
        }
    }

    pub fn arithmetic_negate(t: Type) -> Result<Type, SemanticCheckerError> {
        match t {
            Type::I64 => Ok(Type::I64),
            Type::F64 => Ok(Type::F64),
            t => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform arithmetic negation on type '{:?}'.", t),
            )),
        }
    }

    pub fn add(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::I64, Type::I64) => Ok(Type::I64),
            (Type::F64, Type::F64) => Ok(Type::F64),
            (Type::Str, Type::Str) => Ok(Type::Str),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform addition between values of type '{:?}' and '{:?}'.", a, b),
            )),
        }
    }

    pub fn subtract(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_numeric_operation(t1, t2, "subtraction")
    }

    pub fn multiplication(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_numeric_operation(t1, t2, "multiplication")
    }

    pub fn division(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_numeric_operation(t1, t2, "division")
    }

    pub fn concatenation(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::Bool, Type::Bool) => Ok(Type::Bool),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform concatenation between values of type '{:?}' and '{:?}'.", a, b),
            )),
        }
    }

    pub fn alternative(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::Bool, Type::Bool) => Ok(Type::Bool),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform alternative between values of type '{:?}' and '{:?}'.", a, b),
            )),
        }
    }

    pub fn greater(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_comparison(t1, t2, "greater")
    }

    pub fn greater_or_equal(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_comparison(t1, t2, "greater or equal")
    }

    pub fn less(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_comparison(t1, t2, "less")
    }

    pub fn less_or_equal(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_comparison(t1, t2, "less or equal")
    }

    fn check_comparison(t1: Type, t2: Type, op_name: &str) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::I64, Type::I64) => Ok(Type::Bool),
            (Type::F64, Type::F64) => Ok(Type::Bool),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform {} between values of type '{:?}' and '{:?}'.", op_name, a, b),
            )),
        }
    }

    pub fn equal(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_equality(t1, t2, "equal")
    }

    pub fn not_equal(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_equality(t1, t2, "not equal")
    }

    fn check_equality(t1: Type, t2: Type, op_name: &str) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::I64, Type::I64) | (Type::F64, Type::F64) | (Type::Str, Type::Str) | (Type::Bool, Type::Bool) => Ok(Type::Bool),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform {} between values of type '{:?}' and '{:?}'.", op_name, a, b),
            )),
        }
    }
}
