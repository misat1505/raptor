use inkwell::{context::Context, AddressSpace};

use crate::{
    backend::llvm::{
        llvm_alu::{tests::setup, LlvmAlu, OverflowPolicy},
        LlvmValue,
    },
    common::span::Span,
};

#[test]
fn add_type_mismatch_fails() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let i64_ty = context.i64_type();
    let f64_ty = context.f64_type();

    let left = LlvmValue::I64(i64_ty.const_int(1, true));
    let right = LlvmValue::F64(f64_ty.const_float(2.0));

    let err = alu.add(&builder, &libc, left, right, span).unwrap_err();

    assert!(err.message().contains("addition") || err.message().to_lowercase().contains("type"));
}

#[test]
fn subtract_type_mismatch_fails() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let left = LlvmValue::I64(context.i64_type().const_int(5, true));
    let right = LlvmValue::Str(context.ptr_type(AddressSpace::default()).const_null());

    assert!(alu.subtract(&builder, &libc, left, right, span).is_err());
}

#[test]
fn multiplication_type_mismatch_fails() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let left = LlvmValue::Bool(context.bool_type().const_int(1, false));
    let right = LlvmValue::I32(context.i32_type().const_int(2, true));

    assert!(alu.multiplication(&builder, &libc, left, right, span).is_err());
}

#[test]
fn division_type_mismatch_fails() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let left = LlvmValue::F64(context.f64_type().const_float(1.0));
    let right = LlvmValue::I64(context.i64_type().const_int(2, true));

    assert!(alu.division(&builder, &libc, left, right, span).is_err());
}

#[test]
fn modulo_rejects_float() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let left = LlvmValue::F64(context.f64_type().const_float(5.0));
    let right = LlvmValue::F64(context.f64_type().const_float(2.0));

    assert!(alu.modulo(&builder, &libc, left, right, span).is_err());
}

#[test]
fn add_integers() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let cases: Vec<(LlvmValue, LlvmValue, fn(&LlvmValue) -> bool)> = vec![
        (
            LlvmValue::I8(context.i8_type().const_int(10, true)),
            LlvmValue::I8(context.i8_type().const_int(3, true)),
            |v| matches!(v, LlvmValue::I8(_)),
        ),
        (
            LlvmValue::I16(context.i16_type().const_int(10, true)),
            LlvmValue::I16(context.i16_type().const_int(3, true)),
            |v| matches!(v, LlvmValue::I16(_)),
        ),
        (
            LlvmValue::I32(context.i32_type().const_int(10, true)),
            LlvmValue::I32(context.i32_type().const_int(3, true)),
            |v| matches!(v, LlvmValue::I32(_)),
        ),
        (
            LlvmValue::I64(context.i64_type().const_int(10, true)),
            LlvmValue::I64(context.i64_type().const_int(3, true)),
            |v| matches!(v, LlvmValue::I64(_)),
        ),
        (
            LlvmValue::U8(context.i8_type().const_int(10, false)),
            LlvmValue::U8(context.i8_type().const_int(3, false)),
            |v| matches!(v, LlvmValue::U8(_)),
        ),
        (
            LlvmValue::U16(context.i16_type().const_int(10, false)),
            LlvmValue::U16(context.i16_type().const_int(3, false)),
            |v| matches!(v, LlvmValue::U16(_)),
        ),
        (
            LlvmValue::U32(context.i32_type().const_int(10, false)),
            LlvmValue::U32(context.i32_type().const_int(3, false)),
            |v| matches!(v, LlvmValue::U32(_)),
        ),
        (
            LlvmValue::U64(context.i64_type().const_int(10, false)),
            LlvmValue::U64(context.i64_type().const_int(3, false)),
            |v| matches!(v, LlvmValue::U64(_)),
        ),
    ];

    for (left, right, check) in cases {
        let result = alu.add(&builder, &libc, left, right, span).unwrap();

        assert!(check(&result));
    }
}

#[test]
fn add_floats() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let left = LlvmValue::F64(context.f64_type().const_float(1.5));
    let right = LlvmValue::F64(context.f64_type().const_float(2.5));

    let result = alu.add(&builder, &libc, left, right, span).unwrap();

    assert!(matches!(result, LlvmValue::F64(_)));
}

#[test]
fn subtract_integers_and_float() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let left = LlvmValue::I64(context.i64_type().const_int(10, true));
    let right = LlvmValue::I64(context.i64_type().const_int(3, true));

    assert!(matches!(alu.subtract(&builder, &libc, left, right, span).unwrap(), LlvmValue::I64(_)));

    let left = LlvmValue::F64(context.f64_type().const_float(5.0));
    let right = LlvmValue::F64(context.f64_type().const_float(1.5));

    assert!(matches!(alu.subtract(&builder, &libc, left, right, span).unwrap(), LlvmValue::F64(_)));
}

#[test]
fn multiplication_integers_and_float() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let left = LlvmValue::I32(context.i32_type().const_int(6, true));
    let right = LlvmValue::I32(context.i32_type().const_int(7, true));

    assert!(matches!(
        alu.multiplication(&builder, &libc, left, right, span).unwrap(),
        LlvmValue::I32(_)
    ));

    let left = LlvmValue::F64(context.f64_type().const_float(2.0));
    let right = LlvmValue::F64(context.f64_type().const_float(3.0));

    assert!(matches!(
        alu.multiplication(&builder, &libc, left, right, span).unwrap(),
        LlvmValue::F64(_)
    ));
}

#[test]
fn division_signed_unsigned_float() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    // signed
    let left = LlvmValue::I64(context.i64_type().const_int(10, true));
    let right = LlvmValue::I64(context.i64_type().const_int(3, true));

    assert!(matches!(alu.division(&builder, &libc, left, right, span).unwrap(), LlvmValue::I64(_)));

    // unsigned
    let left = LlvmValue::U32(context.i32_type().const_int(10, false));
    let right = LlvmValue::U32(context.i32_type().const_int(3, false));

    assert!(matches!(alu.division(&builder, &libc, left, right, span).unwrap(), LlvmValue::U32(_)));

    // float
    let left = LlvmValue::F64(context.f64_type().const_float(10.0));
    let right = LlvmValue::F64(context.f64_type().const_float(4.0));

    assert!(matches!(alu.division(&builder, &libc, left, right, span).unwrap(), LlvmValue::F64(_)));
}

#[test]
fn modulo_signed_and_unsigned() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let left = LlvmValue::I64(context.i64_type().const_int(10, true));
    let right = LlvmValue::I64(context.i64_type().const_int(3, true));

    assert!(matches!(alu.modulo(&builder, &libc, left, right, span).unwrap(), LlvmValue::I64(_)));

    let left = LlvmValue::U8(context.i8_type().const_int(10, false));
    let right = LlvmValue::U8(context.i8_type().const_int(3, false));

    assert!(matches!(alu.modulo(&builder, &libc, left, right, span).unwrap(), LlvmValue::U8(_)));
}

#[test]
fn add_str_str_produces_str() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let ptr_ty = context.ptr_type(AddressSpace::default());

    let left = LlvmValue::Str(ptr_ty.const_null());
    let right = LlvmValue::Str(ptr_ty.const_null());

    let result = alu.add(&builder, &libc, left, right, span).unwrap();

    assert!(matches!(result, LlvmValue::Str(_)));
}

#[test]
fn add_char_char_produces_str() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let i8_ty = context.i8_type();

    let left = LlvmValue::Char(i8_ty.const_int(b'a' as u64, false));
    let right = LlvmValue::Char(i8_ty.const_int(b'b' as u64, false));

    let result = alu.add(&builder, &libc, left, right, span).unwrap();

    assert!(matches!(result, LlvmValue::Str(_)));
}

#[test]
fn add_str_char_and_char_str_produce_str() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let ptr_ty = context.ptr_type(AddressSpace::default());
    let i8_ty = context.i8_type();

    let str_val = LlvmValue::Str(ptr_ty.const_null());
    let char_val = LlvmValue::Char(i8_ty.const_int(b'x' as u64, false));

    let result = alu.add(&builder, &libc, str_val.clone(), char_val.clone(), span).unwrap();

    assert!(matches!(result, LlvmValue::Str(_)));

    let result = alu.add(&builder, &libc, char_val, str_val, span).unwrap();

    assert!(matches!(result, LlvmValue::Str(_)));
}

// -----------------------------------------------------------------------------
// Overflow policy
// -----------------------------------------------------------------------------

#[test]
fn arithmetic_operations_use_overflow_policy() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let ignore = LlvmAlu::new(OverflowPolicy::Ignore);
    let warn = LlvmAlu::new(OverflowPolicy::Warn);
    let error = LlvmAlu::new(OverflowPolicy::Error);

    // The important part here is that every policy can be constructed
    // independently and used by the same operation.
    //
    // Non-overflowing operations must work regardless of the selected policy.

    let left = LlvmValue::I8(context.i8_type().const_int(10, true));
    let right = LlvmValue::I8(context.i8_type().const_int(5, true));

    assert!(ignore.add(&builder, &libc, left.clone(), right.clone(), span).is_ok());

    assert!(warn.add(&builder, &libc, left.clone(), right.clone(), span).is_ok());

    assert!(error.add(&builder, &libc, left, right, span).is_ok());
}

#[test]
fn arithmetic_negation_accepts_all_integer_types() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let cases = [
        LlvmValue::I8(context.i8_type().const_int(10, true)),
        LlvmValue::I16(context.i16_type().const_int(10, true)),
        LlvmValue::I32(context.i32_type().const_int(10, true)),
        LlvmValue::I64(context.i64_type().const_int(10, true)),
    ];

    for value in cases {
        let result = alu.arithmetic_negate(&builder, &libc, value, span).unwrap();

        assert!(matches!(
            result,
            LlvmValue::I8(_) | LlvmValue::I16(_) | LlvmValue::I32(_) | LlvmValue::I64(_)
        ));
    }
}

#[test]
fn arithmetic_negation_accepts_f64() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let value = LlvmValue::F64(context.f64_type().const_float(42.5));

    let result = alu.arithmetic_negate(&builder, &libc, value, span).unwrap();

    assert!(matches!(result, LlvmValue::F64(_)));
}

#[test]
fn arithmetic_negation_rejects_unsigned_values() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let values = [
        LlvmValue::U8(context.i8_type().const_int(1, false)),
        LlvmValue::U16(context.i16_type().const_int(1, false)),
        LlvmValue::U32(context.i32_type().const_int(1, false)),
        LlvmValue::U64(context.i64_type().const_int(1, false)),
    ];

    for value in values {
        assert!(alu.arithmetic_negate(&builder, &libc, value, span).is_err());
    }
}

#[test]
fn boolean_negation_accepts_bool() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let value = LlvmValue::Bool(context.bool_type().const_int(1, false));

    let result = alu.boolean_negate(&builder, &libc, value, span).unwrap();

    assert!(matches!(result, LlvmValue::Bool(_)));
}

#[test]
fn boolean_negation_rejects_non_bool() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let alu = LlvmAlu::new(OverflowPolicy::Warn);

    let values = [
        LlvmValue::I64(context.i64_type().const_int(1, true)),
        LlvmValue::F64(context.f64_type().const_float(1.0)),
        LlvmValue::Char(context.i8_type().const_int(b'a' as u64, false)),
        LlvmValue::U32(context.i32_type().const_int(1, false)),
    ];

    for value in values {
        assert!(alu.boolean_negate(&builder, &libc, value, span).is_err());
    }
}
