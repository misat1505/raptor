use crate::{
    common::{errors::IError, span::Span, types::Type},
    semantic::type_alu::TypeALU,
};

#[test]
fn greater_valid() {
    assert_eq!(TypeALU::greater(Type::I64, Type::I64, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::greater(Type::F64, Type::F64, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn greater_invalid() {
    assert_eq!(
        TypeALU::greater(Type::I64, Type::F64, Span::default()).err().unwrap().message(),
        "Cannot perform greater between values of type 'i64' and 'f64'."
    );
}

#[test]
fn greater_or_equal_valid() {
    assert_eq!(TypeALU::greater_or_equal(Type::I64, Type::I64, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn greater_or_equal_invalid() {
    assert_eq!(
        TypeALU::greater_or_equal(Type::Str, Type::Str, Span::default()).err().unwrap().message(),
        "Cannot perform greater or equal between values of type 'str' and 'str'."
    );
}

#[test]
fn less_valid() {
    assert_eq!(TypeALU::less(Type::I64, Type::I64, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn less_invalid() {
    assert_eq!(
        TypeALU::less(Type::Bool, Type::Bool, Span::default()).err().unwrap().message(),
        "Cannot perform less between values of type 'bool' and 'bool'."
    );
}

#[test]
fn less_or_equal_valid() {
    assert_eq!(TypeALU::less_or_equal(Type::F64, Type::F64, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn less_or_equal_invalid() {
    assert_eq!(
        TypeALU::less_or_equal(Type::I64, Type::Str, Span::default()).err().unwrap().message(),
        "Cannot perform less or equal between values of type 'i64' and 'str'."
    );
}

#[test]
fn equal_valid() {
    assert_eq!(TypeALU::equal(Type::I64, Type::I64, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::equal(Type::F64, Type::F64, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::equal(Type::Str, Type::Str, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::equal(Type::Bool, Type::Bool, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn equal_invalid() {
    assert_eq!(
        TypeALU::equal(Type::I64, Type::Str, Span::default()).err().unwrap().message(),
        "Cannot perform equal between values of type 'i64' and 'str'."
    );
}

#[test]
fn not_equal_valid() {
    assert_eq!(TypeALU::not_equal(Type::I64, Type::I64, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn not_equal_invalid() {
    assert_eq!(
        TypeALU::not_equal(Type::Bool, Type::I64, Span::default()).err().unwrap().message(),
        "Cannot perform not equal between values of type 'bool' and 'i64'."
    );
}

#[test]
fn greater_supports_f64() {
    assert_eq!(TypeALU::greater(Type::F64, Type::F64, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn greater_or_equal_supports_f64() {
    assert_eq!(TypeALU::greater_or_equal(Type::F64, Type::F64, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn less_supports_f64() {
    assert_eq!(TypeALU::less(Type::F64, Type::F64, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn less_or_equal_supports_i64() {
    assert_eq!(TypeALU::less_or_equal(Type::I64, Type::I64, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn comparisons_reject_mixed_numeric_types() {
    assert!(TypeALU::greater(Type::I64, Type::F64, Span::default()).is_err());
    assert!(TypeALU::greater_or_equal(Type::F64, Type::I64, Span::default()).is_err());
    assert!(TypeALU::less(Type::I64, Type::F64, Span::default()).is_err());
    assert!(TypeALU::less_or_equal(Type::F64, Type::I64, Span::default()).is_err());
}

#[test]
fn comparisons_reject_non_numeric_types() {
    assert!(TypeALU::greater(Type::Str, Type::Str, Span::default()).is_err());
    assert!(TypeALU::greater_or_equal(Type::Bool, Type::Bool, Span::default()).is_err());
    assert!(TypeALU::less(Type::Str, Type::Str, Span::default()).is_err());
    assert!(TypeALU::less_or_equal(Type::Bool, Type::Bool, Span::default()).is_err());
}

#[test]
fn equality_supports_all_matching_types() {
    assert_eq!(TypeALU::equal(Type::I64, Type::I64, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::equal(Type::F64, Type::F64, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::equal(Type::Str, Type::Str, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::equal(Type::Bool, Type::Bool, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn not_equal_supports_all_matching_types() {
    assert_eq!(TypeALU::not_equal(Type::I64, Type::I64, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::not_equal(Type::F64, Type::F64, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::not_equal(Type::Str, Type::Str, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::not_equal(Type::Bool, Type::Bool, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn equality_rejects_all_mixed_types() {
    assert!(TypeALU::equal(Type::I64, Type::F64, Span::default()).is_err());
    assert!(TypeALU::equal(Type::I64, Type::Str, Span::default()).is_err());
    assert!(TypeALU::equal(Type::I64, Type::Bool, Span::default()).is_err());
    assert!(TypeALU::equal(Type::F64, Type::Str, Span::default()).is_err());
    assert!(TypeALU::equal(Type::F64, Type::Bool, Span::default()).is_err());
    assert!(TypeALU::equal(Type::Str, Type::Bool, Span::default()).is_err());
}

#[test]
fn not_equal_rejects_all_mixed_types() {
    assert!(TypeALU::not_equal(Type::I64, Type::F64, Span::default()).is_err());
    assert!(TypeALU::not_equal(Type::I64, Type::Str, Span::default()).is_err());
    assert!(TypeALU::not_equal(Type::I64, Type::Bool, Span::default()).is_err());
    assert!(TypeALU::not_equal(Type::F64, Type::Str, Span::default()).is_err());
    assert!(TypeALU::not_equal(Type::F64, Type::Bool, Span::default()).is_err());
    assert!(TypeALU::not_equal(Type::Str, Type::Bool, Span::default()).is_err());
}
