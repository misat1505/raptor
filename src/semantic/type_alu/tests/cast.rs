use crate::{
    common::{errors::IError, span::Span, types::Type},
    semantic::type_alu::TypeALU,
};

#[test]
fn cast_to_type_valid() {
    assert_eq!(TypeALU::cast_to_type(Type::I64, &Type::Str, Span::default()).unwrap(), Type::Str);
    assert_eq!(TypeALU::cast_to_type(Type::F64, &Type::Str, Span::default()).unwrap(), Type::Str);
    assert_eq!(TypeALU::cast_to_type(Type::I64, &Type::F64, Span::default()).unwrap(), Type::F64);
    assert_eq!(TypeALU::cast_to_type(Type::F64, &Type::I64, Span::default()).unwrap(), Type::I64);
    assert_eq!(TypeALU::cast_to_type(Type::I64, &Type::Bool, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::cast_to_type(Type::F64, &Type::Bool, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::cast_to_type(Type::Str, &Type::I64, Span::default()).unwrap(), Type::I64);
    assert_eq!(TypeALU::cast_to_type(Type::Str, &Type::F64, Span::default()).unwrap(), Type::F64);
    assert_eq!(TypeALU::cast_to_type(Type::Str, &Type::Bool, Span::default()).unwrap(), Type::Bool);
    assert_eq!(TypeALU::cast_to_type(Type::Bool, &Type::Str, Span::default()).unwrap(), Type::Str);
    assert_eq!(TypeALU::cast_to_type(Type::Bool, &Type::I64, Span::default()).unwrap(), Type::I64);
    assert_eq!(TypeALU::cast_to_type(Type::Bool, &Type::F64, Span::default()).unwrap(), Type::F64);
}

#[test]
fn cast_to_type_invalid() {
    assert_eq!(
        TypeALU::cast_to_type(Type::Vector(Box::new(Type::I64)), &Type::I64, Span::default())
            .err()
            .unwrap()
            .message(),
        "Cannot cast 'i64[]' to 'i64'."
    );
    assert_eq!(
        TypeALU::cast_to_type(Type::Void, &Type::I64, Span::default()).err().unwrap().message(),
        "Cannot cast 'void' to 'i64'."
    );
}

#[test]
fn cast_to_type_same_type_is_invalid() {
    assert_eq!(
        TypeALU::cast_to_type(Type::I64, &Type::I64, Span::default()).err().unwrap().message(),
        "Cannot cast 'i64' to 'i64'."
    );

    assert_eq!(
        TypeALU::cast_to_type(Type::F64, &Type::F64, Span::default()).err().unwrap().message(),
        "Cannot cast 'f64' to 'f64'."
    );

    assert_eq!(
        TypeALU::cast_to_type(Type::Bool, &Type::Bool, Span::default()).err().unwrap().message(),
        "Cannot cast 'bool' to 'bool'."
    );

    assert_eq!(
        TypeALU::cast_to_type(Type::Str, &Type::Str, Span::default()).err().unwrap().message(),
        "Cannot cast 'str' to 'str'."
    );
}

#[test]
fn cast_vector_to_supported_types_is_invalid() {
    assert_eq!(
        TypeALU::cast_to_type(Type::Vector(Box::new(Type::I64)), &Type::Str, Span::default())
            .err()
            .unwrap()
            .message(),
        "Cannot cast 'i64[]' to 'str'."
    );

    assert_eq!(
        TypeALU::cast_to_type(Type::Vector(Box::new(Type::I64)), &Type::Bool, Span::default())
            .err()
            .unwrap()
            .message(),
        "Cannot cast 'i64[]' to 'bool'."
    );
}
