use inkwell::builder::Builder;

use crate::backend::llvm::{libc_functions::LibcFunctions, llvm_alu::llvm_value::LlvmValue, llvm_alu::LlvmAlu};
use crate::common::{errors::IError, span::Span};

impl LlvmAlu {
    pub fn boolean_negate<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        value: LlvmValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match value {
            LlvmValue::Bool(v) => builder
                .build_not(v, "bnottmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, span)),
            other => Err(Self::unary_type_error("boolean negation", other, span)),
        }
    }

    pub fn arithmetic_negate<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        value: LlvmValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match value {
            LlvmValue::I8(value) => builder
                .build_int_neg(value, "i8_neg")
                .map(LlvmValue::I8)
                .map_err(|err| Self::map_err(err, span)),

            LlvmValue::I16(value) => builder
                .build_int_neg(value, "i16_neg")
                .map(LlvmValue::I16)
                .map_err(|err| Self::map_err(err, span)),

            LlvmValue::I32(value) => builder
                .build_int_neg(value, "i32_neg")
                .map(LlvmValue::I32)
                .map_err(|err| Self::map_err(err, span)),

            LlvmValue::I64(value) => builder
                .build_int_neg(value, "i64_neg")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, span)),

            LlvmValue::F64(value) => builder
                .build_float_neg(value, "f64_neg")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, span)),

            other => Err(Self::unary_type_error("arithmetic negation", other, span)),
        }
    }
}
