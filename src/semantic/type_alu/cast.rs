use crate::{
    common::{
        errors::{ErrorSeverity, SemanticCheckerError},
        span::Span,
        types::Type,
    },
    semantic::type_alu::TypeALU,
};

impl TypeALU {
    pub(in crate::semantic) fn cast_to_type(from: Type, to_type: &Type, span: Span) -> Result<Type, SemanticCheckerError> {
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
                span,
            )),
        }
    }
}
