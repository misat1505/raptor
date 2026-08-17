use crate::{
    common::{
        errors::{ErrorSeverity, SemanticCheckerError},
        types::Type,
    },
    semantic::type_alu::TypeALU,
};

impl TypeALU {
    pub(in crate::semantic) fn concatenation(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::Bool, Type::Bool) => Ok(Type::Bool),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform concatenation between values of type '{:?}' and '{:?}'.", a, b),
            )),
        }
    }

    pub(in crate::semantic) fn alternative(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::Bool, Type::Bool) => Ok(Type::Bool),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform alternative between values of type '{:?}' and '{:?}'.", a, b),
            )),
        }
    }
}
