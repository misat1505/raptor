use inkwell::builder::Builder;
use inkwell::values::PointerValue;

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

        let sum_len = builder
            .build_int_add(len_l, len_r, "concat_len")
            .map_err(|err| Self::map_err(err, span))?;

        // +1 for terminating '\0'
        let one = len_l.get_type().const_int(1, false);

        let total_len = builder
            .build_int_add(sum_len, one, "concat_total_len")
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

    pub fn add<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_add(l, r, "addtmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, span)),

            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_add(l, r, "addtmp")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, span)),

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
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_sub(l, r, "subtmp")
                .map(LlvmValue::I64)
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
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_mul(l, r, "multmp")
                .map(LlvmValue::I64)
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
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_signed_div(l, r, "divtmp")
                .map(LlvmValue::I64)
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
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_signed_rem(l, r, "remtmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, span)),

            (l, r) => Err(Self::type_error("modulo", l, r, span)),
        }
    }
}
