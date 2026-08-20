use std::assert_eq;

use crate::backend::interpreter::alu::value::Value;
use crate::backend::interpreter::alu::ALU;
use crate::common::errors::IError;
use crate::common::span::Span;

#[test]
fn greater() {
    assert_eq!(ALU::greater(Value::I64(1), Value::I64(2), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(ALU::greater(Value::I64(2), Value::I64(2), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(ALU::greater(Value::I64(3), Value::I64(2), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(
        ALU::greater(Value::F64(1.0), Value::F64(2.0), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::greater(Value::F64(2.0), Value::F64(2.0), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::greater(Value::F64(3.0), Value::F64(2.0), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::greater(Value::I64(2), Value::F64(3.0), Span::default()).err().unwrap().message(),
        String::from("Cannot perform greater between values of type 'i64' and 'f64'.")
    );
}

#[test]
fn greater_or_equal() {
    assert_eq!(
        ALU::greater_or_equal(Value::I64(1), Value::I64(2), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::greater_or_equal(Value::I64(2), Value::I64(2), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::greater_or_equal(Value::I64(3), Value::I64(2), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::greater_or_equal(Value::F64(1.0), Value::F64(2.0), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::greater_or_equal(Value::F64(2.0), Value::F64(2.0), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::greater_or_equal(Value::F64(3.0), Value::F64(2.0), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::greater_or_equal(Value::I64(2), Value::F64(3.0), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform greater or equal between values of type 'i64' and 'f64'.")
    );
}

#[test]
fn less() {
    assert_eq!(ALU::less(Value::I64(1), Value::I64(2), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::less(Value::I64(2), Value::I64(2), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(ALU::less(Value::I64(3), Value::I64(2), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(ALU::less(Value::F64(1.0), Value::F64(2.0), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::less(Value::F64(2.0), Value::F64(2.0), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(ALU::less(Value::F64(3.0), Value::F64(2.0), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(
        ALU::less(Value::I64(2), Value::F64(3.0), Span::default()).err().unwrap().message(),
        String::from("Cannot perform less between values of type 'i64' and 'f64'.")
    );
}

#[test]
fn less_or_equal() {
    assert_eq!(
        ALU::less_or_equal(Value::I64(1), Value::I64(2), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::less_or_equal(Value::I64(2), Value::I64(2), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::less_or_equal(Value::I64(3), Value::I64(2), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::less_or_equal(Value::F64(1.0), Value::F64(2.0), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::less_or_equal(Value::F64(2.0), Value::F64(2.0), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::less_or_equal(Value::F64(3.0), Value::F64(2.0), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::less_or_equal(Value::I64(2), Value::F64(3.0), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform less or equal between values of type 'i64' and 'f64'.")
    );
}

#[test]
fn equal() {
    assert_eq!(ALU::equal(Value::I64(1), Value::I64(2), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(ALU::equal(Value::I64(2), Value::I64(2), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::equal(Value::F64(1.0), Value::F64(2.0), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(ALU::equal(Value::F64(2.0), Value::F64(2.0), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(
        ALU::equal(Value::String(String::from("a")), Value::String(String::from("b")), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::equal(Value::String(String::from("a")), Value::String(String::from("a")), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::equal(Value::Bool(true), Value::Bool(false), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::equal(Value::Bool(true), Value::Bool(true), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::equal(Value::Bool(true), Value::I64(1), Span::default()).err().unwrap().message(),
        String::from("Cannot perform equal between values of type 'bool' and 'i64'.")
    );
}

#[test]
fn not_equal() {
    assert_eq!(ALU::not_equal(Value::I64(1), Value::I64(2), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::not_equal(Value::I64(2), Value::I64(2), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(
        ALU::not_equal(Value::F64(1.0), Value::F64(2.0), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::not_equal(Value::F64(2.0), Value::F64(2.0), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::not_equal(Value::String(String::from("a")), Value::String(String::from("b")), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::not_equal(Value::String(String::from("a")), Value::String(String::from("a")), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::not_equal(Value::Bool(true), Value::Bool(false), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::not_equal(Value::Bool(true), Value::Bool(true), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::not_equal(Value::Bool(true), Value::I64(1), Span::default()).err().unwrap().message(),
        String::from("Cannot perform not equal between values of type 'bool' and 'i64'.")
    );
}

#[test]
fn comparisons_unsupported_types_fail() {
    assert_eq!(
        ALU::greater(Value::Bool(true), Value::Bool(false), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform greater between values of type 'bool' and 'bool'.")
    );
    assert_eq!(
        ALU::greater_or_equal(Value::String(String::from("a")), Value::String(String::from("b")), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform greater or equal between values of type 'str' and 'str'.")
    );
    assert_eq!(
        ALU::less(Value::Bool(true), Value::Bool(false), Span::default()).err().unwrap().message(),
        String::from("Cannot perform less between values of type 'bool' and 'bool'.")
    );
    assert_eq!(
        ALU::less_or_equal(Value::String(String::from("a")), Value::String(String::from("b")), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform less or equal between values of type 'str' and 'str'.")
    );
}

#[test]
fn greater_all_integer_types() {
    // I8
    assert_eq!(ALU::greater(Value::I8(5), Value::I8(3), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::greater(Value::I8(-1), Value::I8(0), Span::default()).unwrap(), Value::Bool(false));
    // I16
    assert_eq!(
        ALU::greater(Value::I16(1000), Value::I16(999), Span::default()).unwrap(),
        Value::Bool(true)
    );
    // I32
    assert_eq!(
        ALU::greater(Value::I32(-10), Value::I32(-20), Span::default()).unwrap(),
        Value::Bool(true)
    );
    // U8
    assert_eq!(ALU::greater(Value::U8(255), Value::U8(0), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::greater(Value::U8(0), Value::U8(1), Span::default()).unwrap(), Value::Bool(false));
    // U16
    assert_eq!(
        ALU::greater(Value::U16(100), Value::U16(100), Span::default()).unwrap(),
        Value::Bool(false)
    );
    // U32
    assert_eq!(ALU::greater(Value::U32(1), Value::U32(0), Span::default()).unwrap(), Value::Bool(true));
    // U64
    assert_eq!(
        ALU::greater(Value::U64(u64::MAX), Value::U64(0), Span::default()).unwrap(),
        Value::Bool(true)
    );
}

#[test]
fn greater_or_equal_all_integer_types() {
    assert_eq!(
        ALU::greater_or_equal(Value::I8(5), Value::I8(5), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::greater_or_equal(Value::I16(-5), Value::I16(-4), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::greater_or_equal(Value::U8(0), Value::U8(0), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::greater_or_equal(Value::U64(10), Value::U64(9), Span::default()).unwrap(),
        Value::Bool(true)
    );
}

#[test]
fn less_all_integer_types() {
    assert_eq!(ALU::less(Value::I8(-5), Value::I8(0), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::less(Value::I32(100), Value::I32(100), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(ALU::less(Value::U16(1), Value::U16(2), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(
        ALU::less(Value::U32(u32::MAX), Value::U32(0), Span::default()).unwrap(),
        Value::Bool(false)
    );
}

#[test]
fn less_or_equal_all_integer_types() {
    assert_eq!(
        ALU::less_or_equal(Value::I64(i64::MIN), Value::I64(i64::MIN), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::less_or_equal(Value::U8(255), Value::U8(254), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::less_or_equal(Value::I16(0), Value::I16(1), Span::default()).unwrap(),
        Value::Bool(true)
    );
}

#[test]
fn equal_all_supported_types() {
    // remaining integer sizes
    assert_eq!(ALU::equal(Value::I8(42), Value::I8(42), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::equal(Value::I8(42), Value::I8(43), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(ALU::equal(Value::I16(-1), Value::I16(-1), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::equal(Value::I32(0), Value::I32(1), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(ALU::equal(Value::U8(255), Value::U8(255), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::equal(Value::U16(0), Value::U16(0), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::equal(Value::U32(1), Value::U32(2), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(
        ALU::equal(Value::U64(u64::MAX), Value::U64(u64::MAX), Span::default()).unwrap(),
        Value::Bool(true)
    );

    // Char
    assert_eq!(
        ALU::equal(Value::Char('a'), Value::Char('a'), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::equal(Value::Char('a'), Value::Char('b'), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        ALU::equal(Value::Char('\0'), Value::Char('\0'), Span::default()).unwrap(),
        Value::Bool(true)
    );
}

#[test]
fn not_equal_all_supported_types() {
    assert_eq!(ALU::not_equal(Value::I8(1), Value::I8(2), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::not_equal(Value::U64(0), Value::U64(0), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(
        ALU::not_equal(Value::Char('x'), Value::Char('y'), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::not_equal(Value::Char('z'), Value::Char('z'), Span::default()).unwrap(),
        Value::Bool(false)
    );
}

#[test]
fn f64_edge_cases() {
    // NaN comparisons (Rust semantics: NaN is never equal / ordered)
    let nan = Value::F64(f64::NAN);
    assert_eq!(ALU::equal(nan.clone(), nan.clone(), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(ALU::not_equal(nan.clone(), nan.clone(), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::greater(nan.clone(), Value::F64(0.0), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(ALU::less(nan.clone(), Value::F64(0.0), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(
        ALU::greater_or_equal(nan.clone(), Value::F64(0.0), Span::default()).unwrap(),
        Value::Bool(false)
    );
    assert_eq!(ALU::less_or_equal(nan, Value::F64(0.0), Span::default()).unwrap(), Value::Bool(false));

    // Infinity
    assert_eq!(
        ALU::greater(Value::F64(f64::INFINITY), Value::F64(f64::MAX), Span::default()).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        ALU::less(Value::F64(f64::NEG_INFINITY), Value::F64(0.0), Span::default()).unwrap(),
        Value::Bool(true)
    );
}

#[test]
fn type_mismatch_errors_for_all_ops() {
    let pairs = [
        (Value::I8(1), Value::I16(1)),
        (Value::U32(1), Value::I32(1)),
        (Value::F64(1.0), Value::I64(1)),
        (Value::String("a".into()), Value::Char('a')),
        (Value::Bool(true), Value::U8(1)),
        (Value::Char('x'), Value::I64(120)),
    ];

    for (a, b) in pairs {
        // ordered ops
        assert!(ALU::greater(a.clone(), b.clone(), Span::default()).is_err());
        assert!(ALU::greater_or_equal(a.clone(), b.clone(), Span::default()).is_err());
        assert!(ALU::less(a.clone(), b.clone(), Span::default()).is_err());
        assert!(ALU::less_or_equal(a.clone(), b.clone(), Span::default()).is_err());
        // equality
        assert!(ALU::equal(a.clone(), b.clone(), Span::default()).is_err());
        assert!(ALU::not_equal(a, b, Span::default()).is_err());
    }
}

#[test]
fn ordered_ops_reject_non_numeric() {
    // Bool
    assert!(ALU::greater(Value::Bool(true), Value::Bool(false), Span::default()).is_err());
    assert!(ALU::less_or_equal(Value::Bool(false), Value::Bool(true), Span::default()).is_err());

    // String
    assert!(ALU::greater(Value::String("a".into()), Value::String("b".into()), Span::default()).is_err());
    assert!(ALU::less(Value::String("z".into()), Value::String("a".into()), Span::default()).is_err());

    // Char
    assert!(ALU::greater(Value::Char('a'), Value::Char('b'), Span::default()).is_err());
    assert!(ALU::greater_or_equal(Value::Char('x'), Value::Char('x'), Span::default()).is_err());
}
