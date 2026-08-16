use crate::{
    common::{
        errors::{ErrorSeverity, SemanticCheckerError},
        types::Type,
    },
    semantic::type_alu::TypeALU,
};

impl TypeALU {
    pub(in crate::semantic) fn check_numeric_operation(t1: Type, t2: Type, op_name: &str) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::I64, Type::I64) => Ok(Type::I64),
            (Type::F64, Type::F64) => Ok(Type::F64),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform {} between values of type '{:?}' and '{:?}'.", op_name, a, b),
            )),
        }
    }

    pub(in crate::semantic) fn add(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
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

    pub(in crate::semantic) fn subtract(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_numeric_operation(t1, t2, "subtraction")
    }

    pub(in crate::semantic) fn multiplication(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_numeric_operation(t1, t2, "multiplication")
    }

    pub(in crate::semantic) fn division(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_numeric_operation(t1, t2, "division")
    }

    pub(in crate::semantic) fn modulo(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::I64, Type::I64) => Ok(Type::I64),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform modulo between values of type '{:?}' and '{:?}'.", a, b),
            )),
        }
    }
}
