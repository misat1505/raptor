use inkwell::values::PointerValue;
use inkwell::{builder::Builder, values::IntValue};

use crate::{
    backend::llvm::{libc_functions::LibcFunctions, llvm_alu::llvm_value::LlvmValue, llvm_alu::LlvmAlu},
    common::{errors::IError, span::Span},
};

impl LlvmAlu {
    fn concat_strings<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        left: PointerValue<'ctx>,
        right: PointerValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let len_l_call = builder
            .build_call(libc.strlen_fn, &[left.into()], "strlen_l")
            .map_err(|err| Self::map_err(err, span))?;

        let len_l = len_l_call
            .try_as_basic_value()
            .basic()
            .expect("strlen should return a value")
            .into_int_value();

        let len_r_call = builder
            .build_call(libc.strlen_fn, &[right.into()], "strlen_r")
            .map_err(|err| Self::map_err(err, span))?;

        let len_r = len_r_call
            .try_as_basic_value()
            .basic()
            .expect("strlen should return a value")
            .into_int_value();

        let total_len = builder
            .build_int_add(len_l, len_r, "concat_len")
            .and_then(|sum_len| builder.build_int_add(sum_len, len_l.get_type().const_int(1, false), "concat_total_len"))
            .map_err(|err| Self::map_err(err, span))?;

        let malloc_call = builder
            .build_call(libc.malloc_fn, &[total_len.into()], "concat_buf")
            .map_err(|err| Self::map_err(err, span))?;

        let buf = malloc_call
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();

        builder
            .build_call(libc.strcpy_fn, &[buf.into(), left.into()], "strcpy_call")
            .map_err(|err| Self::map_err(err, span))?;

        builder
            .build_call(libc.strcat_fn, &[buf.into(), right.into()], "strcat_call")
            .map_err(|err| Self::map_err(err, span))?;

        Ok(LlvmValue::Str(buf))
    }

    fn char_to_string<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        ch: IntValue<'ctx>,
        span: Span,
    ) -> Result<PointerValue<'ctx>, Box<dyn IError>> {
        let i64_type = ch.get_type().get_context().i64_type();

        // char + '\0'
        let size = i64_type.const_int(2, false);

        let malloc_call = builder
            .build_call(libc.malloc_fn, &[size.into()], "char_str_buf")
            .map_err(|err| Self::map_err(err, span))?;

        let buf = malloc_call
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();

        builder.build_store(buf, ch).map_err(|err| Self::map_err(err, span))?;

        let one = i64_type.const_int(1, false);

        let null_ptr = unsafe {
            builder
                .build_gep(ch.get_type(), buf, &[one], "char_str_null")
                .map_err(|err| Self::map_err(err, span))?
        };

        let zero = ch.get_type().const_zero();

        builder.build_store(null_ptr, zero).map_err(|err| Self::map_err(err, span))?;

        Ok(buf)
    }

    pub fn add<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I8(l), LlvmValue::I8(r)) => builder
                .build_int_add(l, r, "addtmp")
                .map(LlvmValue::I8)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I16(l), LlvmValue::I16(r)) => builder
                .build_int_add(l, r, "addtmp")
                .map(LlvmValue::I16)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I32(l), LlvmValue::I32(r)) => builder
                .build_int_add(l, r, "addtmp")
                .map(LlvmValue::I32)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_add(l, r, "addtmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U8(l), LlvmValue::U8(r)) => builder
                .build_int_add(l, r, "addtmp")
                .map(LlvmValue::U8)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U16(l), LlvmValue::U16(r)) => builder
                .build_int_add(l, r, "addtmp")
                .map(LlvmValue::U16)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U32(l), LlvmValue::U32(r)) => builder
                .build_int_add(l, r, "addtmp")
                .map(LlvmValue::U32)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U64(l), LlvmValue::U64(r)) => builder
                .build_int_add(l, r, "addtmp")
                .map(LlvmValue::U64)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_add(l, r, "addtmp")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::Char(l), LlvmValue::Char(r)) => {
                let left_str = Self::char_to_string(builder, libc, l, span)?;
                let right_str = Self::char_to_string(builder, libc, r, span)?;
                Self::concat_strings(builder, libc, left_str, right_str, span)
            }

            (LlvmValue::Str(l), LlvmValue::Char(r)) => {
                let right_str = Self::char_to_string(builder, libc, r, span)?;
                Self::concat_strings(builder, libc, l, right_str, span)
            }

            (LlvmValue::Char(l), LlvmValue::Str(r)) => {
                let left_str = Self::char_to_string(builder, libc, l, span)?;
                Self::concat_strings(builder, libc, left_str, r, span)
            }

            (LlvmValue::Str(l), LlvmValue::Str(r)) => Self::concat_strings(builder, libc, l, r, span),

            (l, r) => Err(Self::type_error("addition", l, r, span)),
        }
    }

    pub fn subtract<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I8(l), LlvmValue::I8(r)) => builder
                .build_int_sub(l, r, "subtmp")
                .map(LlvmValue::I8)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I16(l), LlvmValue::I16(r)) => builder
                .build_int_sub(l, r, "subtmp")
                .map(LlvmValue::I16)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I32(l), LlvmValue::I32(r)) => builder
                .build_int_sub(l, r, "subtmp")
                .map(LlvmValue::I32)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_sub(l, r, "subtmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U8(l), LlvmValue::U8(r)) => builder
                .build_int_sub(l, r, "subtmp")
                .map(LlvmValue::U8)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U16(l), LlvmValue::U16(r)) => builder
                .build_int_sub(l, r, "subtmp")
                .map(LlvmValue::U16)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U32(l), LlvmValue::U32(r)) => builder
                .build_int_sub(l, r, "subtmp")
                .map(LlvmValue::U32)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U64(l), LlvmValue::U64(r)) => builder
                .build_int_sub(l, r, "subtmp")
                .map(LlvmValue::U64)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_sub(l, r, "subtmp")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, span)),

            (l, r) => Err(Self::type_error("subtraction", l, r, span)),
        }
    }

    pub fn multiplication<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I8(l), LlvmValue::I8(r)) => builder
                .build_int_mul(l, r, "multmp")
                .map(LlvmValue::I8)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I16(l), LlvmValue::I16(r)) => builder
                .build_int_mul(l, r, "multmp")
                .map(LlvmValue::I16)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I32(l), LlvmValue::I32(r)) => builder
                .build_int_mul(l, r, "multmp")
                .map(LlvmValue::I32)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_mul(l, r, "multmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U8(l), LlvmValue::U8(r)) => builder
                .build_int_mul(l, r, "multmp")
                .map(LlvmValue::U8)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U16(l), LlvmValue::U16(r)) => builder
                .build_int_mul(l, r, "multmp")
                .map(LlvmValue::U16)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U32(l), LlvmValue::U32(r)) => builder
                .build_int_mul(l, r, "multmp")
                .map(LlvmValue::U32)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U64(l), LlvmValue::U64(r)) => builder
                .build_int_mul(l, r, "multmp")
                .map(LlvmValue::U64)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_mul(l, r, "multmp")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, span)),

            (l, r) => Err(Self::type_error("multiplication", l, r, span)),
        }
    }

    pub fn division<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I8(l), LlvmValue::I8(r)) => builder
                .build_int_signed_div(l, r, "divtmp")
                .map(LlvmValue::I8)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I16(l), LlvmValue::I16(r)) => builder
                .build_int_signed_div(l, r, "divtmp")
                .map(LlvmValue::I16)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I32(l), LlvmValue::I32(r)) => builder
                .build_int_signed_div(l, r, "divtmp")
                .map(LlvmValue::I32)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_signed_div(l, r, "divtmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U8(l), LlvmValue::U8(r)) => builder
                .build_int_unsigned_div(l, r, "divtmp")
                .map(LlvmValue::U8)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U16(l), LlvmValue::U16(r)) => builder
                .build_int_unsigned_div(l, r, "divtmp")
                .map(LlvmValue::U16)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U32(l), LlvmValue::U32(r)) => builder
                .build_int_unsigned_div(l, r, "divtmp")
                .map(LlvmValue::U32)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U64(l), LlvmValue::U64(r)) => builder
                .build_int_unsigned_div(l, r, "divtmp")
                .map(LlvmValue::U64)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_div(l, r, "divtmp")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, span)),

            (l, r) => Err(Self::type_error("division", l, r, span)),
        }
    }

    pub fn modulo<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I8(l), LlvmValue::I8(r)) => builder
                .build_int_signed_rem(l, r, "remtmp")
                .map(LlvmValue::I8)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I16(l), LlvmValue::I16(r)) => builder
                .build_int_signed_rem(l, r, "remtmp")
                .map(LlvmValue::I16)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I32(l), LlvmValue::I32(r)) => builder
                .build_int_signed_rem(l, r, "remtmp")
                .map(LlvmValue::I32)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_signed_rem(l, r, "remtmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U8(l), LlvmValue::U8(r)) => builder
                .build_int_unsigned_rem(l, r, "remtmp")
                .map(LlvmValue::U8)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U16(l), LlvmValue::U16(r)) => builder
                .build_int_unsigned_rem(l, r, "remtmp")
                .map(LlvmValue::U16)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U32(l), LlvmValue::U32(r)) => builder
                .build_int_unsigned_rem(l, r, "remtmp")
                .map(LlvmValue::U32)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::U64(l), LlvmValue::U64(r)) => builder
                .build_int_unsigned_rem(l, r, "remtmp")
                .map(LlvmValue::U64)
                .map_err(|err| Self::map_err(err, span)),

            (l, r) => Err(Self::type_error("modulo", l, r, span)),
        }
    }
}
