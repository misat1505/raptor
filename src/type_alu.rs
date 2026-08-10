use crate::{
    ast::Type,
    errors::{ErrorSeverity, SemanticCheckerError},
};

pub struct TypeALU;

impl TypeALU {
    fn check_numeric_operation(t1: Type, t2: Type, op_name: &str) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::I64, Type::I64) => Ok(Type::I64),
            (Type::F64, Type::F64) => Ok(Type::F64),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform {} between values of type '{:?}' and '{:?}'.", op_name, a, b),
            )),
        }
    }
}

impl TypeALU {
    pub fn cast_to_type(from: Type, to_type: &Type) -> Result<Type, SemanticCheckerError> {
        match (from, to_type) {
            (Type::I64, Type::Str) => Ok(Type::Str),
            (Type::F64, Type::Str) => Ok(Type::Str),
            (Type::I64, Type::F64) => Ok(Type::F64),
            (Type::F64, Type::I64) => Ok(Type::I64),
            (Type::I64, Type::Bool) => Ok(Type::Bool),
            (Type::F64, Type::Bool) => Ok(Type::Bool),
            (Type::Str, Type::I64) => Ok(Type::I64),
            (Type::Str, Type::F64) => Ok(Type::F64),
            (Type::Str, Type::Bool) => Ok(Type::Bool),
            (Type::Bool, Type::Str) => Ok(Type::Str),
            (Type::Bool, Type::I64) => Ok(Type::I64),
            (Type::Bool, Type::F64) => Ok(Type::F64),
            (value, target_type) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot cast '{:?}' to '{:?}'.", value, target_type),
            )),
        }
    }

    pub fn boolean_negate(t: Type) -> Result<Type, SemanticCheckerError> {
        match t {
            Type::Bool => Ok(Type::Bool),
            t => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform boolean negation on type '{:?}'.", t),
            )),
        }
    }

    pub fn arithmetic_negate(t: Type) -> Result<Type, SemanticCheckerError> {
        match t {
            Type::I64 => Ok(Type::I64),
            Type::F64 => Ok(Type::F64),
            t => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform arithmetic negation on type '{:?}'.", t),
            )),
        }
    }

    pub fn add(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::I64, Type::I64) => Ok(Type::I64),
            (Type::F64, Type::F64) => Ok(Type::F64),
            (Type::Str, Type::Str) => Ok(Type::Str),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform addition between values of type '{:?}' and '{:?}'.", a, b),
            )),
        }
    }

    pub fn subtract(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_numeric_operation(t1, t2, "subtraction")
    }

    pub fn multiplication(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_numeric_operation(t1, t2, "multiplication")
    }

    pub fn division(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_numeric_operation(t1, t2, "division")
    }

    pub fn modulo(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::I64, Type::I64) => Ok(Type::I64),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform modulo between values of type '{:?}' and '{:?}'.", a, b),
            )),
        }
    }

    pub fn concatenation(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::Bool, Type::Bool) => Ok(Type::Bool),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform concatenation between values of type '{:?}' and '{:?}'.", a, b),
            )),
        }
    }

    pub fn alternative(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::Bool, Type::Bool) => Ok(Type::Bool),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform alternative between values of type '{:?}' and '{:?}'.", a, b),
            )),
        }
    }

    pub fn greater(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_comparison(t1, t2, "greater")
    }

    pub fn greater_or_equal(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_comparison(t1, t2, "greater or equal")
    }

    pub fn less(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_comparison(t1, t2, "less")
    }

    pub fn less_or_equal(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_comparison(t1, t2, "less or equal")
    }

    fn check_comparison(t1: Type, t2: Type, op_name: &str) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::I64, Type::I64) => Ok(Type::Bool),
            (Type::F64, Type::F64) => Ok(Type::Bool),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform {} between values of type '{:?}' and '{:?}'.", op_name, a, b),
            )),
        }
    }

    pub fn equal(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_equality(t1, t2, "equal")
    }

    pub fn not_equal(t1: Type, t2: Type) -> Result<Type, SemanticCheckerError> {
        Self::check_equality(t1, t2, "not equal")
    }

    fn check_equality(t1: Type, t2: Type, op_name: &str) -> Result<Type, SemanticCheckerError> {
        match (t1, t2) {
            (Type::I64, Type::I64) | (Type::F64, Type::F64) | (Type::Str, Type::Str) | (Type::Bool, Type::Bool) => Ok(Type::Bool),
            (a, b) => Err(SemanticCheckerError::new(
                ErrorSeverity::HIGH,
                format!("Cannot perform {} between values of type '{:?}' and '{:?}'.", op_name, a, b),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::IError;

    use super::*;

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
    fn boolean_negate_valid() {
        assert_eq!(TypeALU::boolean_negate(Type::Bool).unwrap(), Type::Bool);
    }

    #[test]
    fn boolean_negate_invalid() {
        assert_eq!(
            TypeALU::boolean_negate(Type::I64).err().unwrap().message(),
            "Cannot perform boolean negation on type 'i64'."
        );
    }

    #[test]
    fn arithmetic_negate_valid() {
        assert_eq!(TypeALU::arithmetic_negate(Type::I64).unwrap(), Type::I64);
        assert_eq!(TypeALU::arithmetic_negate(Type::F64).unwrap(), Type::F64);
    }

    #[test]
    fn arithmetic_negate_invalid() {
        assert_eq!(
            TypeALU::arithmetic_negate(Type::Str).err().unwrap().message(),
            "Cannot perform arithmetic negation on type 'str'."
        );
    }

    #[test]
    fn add_valid() {
        assert_eq!(TypeALU::add(Type::I64, Type::I64).unwrap(), Type::I64);
        assert_eq!(TypeALU::add(Type::F64, Type::F64).unwrap(), Type::F64);
        assert_eq!(TypeALU::add(Type::Str, Type::Str).unwrap(), Type::Str);
    }

    #[test]
    fn add_invalid() {
        assert_eq!(
            TypeALU::add(Type::I64, Type::F64).err().unwrap().message(),
            "Cannot perform addition between values of type 'i64' and 'f64'."
        );
        assert_eq!(
            TypeALU::add(Type::Bool, Type::Bool).err().unwrap().message(),
            "Cannot perform addition between values of type 'bool' and 'bool'."
        );
    }

    #[test]
    fn subtract_valid() {
        assert_eq!(TypeALU::subtract(Type::I64, Type::I64).unwrap(), Type::I64);
        assert_eq!(TypeALU::subtract(Type::F64, Type::F64).unwrap(), Type::F64);
    }

    #[test]
    fn subtract_invalid() {
        assert_eq!(
            TypeALU::subtract(Type::Str, Type::Str).err().unwrap().message(),
            "Cannot perform subtraction between values of type 'str' and 'str'."
        );
    }

    #[test]
    fn multiplication_valid() {
        assert_eq!(TypeALU::multiplication(Type::I64, Type::I64).unwrap(), Type::I64);
        assert_eq!(TypeALU::multiplication(Type::F64, Type::F64).unwrap(), Type::F64);
    }

    #[test]
    fn multiplication_invalid() {
        assert_eq!(
            TypeALU::multiplication(Type::I64, Type::F64).err().unwrap().message(),
            "Cannot perform multiplication between values of type 'i64' and 'f64'."
        );
    }

    #[test]
    fn division_valid() {
        assert_eq!(TypeALU::division(Type::I64, Type::I64).unwrap(), Type::I64);
        assert_eq!(TypeALU::division(Type::F64, Type::F64).unwrap(), Type::F64);
    }

    #[test]
    fn division_invalid() {
        assert_eq!(
            TypeALU::division(Type::Bool, Type::I64).err().unwrap().message(),
            "Cannot perform division between values of type 'bool' and 'i64'."
        );
    }

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
    fn greater_valid() {
        assert_eq!(TypeALU::greater(Type::I64, Type::I64).unwrap(), Type::Bool);
        assert_eq!(TypeALU::greater(Type::F64, Type::F64).unwrap(), Type::Bool);
    }

    #[test]
    fn greater_invalid() {
        assert_eq!(
            TypeALU::greater(Type::I64, Type::F64).err().unwrap().message(),
            "Cannot perform greater between values of type 'i64' and 'f64'."
        );
    }

    #[test]
    fn greater_or_equal_valid() {
        assert_eq!(TypeALU::greater_or_equal(Type::I64, Type::I64).unwrap(), Type::Bool);
    }

    #[test]
    fn greater_or_equal_invalid() {
        assert_eq!(
            TypeALU::greater_or_equal(Type::Str, Type::Str).err().unwrap().message(),
            "Cannot perform greater or equal between values of type 'str' and 'str'."
        );
    }

    #[test]
    fn less_valid() {
        assert_eq!(TypeALU::less(Type::I64, Type::I64).unwrap(), Type::Bool);
    }

    #[test]
    fn less_invalid() {
        assert_eq!(
            TypeALU::less(Type::Bool, Type::Bool).err().unwrap().message(),
            "Cannot perform less between values of type 'bool' and 'bool'."
        );
    }

    #[test]
    fn less_or_equal_valid() {
        assert_eq!(TypeALU::less_or_equal(Type::F64, Type::F64).unwrap(), Type::Bool);
    }

    #[test]
    fn less_or_equal_invalid() {
        assert_eq!(
            TypeALU::less_or_equal(Type::I64, Type::Str).err().unwrap().message(),
            "Cannot perform less or equal between values of type 'i64' and 'str'."
        );
    }

    #[test]
    fn equal_valid() {
        assert_eq!(TypeALU::equal(Type::I64, Type::I64).unwrap(), Type::Bool);
        assert_eq!(TypeALU::equal(Type::F64, Type::F64).unwrap(), Type::Bool);
        assert_eq!(TypeALU::equal(Type::Str, Type::Str).unwrap(), Type::Bool);
        assert_eq!(TypeALU::equal(Type::Bool, Type::Bool).unwrap(), Type::Bool);
    }

    #[test]
    fn equal_invalid() {
        assert_eq!(
            TypeALU::equal(Type::I64, Type::Str).err().unwrap().message(),
            "Cannot perform equal between values of type 'i64' and 'str'."
        );
    }

    #[test]
    fn not_equal_valid() {
        assert_eq!(TypeALU::not_equal(Type::I64, Type::I64).unwrap(), Type::Bool);
    }

    #[test]
    fn not_equal_invalid() {
        assert_eq!(
            TypeALU::not_equal(Type::Bool, Type::I64).err().unwrap().message(),
            "Cannot perform not equal between values of type 'bool' and 'i64'."
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

    #[test]
    fn boolean_negate_rejects_all_non_bool_types() {
        for ty in [Type::I64, Type::F64, Type::Str, Type::Void] {
            assert!(TypeALU::boolean_negate(ty).is_err());
        }
    }

    #[test]
    fn arithmetic_negate_rejects_all_non_numeric_types() {
        for ty in [Type::Bool, Type::Str, Type::Void] {
            assert!(TypeALU::arithmetic_negate(ty).is_err());
        }
    }

    #[test]
    fn add_supports_all_valid_type_pairs() {
        assert_eq!(TypeALU::add(Type::I64, Type::I64).unwrap(), Type::I64);
        assert_eq!(TypeALU::add(Type::F64, Type::F64).unwrap(), Type::F64);
        assert_eq!(TypeALU::add(Type::Str, Type::Str).unwrap(), Type::Str);
    }

    #[test]
    fn add_rejects_mixed_numeric_types() {
        assert!(TypeALU::add(Type::I64, Type::F64).is_err());
        assert!(TypeALU::add(Type::F64, Type::I64).is_err());
    }

    #[test]
    fn subtract_rejects_all_non_matching_types() {
        assert!(TypeALU::subtract(Type::I64, Type::F64).is_err());
        assert!(TypeALU::subtract(Type::F64, Type::I64).is_err());
        assert!(TypeALU::subtract(Type::Bool, Type::Bool).is_err());
        assert!(TypeALU::subtract(Type::Str, Type::Str).is_err());
    }

    #[test]
    fn multiplication_rejects_all_non_matching_types() {
        assert!(TypeALU::multiplication(Type::I64, Type::F64).is_err());
        assert!(TypeALU::multiplication(Type::F64, Type::I64).is_err());
        assert!(TypeALU::multiplication(Type::Bool, Type::Bool).is_err());
        assert!(TypeALU::multiplication(Type::Str, Type::Str).is_err());
    }

    #[test]
    fn division_rejects_all_non_matching_types() {
        assert!(TypeALU::division(Type::I64, Type::F64).is_err());
        assert!(TypeALU::division(Type::F64, Type::I64).is_err());
        assert!(TypeALU::division(Type::Bool, Type::Bool).is_err());
        assert!(TypeALU::division(Type::Str, Type::Str).is_err());
    }

    #[test]
    fn modulo_valid() {
        assert_eq!(TypeALU::modulo(Type::I64, Type::I64).unwrap(), Type::I64);
    }

    #[test]
    fn modulo_invalid() {
        assert_eq!(
            TypeALU::modulo(Type::F64, Type::F64).err().unwrap().message(),
            "Cannot perform modulo between values of type 'f64' and 'f64'."
        );

        assert_eq!(
            TypeALU::modulo(Type::I64, Type::F64).err().unwrap().message(),
            "Cannot perform modulo between values of type 'i64' and 'f64'."
        );

        assert_eq!(
            TypeALU::modulo(Type::Bool, Type::Bool).err().unwrap().message(),
            "Cannot perform modulo between values of type 'bool' and 'bool'."
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

    #[test]
    fn greater_supports_f64() {
        assert_eq!(TypeALU::greater(Type::F64, Type::F64).unwrap(), Type::Bool);
    }

    #[test]
    fn greater_or_equal_supports_f64() {
        assert_eq!(TypeALU::greater_or_equal(Type::F64, Type::F64).unwrap(), Type::Bool);
    }

    #[test]
    fn less_supports_f64() {
        assert_eq!(TypeALU::less(Type::F64, Type::F64).unwrap(), Type::Bool);
    }

    #[test]
    fn less_or_equal_supports_i64() {
        assert_eq!(TypeALU::less_or_equal(Type::I64, Type::I64).unwrap(), Type::Bool);
    }

    #[test]
    fn comparisons_reject_mixed_numeric_types() {
        assert!(TypeALU::greater(Type::I64, Type::F64).is_err());
        assert!(TypeALU::greater_or_equal(Type::F64, Type::I64).is_err());
        assert!(TypeALU::less(Type::I64, Type::F64).is_err());
        assert!(TypeALU::less_or_equal(Type::F64, Type::I64).is_err());
    }

    #[test]
    fn comparisons_reject_non_numeric_types() {
        assert!(TypeALU::greater(Type::Str, Type::Str).is_err());
        assert!(TypeALU::greater_or_equal(Type::Bool, Type::Bool).is_err());
        assert!(TypeALU::less(Type::Str, Type::Str).is_err());
        assert!(TypeALU::less_or_equal(Type::Bool, Type::Bool).is_err());
    }

    #[test]
    fn equality_supports_all_matching_types() {
        assert_eq!(TypeALU::equal(Type::I64, Type::I64).unwrap(), Type::Bool);
        assert_eq!(TypeALU::equal(Type::F64, Type::F64).unwrap(), Type::Bool);
        assert_eq!(TypeALU::equal(Type::Str, Type::Str).unwrap(), Type::Bool);
        assert_eq!(TypeALU::equal(Type::Bool, Type::Bool).unwrap(), Type::Bool);
    }

    #[test]
    fn not_equal_supports_all_matching_types() {
        assert_eq!(TypeALU::not_equal(Type::I64, Type::I64).unwrap(), Type::Bool);
        assert_eq!(TypeALU::not_equal(Type::F64, Type::F64).unwrap(), Type::Bool);
        assert_eq!(TypeALU::not_equal(Type::Str, Type::Str).unwrap(), Type::Bool);
        assert_eq!(TypeALU::not_equal(Type::Bool, Type::Bool).unwrap(), Type::Bool);
    }

    #[test]
    fn equality_rejects_all_mixed_types() {
        assert!(TypeALU::equal(Type::I64, Type::F64).is_err());
        assert!(TypeALU::equal(Type::I64, Type::Str).is_err());
        assert!(TypeALU::equal(Type::I64, Type::Bool).is_err());
        assert!(TypeALU::equal(Type::F64, Type::Str).is_err());
        assert!(TypeALU::equal(Type::F64, Type::Bool).is_err());
        assert!(TypeALU::equal(Type::Str, Type::Bool).is_err());
    }

    #[test]
    fn not_equal_rejects_all_mixed_types() {
        assert!(TypeALU::not_equal(Type::I64, Type::F64).is_err());
        assert!(TypeALU::not_equal(Type::I64, Type::Str).is_err());
        assert!(TypeALU::not_equal(Type::I64, Type::Bool).is_err());
        assert!(TypeALU::not_equal(Type::F64, Type::Str).is_err());
        assert!(TypeALU::not_equal(Type::F64, Type::Bool).is_err());
        assert!(TypeALU::not_equal(Type::Str, Type::Bool).is_err());
    }
}
