use crate::{
    common::{
        errors::{ErrorSeverity, SemanticCheckerError},
        types::Type,
    },
    semantic::type_alu::TypeALU,
};

#[test]
fn concatenation_valid() {
    assert_eq!(TypeALU::concatenation(Type::Bool, Type::Bool).unwrap(), Type::Bool);
}

#[test]
fn concatenation_invalid() {
    assert_eq!(
        TypeALU::concatenation(Type::I64, Type::Bool).err().unwrap().message(),
        "Cannot perform concatenation between values of type 'i64' and 'bool'."
    );
}

#[test]
fn alternative_valid() {
    assert_eq!(TypeALU::alternative(Type::Bool, Type::Bool).unwrap(), Type::Bool);
}

#[test]
fn alternative_invalid() {
    assert_eq!(
        TypeALU::alternative(Type::I64, Type::Bool).err().unwrap().message(),
        "Cannot perform alternative between values of type 'i64' and 'bool'."
    );
}

#[test]
fn concatenation_rejects_non_bool_pairs() {
    assert!(TypeALU::concatenation(Type::I64, Type::I64).is_err());
    assert!(TypeALU::concatenation(Type::F64, Type::F64).is_err());
    assert!(TypeALU::concatenation(Type::Str, Type::Str).is_err());
}

#[test]
fn alternative_rejects_non_bool_pairs() {
    assert!(TypeALU::alternative(Type::I64, Type::I64).is_err());
    assert!(TypeALU::alternative(Type::F64, Type::F64).is_err());
    assert!(TypeALU::alternative(Type::Str, Type::Str).is_err());
}
