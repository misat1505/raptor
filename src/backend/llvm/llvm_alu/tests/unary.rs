use inkwell::context::Context;

use crate::{
    backend::llvm::{
        llvm_alu::{tests::setup, LlvmAlu},
        LlvmValue,
    },
    common::span::Span,
};

#[test]
fn boolean_negate_on_bool() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let t = LlvmValue::Bool(context.bool_type().const_int(1, false));
    let f = LlvmValue::Bool(context.bool_type().const_int(0, false));

    assert!(matches!(LlvmAlu::boolean_negate(&builder, &libc, t, span).unwrap(), LlvmValue::Bool(_)));
    assert!(matches!(LlvmAlu::boolean_negate(&builder, &libc, f, span).unwrap(), LlvmValue::Bool(_)));
}

#[test]
fn boolean_negate_rejects_non_bool() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let bad = [
        LlvmValue::I64(context.i64_type().const_int(1, true)),
        LlvmValue::F64(context.f64_type().const_float(1.0)),
        LlvmValue::U8(context.i8_type().const_int(1, false)),
        LlvmValue::Char(context.i8_type().const_int(b'a' as u64, false)),
        LlvmValue::Str(context.ptr_type(inkwell::AddressSpace::default()).const_null()),
    ];

    for v in bad {
        assert!(LlvmAlu::boolean_negate(&builder, &libc, v, span).is_err());
    }
}

#[test]
fn arithmetic_negate_signed_integers() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    assert!(matches!(
        LlvmAlu::arithmetic_negate(&builder, &libc, LlvmValue::I8(context.i8_type().const_int(5, true)), span).unwrap(),
        LlvmValue::I8(_)
    ));
    assert!(matches!(
        LlvmAlu::arithmetic_negate(&builder, &libc, LlvmValue::I16(context.i16_type().const_int(5, true)), span).unwrap(),
        LlvmValue::I16(_)
    ));
    assert!(matches!(
        LlvmAlu::arithmetic_negate(&builder, &libc, LlvmValue::I32(context.i32_type().const_int(5, true)), span).unwrap(),
        LlvmValue::I32(_)
    ));
    assert!(matches!(
        LlvmAlu::arithmetic_negate(&builder, &libc, LlvmValue::I64(context.i64_type().const_int(5, true)), span).unwrap(),
        LlvmValue::I64(_)
    ));
}

#[test]
fn arithmetic_negate_f64() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let v = LlvmValue::F64(context.f64_type().const_float(3.14));
    assert!(matches!(LlvmAlu::arithmetic_negate(&builder, &libc, v, span).unwrap(), LlvmValue::F64(_)));
}

#[test]
fn arithmetic_negate_rejects_unsupported() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let bad = [
        LlvmValue::U8(context.i8_type().const_int(1, false)),
        LlvmValue::U16(context.i16_type().const_int(1, false)),
        LlvmValue::U32(context.i32_type().const_int(1, false)),
        LlvmValue::U64(context.i64_type().const_int(1, false)),
        LlvmValue::Bool(context.bool_type().const_int(1, false)),
        LlvmValue::Char(context.i8_type().const_int(b'x' as u64, false)),
        LlvmValue::Str(context.ptr_type(inkwell::AddressSpace::default()).const_null()),
    ];

    for v in bad {
        assert!(LlvmAlu::arithmetic_negate(&builder, &libc, v, span).is_err());
    }
}
