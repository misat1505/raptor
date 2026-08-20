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

#[test]
fn ordered_comparisons_all_integer_sizes() {
    let ints = [Type::I8, Type::I16, Type::I32, Type::I64, Type::U8, Type::U16, Type::U32, Type::U64];
    for a in &ints {
        for b in &ints {
            // ordered comparisons allow any integer with any integer
            assert_eq!(TypeALU::greater(a.clone(), b.clone(), Span::default()).unwrap(), Type::Bool);
            assert_eq!(TypeALU::greater_or_equal(a.clone(), b.clone(), Span::default()).unwrap(), Type::Bool);
            assert_eq!(TypeALU::less(a.clone(), b.clone(), Span::default()).unwrap(), Type::Bool);
            assert_eq!(TypeALU::less_or_equal(a.clone(), b.clone(), Span::default()).unwrap(), Type::Bool);
        }
    }
}

#[test]
fn ordered_comparisons_reject_char_str_bool() {
    let bad = [Type::Char, Type::Str, Type::Bool];
    for t in bad {
        assert!(TypeALU::greater(t.clone(), t.clone(), Span::default()).is_err());
        assert!(TypeALU::greater_or_equal(t.clone(), t.clone(), Span::default()).is_err());
        assert!(TypeALU::less(t.clone(), t.clone(), Span::default()).is_err());
        assert!(TypeALU::less_or_equal(t.clone(), t.clone(), Span::default()).is_err());
    }
}

#[test]
fn equality_all_integer_and_char_combinations() {
    let types = [
        Type::I8,
        Type::I16,
        Type::I32,
        Type::I64,
        Type::U8,
        Type::U16,
        Type::U32,
        Type::U64,
        Type::Char,
    ];
    for a in &types {
        for b in &types {
            assert_eq!(TypeALU::equal(a.clone(), b.clone(), Span::default()).unwrap(), Type::Bool);
            assert_eq!(TypeALU::not_equal(a.clone(), b.clone(), Span::default()).unwrap(), Type::Bool);
        }
    }
}

#[test]
fn equality_supports_str_bool_f64() {
    assert_eq!(TypeALU::equal(Type::Str, Type::Str, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::equal(Type::Bool, Type::Bool, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::equal(Type::F64, Type::F64, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::not_equal(Type::Str, Type::Str, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::not_equal(Type::Bool, Type::Bool, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::not_equal(Type::F64, Type::F64, Span::default()).unwrap(), Type::Bool);
}

#[test]
fn equality_rejects_cross_category() {
    let bad_pairs = [
        (Type::I64, Type::F64),
        (Type::I64, Type::Str),
        (Type::I64, Type::Bool),
        (Type::Char, Type::F64),
        (Type::Char, Type::Str),
        (Type::Char, Type::Bool),
        (Type::F64, Type::Str),
        (Type::F64, Type::Bool),
        (Type::Str, Type::Bool),
        (Type::Vector(Box::new(Type::I64)), Type::I64),
    ];
    for (a, b) in bad_pairs {
        assert!(TypeALU::equal(a.clone(), b.clone(), Span::default()).is_err());
        assert!(TypeALU::not_equal(a, b, Span::default()).is_err());
    }
}

#[test]
fn ordered_reject_integer_vs_f64() {
    assert!(TypeALU::greater(Type::I32, Type::F64, Span::default()).is_err());
    assert!(TypeALU::less(Type::U64, Type::F64, Span::default()).is_err());
    assert!(TypeALU::greater_or_equal(Type::F64, Type::I8, Span::default()).is_err());
    assert!(TypeALU::less_or_equal(Type::F64, Type::U16, Span::default()).is_err());
}
