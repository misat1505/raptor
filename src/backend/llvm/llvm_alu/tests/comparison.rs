use inkwell::{context::Context, AddressSpace};

use crate::{
    backend::llvm::{
        llvm_alu::{tests::setup, LlvmAlu, OverflowPolicy},
        LlvmValue,
    },
    common::span::Span,
};

#[test]
fn greater_all_numeric() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let pairs = [
        (
            LlvmValue::I8(context.i8_type().const_int(5, true)),
            LlvmValue::I8(context.i8_type().const_int(3, true)),
        ),
        (
            LlvmValue::I16(context.i16_type().const_int(5, true)),
            LlvmValue::I16(context.i16_type().const_int(3, true)),
        ),
        (
            LlvmValue::I32(context.i32_type().const_int(5, true)),
            LlvmValue::I32(context.i32_type().const_int(3, true)),
        ),
        (
            LlvmValue::I64(context.i64_type().const_int(5, true)),
            LlvmValue::I64(context.i64_type().const_int(3, true)),
        ),
        (
            LlvmValue::U8(context.i8_type().const_int(5, false)),
            LlvmValue::U8(context.i8_type().const_int(3, false)),
        ),
        (
            LlvmValue::U16(context.i16_type().const_int(5, false)),
            LlvmValue::U16(context.i16_type().const_int(3, false)),
        ),
        (
            LlvmValue::U32(context.i32_type().const_int(5, false)),
            LlvmValue::U32(context.i32_type().const_int(3, false)),
        ),
        (
            LlvmValue::U64(context.i64_type().const_int(5, false)),
            LlvmValue::U64(context.i64_type().const_int(3, false)),
        ),
        (
            LlvmValue::F64(context.f64_type().const_float(5.0)),
            LlvmValue::F64(context.f64_type().const_float(3.0)),
        ),
    ];

    for (l, r) in pairs {
        let result = alu.greater(&builder, &libc, l, r, span).unwrap();

        assert!(matches!(result, LlvmValue::Bool(_)));
    }
}

#[test]
fn greater_or_equal_all_numeric() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let l = LlvmValue::I64(context.i64_type().const_int(5, true));
    let r = LlvmValue::I64(context.i64_type().const_int(5, true));

    assert!(matches!(alu.greater_or_equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let l = LlvmValue::F64(context.f64_type().const_float(2.0));
    let r = LlvmValue::F64(context.f64_type().const_float(1.0));

    assert!(matches!(alu.greater_or_equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let l = LlvmValue::U32(context.i32_type().const_int(10, false));
    let r = LlvmValue::U32(context.i32_type().const_int(3, false));

    assert!(matches!(alu.greater_or_equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));
}

#[test]
fn less_all_numeric() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let l = LlvmValue::I32(context.i32_type().const_int(1, true));
    let r = LlvmValue::I32(context.i32_type().const_int(9, true));

    assert!(matches!(alu.less(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let l = LlvmValue::U8(context.i8_type().const_int(1, false));
    let r = LlvmValue::U8(context.i8_type().const_int(9, false));

    assert!(matches!(alu.less(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let l = LlvmValue::F64(context.f64_type().const_float(0.5));
    let r = LlvmValue::F64(context.f64_type().const_float(1.5));

    assert!(matches!(alu.less(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));
}

#[test]
fn less_or_equal_all_numeric() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let l = LlvmValue::I64(context.i64_type().const_int(3, true));
    let r = LlvmValue::I64(context.i64_type().const_int(3, true));

    assert!(matches!(alu.less_or_equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let l = LlvmValue::F64(context.f64_type().const_float(1.0));
    let r = LlvmValue::F64(context.f64_type().const_float(2.0));

    assert!(matches!(alu.less_or_equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));
}

#[test]
fn ordered_ops_reject_mismatched_and_non_numeric() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let i64v = LlvmValue::I64(context.i64_type().const_int(1, true));
    let f64v = LlvmValue::F64(context.f64_type().const_float(1.0));
    let strv = LlvmValue::Str(context.ptr_type(AddressSpace::default()).const_null());
    let boolv = LlvmValue::Bool(context.bool_type().const_int(1, false));
    let charv = LlvmValue::Char(context.i8_type().const_int(b'a' as u64, false));

    assert!(alu.greater(&builder, &libc, i64v.clone(), f64v.clone(), span).is_err());

    assert!(alu.less(&builder, &libc, f64v.clone(), i64v.clone(), span).is_err());

    assert!(alu.greater(&builder, &libc, strv.clone(), strv.clone(), span).is_err());

    assert!(alu.greater_or_equal(&builder, &libc, boolv.clone(), boolv.clone(), span).is_err());

    assert!(alu.less(&builder, &libc, charv.clone(), charv.clone(), span).is_err());

    assert!(alu.less_or_equal(&builder, &libc, strv.clone(), i64v.clone(), span).is_err());
}

#[test]
fn equal_same_integer_sizes() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let pairs = [
        (
            LlvmValue::I8(context.i8_type().const_int(1, true)),
            LlvmValue::I8(context.i8_type().const_int(1, true)),
        ),
        (
            LlvmValue::I64(context.i64_type().const_int(1, true)),
            LlvmValue::I64(context.i64_type().const_int(1, true)),
        ),
        (
            LlvmValue::U32(context.i32_type().const_int(1, false)),
            LlvmValue::U32(context.i32_type().const_int(1, false)),
        ),
    ];

    for (l, r) in pairs {
        assert!(matches!(alu.equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));
    }
}

#[test]
fn equal_mixed_integer_sizes() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let l = LlvmValue::I8(context.i8_type().const_int(5, true));
    let r = LlvmValue::I64(context.i64_type().const_int(5, true));

    assert!(matches!(alu.equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let l = LlvmValue::U16(context.i16_type().const_int(10, false));
    let r = LlvmValue::I32(context.i32_type().const_int(10, true));

    assert!(matches!(alu.equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));
}

#[test]
fn equal_f64_bool_char_str() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let l = LlvmValue::F64(context.f64_type().const_float(1.0));
    let r = LlvmValue::F64(context.f64_type().const_float(1.0));

    assert!(matches!(alu.equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let l = LlvmValue::Bool(context.bool_type().const_int(1, false));
    let r = LlvmValue::Bool(context.bool_type().const_int(0, false));

    assert!(matches!(alu.equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let l = LlvmValue::Char(context.i8_type().const_int(b'x' as u64, false));
    let r = LlvmValue::Char(context.i8_type().const_int(b'x' as u64, false));

    assert!(matches!(alu.equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let ptr = context.ptr_type(AddressSpace::default()).const_null();
    let l = LlvmValue::Str(ptr);
    let r = LlvmValue::Str(ptr);

    assert!(matches!(alu.equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));
}

#[test]
fn not_equal_all_supported() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let l = LlvmValue::I64(context.i64_type().const_int(1, true));
    let r = LlvmValue::I64(context.i64_type().const_int(2, true));

    assert!(matches!(alu.not_equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let l = LlvmValue::F64(context.f64_type().const_float(1.0));
    let r = LlvmValue::F64(context.f64_type().const_float(2.0));

    assert!(matches!(alu.not_equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let l = LlvmValue::Bool(context.bool_type().const_int(1, false));
    let r = LlvmValue::Bool(context.bool_type().const_int(0, false));

    assert!(matches!(alu.not_equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let l = LlvmValue::Char(context.i8_type().const_int(b'a' as u64, false));
    let r = LlvmValue::Char(context.i8_type().const_int(b'b' as u64, false));

    assert!(matches!(alu.not_equal(&builder, &libc, l, r, span).unwrap(), LlvmValue::Bool(_)));

    let ptr = context.ptr_type(AddressSpace::default()).const_null();

    assert!(matches!(
        alu.not_equal(&builder, &libc, LlvmValue::Str(ptr), LlvmValue::Str(ptr), span).unwrap(),
        LlvmValue::Bool(_)
    ));
}

#[test]
fn equality_rejects_cross_category() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let alu = LlvmAlu::new(OverflowPolicy::Warn);
    let span = Span::default();

    let i64v = LlvmValue::I64(context.i64_type().const_int(1, true));
    let f64v = LlvmValue::F64(context.f64_type().const_float(1.0));
    let strv = LlvmValue::Str(context.ptr_type(AddressSpace::default()).const_null());
    let boolv = LlvmValue::Bool(context.bool_type().const_int(1, false));
    let charv = LlvmValue::Char(context.i8_type().const_int(b'a' as u64, false));

    assert!(alu.equal(&builder, &libc, i64v.clone(), f64v.clone(), span).is_err());

    assert!(alu.equal(&builder, &libc, i64v.clone(), strv.clone(), span).is_err());

    assert!(alu.equal(&builder, &libc, i64v.clone(), boolv.clone(), span).is_err());

    assert!(alu.equal(&builder, &libc, f64v.clone(), strv.clone(), span).is_err());

    assert!(alu.equal(&builder, &libc, boolv.clone(), charv.clone(), span).is_err());

    assert!(alu.equal(&builder, &libc, strv.clone(), charv.clone(), span).is_err());

    assert!(alu.not_equal(&builder, &libc, i64v.clone(), f64v.clone(), span).is_err());

    assert!(alu.not_equal(&builder, &libc, boolv.clone(), i64v.clone(), span).is_err());
}
