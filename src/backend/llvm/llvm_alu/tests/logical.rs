use inkwell::context::Context;

use crate::{
    backend::llvm::{
        llvm_alu::{tests::setup, LlvmAlu},
        LlvmValue, OverflowPolicy,
    },
    common::span::Span,
};

#[test]
fn concatenation_bool_bool() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let t = LlvmValue::Bool(context.bool_type().const_int(1, false));
    let f = LlvmValue::Bool(context.bool_type().const_int(0, false));

    assert!(matches!(
        alu.concatenation(&builder, &libc, t.clone(), t.clone(), span).unwrap(),
        LlvmValue::Bool(_)
    ));

    assert!(matches!(
        alu.concatenation(&builder, &libc, t.clone(), f.clone(), span).unwrap(),
        LlvmValue::Bool(_)
    ));

    assert!(matches!(
        alu.concatenation(&builder, &libc, f.clone(), f.clone(), span).unwrap(),
        LlvmValue::Bool(_)
    ));
}

#[test]
fn concatenation_rejects_non_bool() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let b = LlvmValue::Bool(context.bool_type().const_int(1, false));
    let i = LlvmValue::I64(context.i64_type().const_int(1, true));
    let f = LlvmValue::F64(context.f64_type().const_float(1.0));

    assert!(alu.concatenation(&builder, &libc, b.clone(), i.clone(), span).is_err());

    assert!(alu.concatenation(&builder, &libc, i.clone(), b.clone(), span).is_err());

    assert!(alu.concatenation(&builder, &libc, i.clone(), i.clone(), span).is_err());

    assert!(alu.concatenation(&builder, &libc, f.clone(), f.clone(), span).is_err());
}

#[test]
fn alternative_bool_bool() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let t = LlvmValue::Bool(context.bool_type().const_int(1, false));
    let f = LlvmValue::Bool(context.bool_type().const_int(0, false));

    assert!(matches!(
        alu.alternative(&builder, &libc, t.clone(), t.clone(), span).unwrap(),
        LlvmValue::Bool(_)
    ));

    assert!(matches!(
        alu.alternative(&builder, &libc, t.clone(), f.clone(), span).unwrap(),
        LlvmValue::Bool(_)
    ));

    assert!(matches!(
        alu.alternative(&builder, &libc, f.clone(), f.clone(), span).unwrap(),
        LlvmValue::Bool(_)
    ));
}

#[test]
fn alternative_rejects_non_bool() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let b = LlvmValue::Bool(context.bool_type().const_int(1, false));
    let i = LlvmValue::I64(context.i64_type().const_int(1, true));
    let s = LlvmValue::Str(context.ptr_type(inkwell::AddressSpace::default()).const_null());

    assert!(alu.alternative(&builder, &libc, b.clone(), i.clone(), span).is_err());

    assert!(alu.alternative(&builder, &libc, i.clone(), b.clone(), span).is_err());

    assert!(alu.alternative(&builder, &libc, i.clone(), i.clone(), span).is_err());

    assert!(alu.alternative(&builder, &libc, s.clone(), s.clone(), span).is_err());
}
