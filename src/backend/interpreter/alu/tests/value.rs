use std::{assert_eq, cell::RefCell, rc::Rc, vec};

use crate::{
    backend::interpreter::Value,
    common::{errors::IError, span::Span, types::Type},
};

#[test]
fn default_values() {
    let data = [Type::Bool, Type::I64, Type::F64, Type::Str];

    let expected = [Value::Bool(false), Value::I64(0), Value::F64(0.0), Value::String(String::from(""))];

    for idx in 0..data.len() {
        assert_eq!(Value::default_value(&data[idx], Span::default()).unwrap(), expected[idx]);
    }
}

#[test]
fn default_values_fail() {
    assert_eq!(
        Value::default_value(&Type::Void, Span::default()).err().unwrap().message(),
        String::from("Cannot create default value for type 'void'.")
    );
}

#[test]
fn value_to_type() {
    let values = [Value::Bool(true), Value::I64(5), Value::F64(5.5), Value::String(String::from("hello"))];

    let exp = [Type::Bool, Type::I64, Type::F64, Type::Str];

    for idx in 0..values.len() {
        assert_eq!(values[idx].to_type(), exp[idx]);
    }
}

#[test]
fn try_into_bool() {
    assert_eq!(Value::Bool(true).try_into_bool(Span::default()).unwrap(), true);
    assert_eq!(
        Value::I64(5).try_into_bool(Span::default()).err().unwrap().message(),
        String::from("Given value is not a boolean.")
    );
}

#[test]
fn default_vector_value() {
    let value = Value::default_value(&Type::Vector(Box::new(Type::I64)), Span::default()).unwrap();

    assert_eq!(
        value,
        Value::Vector {
            kind: Box::new(Type::Vector(Box::new(Type::I64))),
            values: Rc::new(RefCell::new(vec![])),
        }
    );
}

#[test]
fn default_vector_preserves_inner_type() {
    let types = [Type::Bool, Type::I64, Type::F64, Type::Str, Type::Vector(Box::new(Type::I64))];

    for inner_type in types {
        let vector_type = Type::Vector(Box::new(inner_type.clone()));
        let value = Value::default_value(&vector_type, Span::default()).unwrap();

        assert_eq!(value.to_type(), vector_type);
    }
}

#[test]
fn default_vector_is_empty() {
    let ty = Type::Vector(Box::new(Type::F64));
    let value = Value::default_value(&ty, Span::default()).unwrap();

    match value {
        Value::Vector { kind, values } => {
            assert_eq!(*kind, ty);
            assert!(values.borrow().is_empty());
        }
        _ => panic!("Expected vector value."),
    }
}

#[test]
fn default_values_fail_for_unsupported_types() {
    assert!(Value::default_value(&Type::Void, Span::default()).is_err());
}

#[test]
fn value_to_type_vector() {
    let value = Value::Vector {
        kind: Box::new(Type::I64),
        values: Rc::new(RefCell::new(vec![])),
    };

    assert_eq!(value.to_type(), Type::I64);
}

#[test]
fn value_to_type_nested_vector() {
    let value = Value::Vector {
        kind: Box::new(Type::Vector(Box::new(Type::I64))),
        values: Rc::new(RefCell::new(vec![])),
    };

    assert_eq!(value.to_type(), Type::Vector(Box::new(Type::I64)));
}

#[test]
fn value_to_type_does_not_depend_on_vector_contents() {
    let values = Rc::new(RefCell::new(vec![
        Rc::new(RefCell::new(Value::I64(1))),
        Rc::new(RefCell::new(Value::I64(2))),
    ]));

    let value = Value::Vector {
        kind: Box::new(Type::I64),
        values,
    };

    assert_eq!(value.to_type(), Type::I64);
}

#[test]
fn try_into_bool_accepts_true() {
    assert_eq!(Value::Bool(true).try_into_bool(Span::default()).unwrap(), true);
}

#[test]
fn try_into_bool_accepts_false() {
    assert_eq!(Value::Bool(false).try_into_bool(Span::default()).unwrap(), false);
}

#[test]
fn try_into_bool_rejects_i64() {
    assert_eq!(
        Value::I64(0).try_into_bool(Span::default()).err().unwrap().message(),
        "Given value is not a boolean."
    );
}

#[test]
fn try_into_bool_rejects_f64() {
    assert_eq!(
        Value::F64(0.0).try_into_bool(Span::default()).err().unwrap().message(),
        "Given value is not a boolean."
    );
}

#[test]
fn try_into_bool_rejects_string() {
    assert_eq!(
        Value::String(String::from("true"))
            .try_into_bool(Span::default())
            .err()
            .unwrap()
            .message(),
        "Given value is not a boolean."
    );
}

#[test]
fn try_into_bool_rejects_vector() {
    let value = Value::Vector {
        kind: Box::new(Type::Bool),
        values: Rc::new(RefCell::new(vec![])),
    };

    assert_eq!(
        value.try_into_bool(Span::default()).err().unwrap().message(),
        "Given value is not a boolean."
    );
}

#[test]
fn vector_values_can_be_mutated_through_shared_reference() {
    let values = Rc::new(RefCell::new(vec![Rc::new(RefCell::new(Value::I64(1)))]));

    let value = Value::Vector {
        kind: Box::new(Type::I64),
        values: values.clone(),
    };

    if let Value::Vector { values, .. } = value {
        values.borrow_mut().push(Rc::new(RefCell::new(Value::I64(2))));
    }

    assert_eq!(values.borrow().len(), 2);
}

#[test]
fn vector_kind_is_independent_from_values_length() {
    let value = Value::Vector {
        kind: Box::new(Type::Str),
        values: Rc::new(RefCell::new(vec![])),
    };

    assert_eq!(value.to_type(), Type::Str);
}

#[test]
fn scalar_values_have_correct_types() {
    assert_eq!(Value::Bool(false).to_type(), Type::Bool);
    assert_eq!(Value::I64(0).to_type(), Type::I64);
    assert_eq!(Value::F64(0.0).to_type(), Type::F64);
    assert_eq!(Value::String(String::new()).to_type(), Type::Str);
}

#[test]
fn default_values_all_integer_types() {
    assert_eq!(Value::default_value(&Type::I8, Span::default()).unwrap(), Value::I8(0));
    assert_eq!(Value::default_value(&Type::I16, Span::default()).unwrap(), Value::I16(0));
    assert_eq!(Value::default_value(&Type::I32, Span::default()).unwrap(), Value::I32(0));
    assert_eq!(Value::default_value(&Type::U8, Span::default()).unwrap(), Value::U8(0));
    assert_eq!(Value::default_value(&Type::U16, Span::default()).unwrap(), Value::U16(0));
    assert_eq!(Value::default_value(&Type::U32, Span::default()).unwrap(), Value::U32(0));
    assert_eq!(Value::default_value(&Type::U64, Span::default()).unwrap(), Value::U64(0));
}

#[test]
fn default_value_char_fails() {
    let r = Value::default_value(&Type::Char, Span::default());
    assert!(r.is_err());
    assert_eq!(r.err().unwrap().message(), "Cannot create default value for type 'char'.");
}

#[test]
fn value_to_type_all_scalars() {
    assert_eq!(Value::I8(0).to_type(), Type::I8);
    assert_eq!(Value::I16(0).to_type(), Type::I16);
    assert_eq!(Value::I32(0).to_type(), Type::I32);
    assert_eq!(Value::U8(0).to_type(), Type::U8);
    assert_eq!(Value::U16(0).to_type(), Type::U16);
    assert_eq!(Value::U32(0).to_type(), Type::U32);
    assert_eq!(Value::U64(0).to_type(), Type::U64);
    assert_eq!(Value::Char('a').to_type(), Type::Char);
}

#[test]
fn try_into_bool_rejects_all_non_bool() {
    let non_bools = [
        Value::I8(0),
        Value::I16(0),
        Value::I32(0),
        Value::U8(0),
        Value::U16(0),
        Value::U32(0),
        Value::U64(0),
        Value::Char('t'),
    ];
    for v in non_bools {
        let r = v.try_into_bool(Span::default());
        assert!(r.is_err());
        assert_eq!(r.err().unwrap().message(), "Given value is not a boolean.");
    }
}

#[test]
fn default_nested_vector() {
    let nested = Type::Vector(Box::new(Type::Vector(Box::new(Type::Bool))));
    let value = Value::default_value(&nested, Span::default()).unwrap();
    match value {
        Value::Vector { kind, values } => {
            assert_eq!(*kind, nested);
            assert!(values.borrow().is_empty());
            // to_type returns the full vector type (kind)
            assert_eq!(
                Value::Vector {
                    kind: kind.clone(),
                    values: values.clone()
                }
                .to_type(),
                nested
            );
        }
        _ => panic!("expected Vector"),
    }
}
