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
