use inkwell::AddressSpace;
use inkwell::{builder::Builder, values::IntValue};

use crate::common::{errors::IError, span::Span};
use crate::{
    backend::llvm::{
        libc_functions::LibcFunctions,
        llvm_alu::{llvm_value::LlvmValue, LlvmAlu, OverflowPolicy},
    },
    common::errors::CompilerError,
};

impl LlvmAlu {
    pub fn boolean_negate<'ctx>(
        &self,
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

    fn emit_neg_overflow_check<'ctx>(
        &self,
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: IntValue<'ctx>,
        span: Span,
    ) -> Result<(), Box<dyn IError>> {
        if self.overflow_policy == OverflowPolicy::Ignore {
            return Ok(());
        }

        let context = Self::context(builder);
        let int_type = value.get_type();
        let bits = int_type.get_bit_width();

        // For signed negation, overflow can only happen when:
        //
        //     value == MIN
        //
        // because -MIN cannot be represented by the same signed integer type.
        //
        // Examples:
        //     i8  MIN = -128  -> -(-128) = 128  -> overflow
        //     i16 MIN = -32768 -> 32768          -> overflow
        //     i32 MIN = -2147483648 -> 2147483648
        //     i64 MIN = -9223372036854775808 -> 9223372036854775808
        let min_value: i128 = match bits {
            8 => i8::MIN as i128,
            16 => i16::MIN as i128,
            32 => i32::MIN as i128,
            64 => i64::MIN as i128,
            _ => {
                return Ok(());
            }
        };

        let min_const = int_type.const_int(min_value as u64, true);

        let is_min = builder
            .build_int_compare(inkwell::IntPredicate::EQ, value, min_const, "neg_overflow.check")
            .map_err(|err| Self::map_err(err, span))?;

        let function = builder
            .get_insert_block()
            .expect("builder should be positioned inside a function")
            .get_parent()
            .expect("basic block should belong to a function");

        let overflow_block = context.append_basic_block(function, "neg_overflow");
        let continue_block = context.append_basic_block(function, "neg_continue");

        builder
            .build_conditional_branch(is_min, overflow_block, continue_block)
            .map_err(|err| Self::map_err(err, span))?;

        builder.position_at_end(overflow_block);

        let error = CompilerError::at(
            self.overflow_policy.severity(),
            String::from("Arithmetic negation overflow: minimum signed integer cannot be negated"),
            span,
        );

        let message = format!("{}\n", error.get_stderr_message());

        let format_str = builder
            .build_global_string_ptr(&message, "neg_overflow.msg")
            .map_err(|err| Self::map_err(err, span))?;

        let stderr = builder
            .build_load(context.ptr_type(AddressSpace::default()), libc.stderr.as_pointer_value(), "stderr")
            .map_err(|err| Self::map_err(err, span))?;

        builder
            .build_call(
                libc.fprintf_fn,
                &[stderr.into(), format_str.as_pointer_value().into()],
                "neg_overflow.fprintf",
            )
            .map_err(|err| Self::map_err(err, span))?;

        match self.overflow_policy {
            OverflowPolicy::Error => {
                let i32_type = context.i32_type();

                builder
                    .build_call(libc.exit_fn, &[i32_type.const_int(1, false).into()], "neg_overflow.exit")
                    .map_err(|err| Self::map_err(err, span))?;

                builder.build_unreachable().map_err(|err| Self::map_err(err, span))?;
            }

            OverflowPolicy::Warn => {
                builder
                    .build_unconditional_branch(continue_block)
                    .map_err(|err| Self::map_err(err, span))?;
            }

            OverflowPolicy::Ignore => unreachable!(),
        }

        builder.position_at_end(continue_block);

        Ok(())
    }

    pub fn arithmetic_negate<'ctx>(
        &self,
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: LlvmValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match value {
            LlvmValue::I8(value) => {
                self.emit_neg_overflow_check(builder, libc, value, span)?;

                builder
                    .build_int_neg(value, "i8_neg")
                    .map(LlvmValue::I8)
                    .map_err(|err| Self::map_err(err, span))
            }

            LlvmValue::I16(value) => {
                self.emit_neg_overflow_check(builder, libc, value, span)?;

                builder
                    .build_int_neg(value, "i16_neg")
                    .map(LlvmValue::I16)
                    .map_err(|err| Self::map_err(err, span))
            }

            LlvmValue::I32(value) => {
                self.emit_neg_overflow_check(builder, libc, value, span)?;

                builder
                    .build_int_neg(value, "i32_neg")
                    .map(LlvmValue::I32)
                    .map_err(|err| Self::map_err(err, span))
            }

            LlvmValue::I64(value) => {
                self.emit_neg_overflow_check(builder, libc, value, span)?;

                builder
                    .build_int_neg(value, "i64_neg")
                    .map(LlvmValue::I64)
                    .map_err(|err| Self::map_err(err, span))
            }

            LlvmValue::F64(value) => builder
                .build_float_neg(value, "f64_neg")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, span)),

            other => Err(Self::unary_type_error("arithmetic negation", other, span)),
        }
    }
}
