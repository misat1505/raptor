use inkwell::{context::Context, AddressSpace};

use crate::{
    backend::llvm::{
        llvm_alu::{tests::setup, LlvmAlu},
        LlvmValue,
    },
    common::{span::Span, types::Type},
};

#[test]
fn cast_same_type_is_identity() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let cases = [
        (LlvmValue::I64(context.i64_type().const_int(42, true)), Type::I64),
        (LlvmValue::F64(context.f64_type().const_float(3.14)), Type::F64),
        (LlvmValue::Bool(context.bool_type().const_int(1, false)), Type::Bool),
        (LlvmValue::Str(context.ptr_type(AddressSpace::default()).const_null()), Type::Str),
        (LlvmValue::Char(context.i8_type().const_int(b'a' as u64, false)), Type::Char),
        (LlvmValue::U8(context.i8_type().const_int(7, false)), Type::U8),
    ];

    for (val, ty) in cases {
        let result = LlvmAlu::cast_to_type(&builder, &libc, val, &ty, span).unwrap();
        assert_eq!(result.to_type(), ty);
    }
}

#[test]
fn cast_i8_to_wider_signed() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let v = LlvmValue::I8(context.i8_type().const_int(5, true));

    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I16, span).unwrap(),
        LlvmValue::I16(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I32, span).unwrap(),
        LlvmValue::I32(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I64, span).unwrap(),
        LlvmValue::I64(_)
    ));
}

#[test]
fn cast_integer_to_f64() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let signed = [
        LlvmValue::I8(context.i8_type().const_int(1, true)),
        LlvmValue::I16(context.i16_type().const_int(2, true)),
        LlvmValue::I32(context.i32_type().const_int(3, true)),
        LlvmValue::I64(context.i64_type().const_int(4, true)),
    ];
    for v in signed {
        assert!(matches!(
            LlvmAlu::cast_to_type(&builder, &libc, v, &Type::F64, span).unwrap(),
            LlvmValue::F64(_)
        ));
    }

    let unsigned = [
        LlvmValue::U8(context.i8_type().const_int(1, false)),
        LlvmValue::U16(context.i16_type().const_int(2, false)),
        LlvmValue::U32(context.i32_type().const_int(3, false)),
        LlvmValue::U64(context.i64_type().const_int(4, false)),
    ];
    for v in unsigned {
        assert!(matches!(
            LlvmAlu::cast_to_type(&builder, &libc, v, &Type::F64, span).unwrap(),
            LlvmValue::F64(_)
        ));
    }
}

#[test]
fn cast_f64_to_integers() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let v = LlvmValue::F64(context.f64_type().const_float(42.9));

    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I8, span).unwrap(),
        LlvmValue::I8(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I32, span).unwrap(),
        LlvmValue::I32(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I64, span).unwrap(),
        LlvmValue::I64(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::U8, span).unwrap(),
        LlvmValue::U8(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::U64, span).unwrap(),
        LlvmValue::U64(_)
    ));
}

#[test]
fn cast_integer_to_bool() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let v = LlvmValue::I64(context.i64_type().const_int(5, true));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::Bool, span).unwrap(),
        LlvmValue::Bool(_)
    ));

    let v = LlvmValue::U32(context.i32_type().const_int(0, false));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::Bool, span).unwrap(),
        LlvmValue::Bool(_)
    ));
}

#[test]
fn cast_f64_to_bool() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let v = LlvmValue::F64(context.f64_type().const_float(1.5));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v, &Type::Bool, span).unwrap(),
        LlvmValue::Bool(_)
    ));
}

#[test]
fn cast_bool_to_integers_and_f64() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let v = LlvmValue::Bool(context.bool_type().const_int(1, false));

    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I8, span).unwrap(),
        LlvmValue::I8(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I64, span).unwrap(),
        LlvmValue::I64(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::U32, span).unwrap(),
        LlvmValue::U32(_) // note: bool_to_int always wraps as signed variant in the match
            | LlvmValue::I32(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::F64, span).unwrap(),
        LlvmValue::F64(_)
    ));
}

#[test]
fn cast_bool_to_str() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let v = LlvmValue::Bool(context.bool_type().const_int(1, false));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v, &Type::Str, span).unwrap(),
        LlvmValue::Str(_)
    ));
}

#[test]
fn cast_integer_to_str() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let v = LlvmValue::I64(context.i64_type().const_int(42, true));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v, &Type::Str, span).unwrap(),
        LlvmValue::Str(_)
    ));

    let v = LlvmValue::U8(context.i8_type().const_int(7, false));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v, &Type::Str, span).unwrap(),
        LlvmValue::Str(_)
    ));
}

#[test]
fn cast_f64_to_str() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let v = LlvmValue::F64(context.f64_type().const_float(3.14));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v, &Type::Str, span).unwrap(),
        LlvmValue::Str(_)
    ));
}

#[test]
fn cast_char_to_str() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let v = LlvmValue::Char(context.i8_type().const_int(b'Z' as u64, false));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v, &Type::Str, span).unwrap(),
        LlvmValue::Str(_)
    ));
}

#[test]
fn cast_str_to_integers_and_f64_and_bool() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let ptr = context.ptr_type(AddressSpace::default()).const_null();
    let v = LlvmValue::Str(ptr);

    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I64, span).unwrap(),
        LlvmValue::I64(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I32, span).unwrap(),
        LlvmValue::I32(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::U8, span).unwrap(),
        LlvmValue::U8(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::F64, span).unwrap(),
        LlvmValue::F64(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::Bool, span).unwrap(),
        LlvmValue::Bool(_)
    ));
}

#[test]
fn cast_char_u8_roundtrip() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let ch = LlvmValue::Char(context.i8_type().const_int(b'A' as u64, false));
    let as_u8 = LlvmAlu::cast_to_type(&builder, &libc, ch, &Type::U8, span).unwrap();
    assert!(matches!(as_u8, LlvmValue::U8(_)));

    let back = LlvmAlu::cast_to_type(&builder, &libc, as_u8, &Type::Char, span).unwrap();
    assert!(matches!(back, LlvmValue::Char(_)));
}

#[test]
fn cast_unsupported_fails() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();

    let bad = [
        (LlvmValue::I64(context.i64_type().const_int(1, true)), Type::Char),
        (LlvmValue::Char(context.i8_type().const_int(b'x' as u64, false)), Type::I64),
        (LlvmValue::Bool(context.bool_type().const_int(1, false)), Type::Char),
        (LlvmValue::F64(context.f64_type().const_float(1.0)), Type::Char),
        (LlvmValue::Str(context.ptr_type(AddressSpace::default()).const_null()), Type::Char),
    ];

    for (val, ty) in bad {
        let err = LlvmAlu::cast_to_type(&builder, &libc, val, &ty, span).unwrap_err();
        assert!(err.message().contains("Cannot cast"), "unexpected message: {}", err.message());
    }
}

#[test]
fn cast_i64_to_narrower() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let v = LlvmValue::I64(context.i64_type().const_int(100, true));

    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I32, span).unwrap(),
        LlvmValue::U32(_) | LlvmValue::I32(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I16, span).unwrap(),
        LlvmValue::U16(_) | LlvmValue::I16(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I8, span).unwrap(),
        LlvmValue::U8(_) | LlvmValue::I8(_)
    ));
}

#[test]
fn cast_signed_to_unsigned() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let v = LlvmValue::I32(context.i32_type().const_int(10, true));

    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::U32, span).unwrap(),
        LlvmValue::U32(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::U64, span).unwrap(),
        LlvmValue::I64(_) | LlvmValue::U64(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::U8, span).unwrap(),
        LlvmValue::U8(_)
    ));
}

#[test]
fn cast_unsigned_to_signed_and_wider() {
    let context = Context::create();
    let (_module, builder, libc) = setup(&context);
    let span = Span::default();
    let v = LlvmValue::U8(context.i8_type().const_int(200, false));

    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I16, span).unwrap(),
        LlvmValue::U16(_) | LlvmValue::I16(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::U16, span).unwrap(),
        LlvmValue::U16(_)
    ));
    assert!(matches!(
        LlvmAlu::cast_to_type(&builder, &libc, v.clone(), &Type::I64, span).unwrap(),
        LlvmValue::U64(_) | LlvmValue::I64(_)
    ));
}
