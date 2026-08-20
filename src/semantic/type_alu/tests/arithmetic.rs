use crate::{
    common::{errors::IError, span::Span, types::Type},
    semantic::type_alu::TypeALU,
};

#[test]
fn add_valid() {
    assert_eq!(TypeALU::add(Type::I64, Type::I64, Span::default()).unwrap(), Type::I64);
    assert_eq!(TypeALU::add(Type::F64, Type::F64, Span::default()).unwrap(), Type::F64);
    assert_eq!(TypeALU::add(Type::Str, Type::Str, Span::default()).unwrap(), Type::Str);
}

#[test]
fn add_invalid() {
    assert_eq!(
        TypeALU::add(Type::I64, Type::F64, Span::default()).err().unwrap().message(),
        "Cannot perform addition between values of type 'i64' and 'f64'."
    );
    assert_eq!(
        TypeALU::add(Type::Bool, Type::Bool, Span::default()).err().unwrap().message(),
        "Cannot perform addition between values of type 'bool' and 'bool'."
    );
}

#[test]
fn subtract_valid() {
    assert_eq!(TypeALU::subtract(Type::I64, Type::I64, Span::default()).unwrap(), Type::I64);
    assert_eq!(TypeALU::subtract(Type::F64, Type::F64, Span::default()).unwrap(), Type::F64);
}

#[test]
fn subtract_invalid() {
    assert_eq!(
        TypeALU::subtract(Type::Str, Type::Str, Span::default()).err().unwrap().message(),
        "Cannot perform subtraction between values of type 'str' and 'str'."
    );
}

#[test]
fn multiplication_valid() {
    assert_eq!(TypeALU::multiplication(Type::I64, Type::I64, Span::default()).unwrap(), Type::I64);
    assert_eq!(TypeALU::multiplication(Type::F64, Type::F64, Span::default()).unwrap(), Type::F64);
}

#[test]
fn multiplication_invalid() {
    assert_eq!(
        TypeALU::multiplication(Type::I64, Type::F64, Span::default()).err().unwrap().message(),
        "Cannot perform multiplication between values of type 'i64' and 'f64'."
    );
}

#[test]
fn division_valid() {
    assert_eq!(TypeALU::division(Type::I64, Type::I64, Span::default()).unwrap(), Type::I64);
    assert_eq!(TypeALU::division(Type::F64, Type::F64, Span::default()).unwrap(), Type::F64);
}

#[test]
fn division_invalid() {
    assert_eq!(
        TypeALU::division(Type::Bool, Type::I64, Span::default()).err().unwrap().message(),
        "Cannot perform division between values of type 'bool' and 'i64'."
    );
}

#[test]
fn add_supports_all_valid_type_pairs() {
    assert_eq!(TypeALU::add(Type::I64, Type::I64, Span::default()).unwrap(), Type::I64);
    assert_eq!(TypeALU::add(Type::F64, Type::F64, Span::default()).unwrap(), Type::F64);
    assert_eq!(TypeALU::add(Type::Str, Type::Str, Span::default()).unwrap(), Type::Str);
}

#[test]
fn add_rejects_mixed_numeric_types() {
    assert!(TypeALU::add(Type::I64, Type::F64, Span::default()).is_err());
    assert!(TypeALU::add(Type::F64, Type::I64, Span::default()).is_err());
}

#[test]
fn subtract_rejects_all_non_matching_types() {
    assert!(TypeALU::subtract(Type::I64, Type::F64, Span::default()).is_err());
    assert!(TypeALU::subtract(Type::F64, Type::I64, Span::default()).is_err());
    assert!(TypeALU::subtract(Type::Bool, Type::Bool, Span::default()).is_err());
    assert!(TypeALU::subtract(Type::Str, Type::Str, Span::default()).is_err());
}

#[test]
fn multiplication_rejects_all_non_matching_types() {
    assert!(TypeALU::multiplication(Type::I64, Type::F64, Span::default()).is_err());
    assert!(TypeALU::multiplication(Type::F64, Type::I64, Span::default()).is_err());
    assert!(TypeALU::multiplication(Type::Bool, Type::Bool, Span::default()).is_err());
    assert!(TypeALU::multiplication(Type::Str, Type::Str, Span::default()).is_err());
}

#[test]
fn division_rejects_all_non_matching_types() {
    assert!(TypeALU::division(Type::I64, Type::F64, Span::default()).is_err());
    assert!(TypeALU::division(Type::F64, Type::I64, Span::default()).is_err());
    assert!(TypeALU::division(Type::Bool, Type::Bool, Span::default()).is_err());
    assert!(TypeALU::division(Type::Str, Type::Str, Span::default()).is_err());
}

#[test]
fn modulo_valid() {
    assert_eq!(TypeALU::modulo(Type::I64, Type::I64, Span::default()).unwrap(), Type::I64);
}

#[test]
fn modulo_invalid() {
    assert_eq!(
        TypeALU::modulo(Type::F64, Type::F64, Span::default()).err().unwrap().message(),
        "Cannot perform modulo between values of type 'f64' and 'f64'."
    );

    assert_eq!(
        TypeALU::modulo(Type::I64, Type::F64, Span::default()).err().unwrap().message(),
        "Cannot perform modulo between values of type 'i64' and 'f64'."
    );

    assert_eq!(
        TypeALU::modulo(Type::Bool, Type::Bool, Span::default()).err().unwrap().message(),
        "Cannot perform modulo between values of type 'bool' and 'bool'."
    );
}

#[test]
fn add_all_integer_sizes() {
    let ints = [Type::I8, Type::I16, Type::I32, Type::I64, Type::U8, Type::U16, Type::U32, Type::U64];
    for t in ints {
        assert_eq!(TypeALU::add(t.clone(), t.clone(), Span::default()).unwrap(), t);
    }
}

#[test]
fn add_string_and_char_combinations() {
    assert_eq!(TypeALU::add(Type::Str, Type::Str, Span::default()).unwrap(), Type::Str);
    assert_eq!(TypeALU::add(Type::Char, Type::Char, Span::default()).unwrap(), Type::Str);
    assert_eq!(TypeALU::add(Type::Str, Type::Char, Span::default()).unwrap(), Type::Str);
    assert_eq!(TypeALU::add(Type::Char, Type::Str, Span::default()).unwrap(), Type::Str);
}

#[test]
fn add_rejects_unsupported() {
    let bad_pairs = [
        (Type::I64, Type::Str),
        (Type::Str, Type::I64),
        (Type::Bool, Type::I64),
        (Type::Char, Type::I64),
        (Type::F64, Type::Str),
        (Type::U8, Type::I8),
    ];
    for (a, b) in bad_pairs {
        assert!(TypeALU::add(a, b, Span::default()).is_err());
    }
}

#[test]
fn subtract_all_numeric_sizes() {
    let nums = [
        Type::I8,
        Type::I16,
        Type::I32,
        Type::I64,
        Type::U8,
        Type::U16,
        Type::U32,
        Type::U64,
        Type::F64,
    ];
    for t in nums {
        assert_eq!(TypeALU::subtract(t.clone(), t.clone(), Span::default()).unwrap(), t);
    }
}

#[test]
fn multiplication_all_numeric_sizes() {
    let nums = [
        Type::I8,
        Type::I16,
        Type::I32,
        Type::I64,
        Type::U8,
        Type::U16,
        Type::U32,
        Type::U64,
        Type::F64,
    ];
    for t in nums {
        assert_eq!(TypeALU::multiplication(t.clone(), t.clone(), Span::default()).unwrap(), t);
    }
}

#[test]
fn division_all_numeric_sizes() {
    let nums = [
        Type::I8,
        Type::I16,
        Type::I32,
        Type::I64,
        Type::U8,
        Type::U16,
        Type::U32,
        Type::U64,
        Type::F64,
    ];
    for t in nums {
        assert_eq!(TypeALU::division(t.clone(), t.clone(), Span::default()).unwrap(), t);
    }
}

#[test]
fn modulo_all_integer_sizes() {
    let ints = [Type::I8, Type::I16, Type::I32, Type::I64, Type::U8, Type::U16, Type::U32, Type::U64];
    for t in ints {
        assert_eq!(TypeALU::modulo(t.clone(), t.clone(), Span::default()).unwrap(), t);
    }
}

#[test]
fn modulo_rejects_float_and_non_integers() {
    assert!(TypeALU::modulo(Type::F64, Type::F64, Span::default()).is_err());
    assert!(TypeALU::modulo(Type::Str, Type::Str, Span::default()).is_err());
    assert!(TypeALU::modulo(Type::Bool, Type::Bool, Span::default()).is_err());
    assert!(TypeALU::modulo(Type::Char, Type::Char, Span::default()).is_err());
    assert!(TypeALU::modulo(Type::I64, Type::U64, Span::default()).is_err());
}

#[test]
fn numeric_ops_reject_mismatched_sizes() {
    // different integer widths are not allowed
    assert!(TypeALU::subtract(Type::I8, Type::I16, Span::default()).is_err());
    assert!(TypeALU::multiplication(Type::U32, Type::U64, Span::default()).is_err());
    assert!(TypeALU::division(Type::I32, Type::U32, Span::default()).is_err());
    assert!(TypeALU::modulo(Type::I64, Type::I32, Span::default()).is_err());
}

#[test]
fn check_numeric_operation_directly() {
    assert_eq!(
        TypeALU::check_numeric_operation(Type::I32, Type::I32, "test", Span::default()).unwrap(),
        Type::I32
    );
    assert!(TypeALU::check_numeric_operation(Type::I32, Type::F64, "test", Span::default()).is_err());
    assert!(TypeALU::check_numeric_operation(Type::Str, Type::Str, "test", Span::default()).is_err());
}
