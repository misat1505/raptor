use crate::{
    common::{
        errors::{ErrorSeverity, SemanticCheckerError},
        types::Type,
    },
    semantic::type_alu::TypeALU,
};

impl TypeALU {
    pub(in crate::semantic) fn greater(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_comparison(t1, t2, "greater")
    }

    pub(in crate::semantic) fn greater_or_equal(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_comparison(t1, t2, "greater or equal")
    }

    pub(in crate::semantic) fn less(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_comparison(t1, t2, "less")
    }

    pub(in crate::semantic) fn less_or_equal(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_comparison(t1, t2, "less or equal")
    }

    pub(in crate::semantic) fn equal(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_equality(t1, t2, "equal")
    }

    pub(in crate::semantic) fn not_equal(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
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
}
