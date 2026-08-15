use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    common::errors::{ComputationError, ErrorSeverity},
    frontend::ast::Type,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    I64(i64),
    F64(f64),
    String(String),
    Bool(bool),
    Vector {
        kind: Box<Type>,
        values: Rc<RefCell<Vec<Rc<RefCell<Value>>>>>,
    },
}

impl Value {
    pub fn default_value(var_type: &Type) -> Result<Value, ComputationError> {
        match var_type {
            Type::Bool => Ok(Value::Bool(false)),
            Type::I64 => Ok(Value::I64(0)),
            Type::F64 => Ok(Value::F64(0.0)),
            Type::Str => Ok(Value::String("".to_owned())),
            Type::Vector(inner) => Ok(Value::Vector {
                kind: Box::new((**inner).clone()),
                values: Rc::new(RefCell::new(vec![])),
            }),
            other => Err(ComputationError::new(
                ErrorSeverity::HIGH,
                format!("Cannot create default value for type '{:?}'.", other),
            )),
        }
    }

    pub fn to_type(&self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::F64(_) => Type::F64,
            Value::I64(_) => Type::I64,
            Value::String(_) => Type::Str,
            Value::Vector { kind, .. } => kind.as_ref().clone(),
        }
    }

    pub fn try_into_bool(&self) -> Result<bool, ComputationError> {
        match self {
            Value::Bool(bool) => Ok(*bool),
            _ => Err(ComputationError::new(ErrorSeverity::HIGH, String::from("Given value is not a boolean."))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::IError;

    use super::*;

    #[test]
    fn default_values() {
        let data = [Type::Bool, Type::I64, Type::F64, Type::Str];

        let expected = [Value::Bool(false), Value::I64(0), Value::F64(0.0), Value::String(String::from(""))];

        for idx in 0..data.len() {
            assert_eq!(Value::default_value(&data[idx]).unwrap(), expected[idx]);
        }
    }

    #[test]
    fn default_values_fail() {
        assert_eq!(
            Value::default_value(&Type::Void).err().unwrap().message(),
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
        assert_eq!(Value::Bool(true).try_into_bool().unwrap(), true);
        assert_eq!(
            Value::I64(5).try_into_bool().err().unwrap().message(),
            String::from("Given value is not a boolean.")
        );
    }

    #[test]
    fn default_vector_value() {
        let value = Value::default_value(&Type::Vector(Box::new(Type::I64))).unwrap();

        assert_eq!(
            value,
            Value::Vector {
                kind: Box::new(Type::I64),
                values: Rc::new(RefCell::new(vec![])),
            }
        );
    }

    #[test]
    fn default_vector_preserves_inner_type() {
        let types = [Type::Bool, Type::I64, Type::F64, Type::Str, Type::Vector(Box::new(Type::I64))];

        for inner_type in types {
            let vector_type = Type::Vector(Box::new(inner_type.clone()));
            let value = Value::default_value(&vector_type).unwrap();

            assert_eq!(value.to_type(), inner_type);
        }
    }

    #[test]
    fn default_vector_is_empty() {
        let value = Value::default_value(&Type::Vector(Box::new(Type::F64))).unwrap();

        match value {
            Value::Vector { kind, values } => {
                assert_eq!(*kind, Type::F64);
                assert!(values.borrow().is_empty());
            }
            _ => panic!("Expected vector value."),
        }
    }

    #[test]
    fn default_values_fail_for_unsupported_types() {
        assert!(Value::default_value(&Type::Void).is_err());
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
        assert_eq!(Value::Bool(true).try_into_bool().unwrap(), true);
    }

    #[test]
    fn try_into_bool_accepts_false() {
        assert_eq!(Value::Bool(false).try_into_bool().unwrap(), false);
    }

    #[test]
    fn try_into_bool_rejects_i64() {
        assert_eq!(Value::I64(0).try_into_bool().err().unwrap().message(), "Given value is not a boolean.");
    }

    #[test]
    fn try_into_bool_rejects_f64() {
        assert_eq!(Value::F64(0.0).try_into_bool().err().unwrap().message(), "Given value is not a boolean.");
    }

    #[test]
    fn try_into_bool_rejects_string() {
        assert_eq!(
            Value::String(String::from("true")).try_into_bool().err().unwrap().message(),
            "Given value is not a boolean."
        );
    }

    #[test]
    fn try_into_bool_rejects_vector() {
        let value = Value::Vector {
            kind: Box::new(Type::Bool),
            values: Rc::new(RefCell::new(vec![])),
        };

        assert_eq!(value.try_into_bool().err().unwrap().message(), "Given value is not a boolean.");
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
}
