use inkwell::builder::Builder;

use crate::backend::llvm::{libc_functions::LibcFunctions, llvm_alu::llvm_value::LlvmValue, llvm_alu::LlvmAlu};
use crate::common::errors::IError;
use crate::common::span::Span;

impl LlvmAlu {
    pub fn concatenation<'ctx>(
        &self,
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::Bool(l), LlvmValue::Bool(r)) => builder
                .build_and(l, r, "andtmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, span)),
            (l, r) => Err(Self::type_error("concatenation", l, r, span)),
        }
    }

    pub fn alternative<'ctx>(
        &self,
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::Bool(l), LlvmValue::Bool(r)) => builder
                .build_or(l, r, "ortmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, span)),
            (l, r) => Err(Self::type_error("alternative", l, r, span)),
        }
    }
}
