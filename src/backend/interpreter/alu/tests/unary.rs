use std::assert_eq;

use crate::backend::interpreter::alu::value::Value;
use crate::backend::interpreter::alu::ALU;
use crate::common::errors::IError;
use crate::common::span::Span;

#[test]
fn boolean_negation() {
    assert_eq!(ALU::boolean_negate(Value::Bool(false), Span::default()).unwrap(), Value::Bool(true));
    assert_eq!(ALU::boolean_negate(Value::Bool(true), Span::default()).unwrap(), Value::Bool(false));
    assert_eq!(
        ALU::boolean_negate(Value::I64(1), Span::default()).err().unwrap().message(),
        String::from("Cannot perform boolean negation on type 'i64'.")
    );
}

#[test]
fn arithmetic_negation() {
    assert_eq!(ALU::arithmetic_negate(Value::I64(1), Span::default()).unwrap(), Value::I64(-1));
    assert_eq!(ALU::arithmetic_negate(Value::F64(-21.37), Span::default()).unwrap(), Value::F64(21.37));
    assert_eq!(
        ALU::arithmetic_negate(Value::String(String::from("abc")), Span::default())
            .err()
            .unwrap()
            .message(),
        String::from("Cannot perform arithmetic negation on type 'str'.")
    );
}

#[test]
fn boolean_negation_unsupported_types() {
    let unsupported = [
        Value::I8(1),
        Value::I16(1),
        Value::I32(1),
        Value::U8(1),
        Value::U16(1),
        Value::U32(1),
        Value::U64(1),
        Value::F64(1.0),
        Value::String("true".into()),
        Value::Char('t'),
    ];
    for val in unsupported {
        let r = ALU::boolean_negate(val, Span::default());
        assert!(r.is_err());
        let msg = r.err().unwrap().message();
        assert!(msg.starts_with("Cannot perform boolean negation on type"), "got: {}", msg);
    }
}

#[test]
fn arithmetic_negation_all_signed_integers() {
    // I8
    assert_eq!(ALU::arithmetic_negate(Value::I8(5), Span::default()).unwrap(), Value::I8(-5));
    assert_eq!(ALU::arithmetic_negate(Value::I8(-5), Span::default()).unwrap(), Value::I8(5));
    assert_eq!(ALU::arithmetic_negate(Value::I8(0), Span::default()).unwrap(), Value::I8(0));

    // I16
    assert_eq!(ALU::arithmetic_negate(Value::I16(1000), Span::default()).unwrap(), Value::I16(-1000));
    assert_eq!(ALU::arithmetic_negate(Value::I16(-1000), Span::default()).unwrap(), Value::I16(1000));

    // I32
    assert_eq!(
        ALU::arithmetic_negate(Value::I32(i32::MAX), Span::default()).unwrap(),
        Value::I32(-i32::MAX)
    );
    assert_eq!(ALU::arithmetic_negate(Value::I32(-42), Span::default()).unwrap(), Value::I32(42));

    // I64 already partially covered, add zero and negative
    assert_eq!(ALU::arithmetic_negate(Value::I64(0), Span::default()).unwrap(), Value::I64(0));
    assert_eq!(
        ALU::arithmetic_negate(Value::I64(i64::MAX), Span::default()).unwrap(),
        Value::I64(-i64::MAX)
    );
}

#[test]
fn arithmetic_negation_overflow() {
    // i8::MIN cannot be negated
    let r = ALU::arithmetic_negate(Value::I8(i8::MIN), Span::default());
    assert!(r.is_err());
    assert_eq!(r.err().unwrap().message(), "Overflow occurred when performing arithmetic negation on i8.");

    // i16::MIN
    let r = ALU::arithmetic_negate(Value::I16(i16::MIN), Span::default());
    assert!(r.is_err());
    assert_eq!(
        r.err().unwrap().message(),
        "Overflow occurred when performing arithmetic negation on i16."
    );

    // i32::MIN
    let r = ALU::arithmetic_negate(Value::I32(i32::MIN), Span::default());
    assert!(r.is_err());
    assert_eq!(
        r.err().unwrap().message(),
        "Overflow occurred when performing arithmetic negation on i32."
    );

    // i64::MIN
    let r = ALU::arithmetic_negate(Value::I64(i64::MIN), Span::default());
    assert!(r.is_err());
    assert_eq!(
        r.err().unwrap().message(),
        "Overflow occurred when performing arithmetic negation on i64."
    );
}

#[test]
fn arithmetic_negation_f64_edge_cases() {
    assert_eq!(ALU::arithmetic_negate(Value::F64(0.0), Span::default()).unwrap(), Value::F64(-0.0));
    assert_eq!(ALU::arithmetic_negate(Value::F64(-0.0), Span::default()).unwrap(), Value::F64(0.0));
    assert_eq!(
        ALU::arithmetic_negate(Value::F64(f64::INFINITY), Span::default()).unwrap(),
        Value::F64(f64::NEG_INFINITY)
    );
    assert_eq!(
        ALU::arithmetic_negate(Value::F64(f64::NEG_INFINITY), Span::default()).unwrap(),
        Value::F64(f64::INFINITY)
    );
    // NaN stays NaN
    let res = ALU::arithmetic_negate(Value::F64(f64::NAN), Span::default()).unwrap();
    if let Value::F64(v) = res {
        assert!(v.is_nan());
    } else {
        panic!("expected F64");
    }
}

#[test]
fn arithmetic_negation_unsupported_types() {
    let unsupported = [
        Value::U8(1),
        Value::U16(1),
        Value::U32(1),
        Value::U64(1),
        Value::Bool(true),
        Value::Char('a'),
        Value::String("1".into()),
    ];
    for val in unsupported {
        let r = ALU::arithmetic_negate(val, Span::default());
        assert!(r.is_err());
        let msg = r.err().unwrap().message();
        assert!(msg.starts_with("Cannot perform arithmetic negation on type"), "got: {}", msg);
    }
}
