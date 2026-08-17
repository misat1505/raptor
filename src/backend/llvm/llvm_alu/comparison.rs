use inkwell::builder::Builder;
use inkwell::values::{IntValue, PointerValue};
use inkwell::{FloatPredicate, IntPredicate};

use crate::backend::llvm::{libc_functions::LibcFunctions, llvm_alu::llvm_value::LlvmValue, llvm_alu::LlvmAlu};
use crate::common::{errors::IError, position::Position};

impl LlvmAlu {
    pub(in crate::backend::llvm::llvm_alu) fn strcmp<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        left: PointerValue<'ctx>,
        right: PointerValue<'ctx>,
        position: Position,
    ) -> Result<IntValue<'ctx>, Box<dyn IError>> {
        let call = builder
            .build_call(libc.strcmp_fn, &[left.into(), right.into()], "strcmp_call")
            .map_err(|err| Self::map_err(err, position))?;

        Ok(call.try_as_basic_value().unwrap_basic().into_int_value())
    }

    pub fn greater<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_compare(IntPredicate::SGT, l, r, "gttmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_compare(FloatPredicate::OGT, l, r, "gttmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (l, r) => Err(Self::type_error("greater", l, r, position)),
        }
    }

    pub fn greater_or_equal<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_compare(IntPredicate::SGE, l, r, "getmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_compare(FloatPredicate::OGE, l, r, "getmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (l, r) => Err(Self::type_error("greater or equal", l, r, position)),
        }
    }

    pub fn less<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_compare(IntPredicate::SLT, l, r, "lttmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_compare(FloatPredicate::OLT, l, r, "lttmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (l, r) => Err(Self::type_error("less", l, r, position)),
        }
    }

    pub fn less_or_equal<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_compare(IntPredicate::SLE, l, r, "letmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_compare(FloatPredicate::OLE, l, r, "letmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (l, r) => Err(Self::type_error("less or equal", l, r, position)),
        }
    }

    pub fn equal<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_compare(IntPredicate::EQ, l, r, "eqtmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_compare(FloatPredicate::OEQ, l, r, "eqtmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::Bool(l), LlvmValue::Bool(r)) => builder
                .build_int_compare(IntPredicate::EQ, l, r, "eqtmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::Str(l), LlvmValue::Str(r)) => {
                let cmp = Self::strcmp(builder, libc, l, r, position)?;
                let zero = cmp.get_type().const_int(0, false);
                builder
                    .build_int_compare(IntPredicate::EQ, cmp, zero, "eqtmp")
                    .map(LlvmValue::Bool)
                    .map_err(|err| Self::map_err(err, position))
            }
            (l, r) => Err(Self::type_error("equal", l, r, position)),
        }
    }

    pub fn not_equal<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_compare(IntPredicate::NE, l, r, "netmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_compare(FloatPredicate::ONE, l, r, "netmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::Bool(l), LlvmValue::Bool(r)) => builder
                .build_int_compare(IntPredicate::NE, l, r, "netmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::Str(l), LlvmValue::Str(r)) => {
                let cmp = Self::strcmp(builder, libc, l, r, position)?;
                let zero = cmp.get_type().const_int(0, false);
                builder
                    .build_int_compare(IntPredicate::NE, cmp, zero, "netmp")
                    .map(LlvmValue::Bool)
                    .map_err(|err| Self::map_err(err, position))
            }
            (l, r) => Err(Self::type_error("not equal", l, r, position)),
        }
    }
}
