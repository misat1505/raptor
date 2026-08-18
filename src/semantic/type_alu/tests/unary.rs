use crate::{
    common::{errors::IError, span::Span, types::Type},
    semantic::type_alu::TypeALU,
};

#[test]
fn boolean_negate_valid() {
    assert_eq!(TypeALU::boolean_negate(Type::Bool, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn boolean_negate_invalid() {
    assert_eq!(
        TypeALU::boolean_negate(Type::I64, Span::default()).err().unwrap().message(),
        "Cannot perform boolean negation on type 'i64'."
    );
}

#[test]
fn arithmetic_negate_valid() {
    assert_eq!(TypeALU::arithmetic_negate(Type::I64, Span::default()).unwrap(), Type::I64);
    assert_eq!(TypeALU::arithmetic_negate(Type::F64, Span::default()).unwrap(), Type::F64);
}

#[test]
fn arithmetic_negate_invalid() {
    assert_eq!(
        TypeALU::arithmetic_negate(Type::Str, Span::default()).err().unwrap().message(),
        "Cannot perform arithmetic negation on type 'str'."
    );
}

#[test]
fn boolean_negate_rejects_all_non_bool_types() {
    for ty in [Type::I64, Type::F64, Type::Str, Type::Void] {
        assert!(TypeALU::boolean_negate(ty, Span::default()).is_err());
    }
}

#[test]
fn arithmetic_negate_rejects_all_non_numeric_types() {
    for ty in [Type::Bool, Type::Str, Type::Void] {
        assert!(TypeALU::arithmetic_negate(ty, Span::default()).is_err());
    }
}
