use inkwell::{
    builder::Builder,
    values::{FloatValue, IntValue},
    FloatPredicate, IntPredicate,
};

use crate::{
    backend::llvm::{libc_functions::LibcFunctions, llvm_alu::llvm_value::LlvmValue, llvm_alu::LlvmAlu},
    common::{
        errors::{CompilerError, ErrorSeverity, IError},
        span::Span,
        types::Type,
    },
};

impl LlvmAlu {
    pub fn cast_to_type<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: LlvmValue<'ctx>,
        to_type: &Type,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (value, to_type) {
            // Integer / float / bool -> string
            (LlvmValue::I64(v), Type::Str) => Self::int_to_str(builder, libc, v, span),

            (LlvmValue::F64(v), Type::Str) => Self::float_to_str(builder, libc, v, span),

            (LlvmValue::Bool(v), Type::Str) => Self::bool_to_str(builder, v, span),

            // Integer -> float
            (LlvmValue::I64(v), Type::F64) => builder
                .build_signed_int_to_float(
                    v,
                    builder
                        .get_insert_block()
                        .expect("builder should be positioned inside a block")
                        .get_context()
                        .f64_type(),
                    "i64_to_f64",
                )
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, span)),

            // Float -> integer
            (LlvmValue::F64(v), Type::I64) => builder
                .build_float_to_signed_int(
                    v,
                    builder
                        .get_insert_block()
                        .expect("builder should be positioned inside a block")
                        .get_context()
                        .i64_type(),
                    "f64_to_i64",
                )
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, span)),

            // Integer -> bool
            (LlvmValue::I64(v), Type::Bool) => {
                let zero = v.get_type().const_int(0, false);

                builder
                    .build_int_compare(IntPredicate::SGT, v, zero, "i64_to_bool")
                    .map(LlvmValue::Bool)
                    .map_err(|err| Self::map_err(err, span))
            }

            // Float -> bool
            (LlvmValue::F64(v), Type::Bool) => {
                let zero = v.get_type().const_float(0.0);

                builder
                    .build_float_compare(FloatPredicate::OGT, v, zero, "f64_to_bool")
                    .map(LlvmValue::Bool)
                    .map_err(|err| Self::map_err(err, span))
            }

            // String -> integer
            (LlvmValue::Str(v), Type::I64) => {
                let call = builder
                    .build_call(libc.atoll_fn, &[v.into()], "atoll_call")
                    .map_err(|err| Self::map_err(err, span))?;

                let value = call.try_as_basic_value().basic().expect("atoll should return a value").into_int_value();

                Ok(LlvmValue::I64(value))
            }

            // String -> float
            (LlvmValue::Str(v), Type::F64) => {
                let call = builder
                    .build_call(libc.atof_fn, &[v.into()], "atof_call")
                    .map_err(|err| Self::map_err(err, span))?;

                let value = call.try_as_basic_value().basic().expect("atof should return a value").into_float_value();

                Ok(LlvmValue::F64(value))
            }

            // String -> bool
            (LlvmValue::Str(v), Type::Bool) => {
                let call = builder
                    .build_call(libc.strlen_fn, &[v.into()], "strlen_call")
                    .map_err(|err| Self::map_err(err, span))?;

                let len = call.try_as_basic_value().basic().expect("strlen should return a value").into_int_value();

                let zero = len.get_type().const_int(0, false);

                builder
                    .build_int_compare(IntPredicate::NE, len, zero, "str_to_bool")
                    .map(LlvmValue::Bool)
                    .map_err(|err| Self::map_err(err, span))
            }

            // Bool -> integer
            (LlvmValue::Bool(v), Type::I64) => builder
                .build_int_z_extend(
                    v,
                    builder
                        .get_insert_block()
                        .expect("builder should be positioned inside a block")
                        .get_context()
                        .i64_type(),
                    "bool_to_i64",
                )
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, span)),

            // Bool -> float
            (LlvmValue::Bool(v), Type::F64) => builder
                .build_unsigned_int_to_float(
                    v,
                    builder
                        .get_insert_block()
                        .expect("builder should be positioned inside a block")
                        .get_context()
                        .f64_type(),
                    "bool_to_f64",
                )
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, span)),

            // Unsupported conversion
            (value, target_type) => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Cannot cast '{:?}' to '{:?}'.", value.to_type(), target_type),
                span,
            ))),
        }
    }

    pub(in crate::backend::llvm::llvm_alu) fn int_to_str<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: IntValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let context = builder
            .get_insert_block()
            .expect("builder should be positioned inside a block")
            .get_context();

        let buffer_type = context.i8_type().array_type(24);

        let buffer_ptr = builder
            .build_alloca(buffer_type, "int_to_str_buf")
            .map_err(|err| Self::map_err(err, span))?;

        let format_str = builder
            .build_global_string_ptr("%lld", "int_fmt")
            .map_err(|err| Self::map_err(err, span))?;

        let size = context.i64_type().const_int(24, false);

        builder
            .build_call(
                libc.snprintf_fn,
                &[buffer_ptr.into(), size.into(), format_str.as_pointer_value().into(), value.into()],
                "snprintf_call",
            )
            .map_err(|err| Self::map_err(err, span))?;

        Ok(LlvmValue::Str(buffer_ptr))
    }

    pub(in crate::backend::llvm::llvm_alu) fn float_to_str<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: FloatValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let context = builder
            .get_insert_block()
            .expect("builder should be positioned inside a block")
            .get_context();

        let buffer_type = context.i8_type().array_type(32);

        let buffer_ptr = builder
            .build_alloca(buffer_type, "float_to_str_buf")
            .map_err(|err| Self::map_err(err, span))?;

        let format_str = builder
            .build_global_string_ptr("%g", "float_fmt")
            .map_err(|err| Self::map_err(err, span))?;

        let size = context.i64_type().const_int(32, false);

        builder
            .build_call(
                libc.snprintf_fn,
                &[buffer_ptr.into(), size.into(), format_str.as_pointer_value().into(), value.into()],
                "snprintf_call",
            )
            .map_err(|err| Self::map_err(err, span))?;

        Ok(LlvmValue::Str(buffer_ptr))
    }

    pub(in crate::backend::llvm::llvm_alu) fn bool_to_str<'ctx>(
        builder: &Builder<'ctx>,
        value: IntValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let true_str = builder
            .build_global_string_ptr("true", "true_str")
            .map_err(|err| Self::map_err(err, span))?;

        let false_str = builder
            .build_global_string_ptr("false", "false_str")
            .map_err(|err| Self::map_err(err, span))?;

        builder
            .build_select(value, true_str.as_pointer_value(), false_str.as_pointer_value(), "bool_to_str")
            .map(|value| LlvmValue::Str(value.into_pointer_value()))
            .map_err(|err| Self::map_err(err, span))
    }
}
