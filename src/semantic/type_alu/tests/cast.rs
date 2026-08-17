use crate::{
    common::{
        errors::IError,
        types::Type,
    },
    semantic::type_alu::TypeALU,
};

#[test]
fn cast_to_type_valid() {
    assert_eq!(TypeALU::cast_to_type(Type::I64, &Type::Str).unwrap(), Type::Str);
    assert_eq!(TypeALU::cast_to_type(Type::F64, &Type::Str).unwrap(), Type::Str);
    assert_eq!(TypeALU::cast_to_type(Type::I64, &Type::F64).unwrap(), Type::F64);
    assert_eq!(TypeALU::cast_to_type(Type::F64, &Type::I64).unwrap(), Type::I64);
    assert_eq!(TypeALU::cast_to_type(Type::I64, &Type::Bool).unwrap(), Type::Bool);
    assert_eq!(TypeALU::cast_to_type(Type::F64, &Type::Bool).unwrap(), Type::Bool);
    assert_eq!(TypeALU::cast_to_type(Type::Str, &Type::I64).unwrap(), Type::I64);
    assert_eq!(TypeALU::cast_to_type(Type::Str, &Type::F64).unwrap(), Type::F64);
    assert_eq!(TypeALU::cast_to_type(Type::Str, &Type::Bool).unwrap(), Type::Bool);
    assert_eq!(TypeALU::cast_to_type(Type::Bool, &Type::Str).unwrap(), Type::Str);
    assert_eq!(TypeALU::cast_to_type(Type::Bool, &Type::I64).unwrap(), Type::I64);
    assert_eq!(TypeALU::cast_to_type(Type::Bool, &Type::F64).unwrap(), Type::F64);
}

#[test]
fn cast_to_type_invalid() {
    assert_eq!(
        TypeALU::cast_to_type(Type::Vector(Box::new(Type::I64)), &Type::I64)
            .err()
            .unwrap()
            .message(),
        "Cannot cast 'i64[]' to 'i64'."
    );
    assert_eq!(
        TypeALU::cast_to_type(Type::Void, &Type::I64).err().unwrap().message(),
        "Cannot cast 'void' to 'i64'."
    );
}

#[test]
fn cast_to_type_same_type_is_invalid() {
    assert_eq!(
        TypeALU::cast_to_type(Type::I64, &Type::I64).err().unwrap().message(),
        "Cannot cast 'i64' to 'i64'."
    );

    assert_eq!(
        TypeALU::cast_to_type(Type::F64, &Type::F64).err().unwrap().message(),
        "Cannot cast 'f64' to 'f64'."
    );

    assert_eq!(
        TypeALU::cast_to_type(Type::Bool, &Type::Bool).err().unwrap().message(),
        "Cannot cast 'bool' to 'bool'."
    );

    assert_eq!(
        TypeALU::cast_to_type(Type::Str, &Type::Str).err().unwrap().message(),
        "Cannot cast 'str' to 'str'."
    );
}

#[test]
fn cast_vector_to_supported_types_is_invalid() {
    assert_eq!(
        TypeALU::cast_to_type(Type::Vector(Box::new(Type::I64)), &Type::Str)
            .err()
            .unwrap()
            .message(),
        "Cannot cast 'i64[]' to 'str'."
    );

    assert_eq!(
        TypeALU::cast_to_type(Type::Vector(Box::new(Type::I64)), &Type::Bool)
            .err()
            .unwrap()
            .message(),
        "Cannot cast 'i64[]' to 'bool'."
    );
}
