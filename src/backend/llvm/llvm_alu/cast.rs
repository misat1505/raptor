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
            // -----------------------------------------------------------------
            // Same type
            // -----------------------------------------------------------------
            (value, target_type) if value.to_type() == *target_type => Ok(value),

            // -----------------------------------------------------------------
            // Integer -> Integer
            // -----------------------------------------------------------------
            (LlvmValue::I8(value), Type::I8) => Ok(LlvmValue::I8(value)),
            (LlvmValue::I8(value), Type::I16) => Self::int_extend_signed(builder, value, 16, span),
            (LlvmValue::I8(value), Type::I32) => Self::int_extend_signed(builder, value, 32, span),
            (LlvmValue::I8(value), Type::I64) => Self::int_extend_signed(builder, value, 64, span),

            (LlvmValue::I8(value), Type::U8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::I8(value), Type::U16) => Self::int_extend_signed(builder, value, 16, span),
            (LlvmValue::I8(value), Type::U32) => Self::int_extend_signed(builder, value, 32, span),
            (LlvmValue::I8(value), Type::U64) => Self::int_extend_signed(builder, value, 64, span),

            (LlvmValue::I16(value), Type::I8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::I16(value), Type::I16) => Ok(LlvmValue::I16(value)),
            (LlvmValue::I16(value), Type::I32) => Self::int_extend_signed(builder, value, 32, span),
            (LlvmValue::I16(value), Type::I64) => Self::int_extend_signed(builder, value, 64, span),

            (LlvmValue::I16(value), Type::U8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::I16(value), Type::U16) => Self::int_cast(builder, value, 16, false, span),
            (LlvmValue::I16(value), Type::U32) => Self::int_extend_signed(builder, value, 32, span),
            (LlvmValue::I16(value), Type::U64) => Self::int_extend_signed(builder, value, 64, span),

            (LlvmValue::I32(value), Type::I8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::I32(value), Type::I16) => Self::int_cast(builder, value, 16, false, span),
            (LlvmValue::I32(value), Type::I32) => Ok(LlvmValue::I32(value)),
            (LlvmValue::I32(value), Type::I64) => Self::int_extend_signed(builder, value, 64, span),

            (LlvmValue::I32(value), Type::U8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::I32(value), Type::U16) => Self::int_cast(builder, value, 16, false, span),
            (LlvmValue::I32(value), Type::U32) => Self::int_cast(builder, value, 32, false, span),
            (LlvmValue::I32(value), Type::U64) => Self::int_extend_signed(builder, value, 64, span),

            (LlvmValue::I64(value), Type::I8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::I64(value), Type::I16) => Self::int_cast(builder, value, 16, false, span),
            (LlvmValue::I64(value), Type::I32) => Self::int_cast(builder, value, 32, false, span),
            (LlvmValue::I64(value), Type::I64) => Ok(LlvmValue::I64(value)),

            (LlvmValue::I64(value), Type::U8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::I64(value), Type::U16) => Self::int_cast(builder, value, 16, false, span),
            (LlvmValue::I64(value), Type::U32) => Self::int_cast(builder, value, 32, false, span),
            (LlvmValue::I64(value), Type::U64) => Self::int_cast(builder, value, 64, false, span),

            // -----------------------------------------------------------------
            // Unsigned integer -> Integer
            // -----------------------------------------------------------------
            (LlvmValue::U8(value), Type::I8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::U8(value), Type::I16) => Self::int_extend_unsigned(builder, value, 16, span),
            (LlvmValue::U8(value), Type::I32) => Self::int_extend_unsigned(builder, value, 32, span),
            (LlvmValue::U8(value), Type::I64) => Self::int_extend_unsigned(builder, value, 64, span),

            (LlvmValue::U8(value), Type::U8) => Ok(LlvmValue::U8(value)),
            (LlvmValue::U8(value), Type::U16) => Self::int_extend_unsigned(builder, value, 16, span),
            (LlvmValue::U8(value), Type::U32) => Self::int_extend_unsigned(builder, value, 32, span),
            (LlvmValue::U8(value), Type::U64) => Self::int_extend_unsigned(builder, value, 64, span),

            (LlvmValue::U16(value), Type::I8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::U16(value), Type::I16) => Self::int_cast(builder, value, 16, false, span),
            (LlvmValue::U16(value), Type::I32) => Self::int_extend_unsigned(builder, value, 32, span),
            (LlvmValue::U16(value), Type::I64) => Self::int_extend_unsigned(builder, value, 64, span),

            (LlvmValue::U16(value), Type::U8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::U16(value), Type::U16) => Ok(LlvmValue::U16(value)),
            (LlvmValue::U16(value), Type::U32) => Self::int_extend_unsigned(builder, value, 32, span),
            (LlvmValue::U16(value), Type::U64) => Self::int_extend_unsigned(builder, value, 64, span),

            (LlvmValue::U32(value), Type::I8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::U32(value), Type::I16) => Self::int_cast(builder, value, 16, false, span),
            (LlvmValue::U32(value), Type::I32) => Self::int_cast(builder, value, 32, false, span),
            (LlvmValue::U32(value), Type::I64) => Self::int_extend_unsigned(builder, value, 64, span),

            (LlvmValue::U32(value), Type::U8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::U32(value), Type::U16) => Self::int_cast(builder, value, 16, false, span),
            (LlvmValue::U32(value), Type::U32) => Ok(LlvmValue::U32(value)),
            (LlvmValue::U32(value), Type::U64) => Self::int_extend_unsigned(builder, value, 64, span),

            (LlvmValue::U64(value), Type::I8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::U64(value), Type::I16) => Self::int_cast(builder, value, 16, false, span),
            (LlvmValue::U64(value), Type::I32) => Self::int_cast(builder, value, 32, false, span),
            (LlvmValue::U64(value), Type::I64) => Self::int_cast(builder, value, 64, false, span),

            (LlvmValue::U64(value), Type::U8) => Self::int_cast(builder, value, 8, false, span),
            (LlvmValue::U64(value), Type::U16) => Self::int_cast(builder, value, 16, false, span),
            (LlvmValue::U64(value), Type::U32) => Self::int_cast(builder, value, 32, false, span),
            (LlvmValue::U64(value), Type::U64) => Ok(LlvmValue::U64(value)),

            // -----------------------------------------------------------------
            // Integer -> F64
            // -----------------------------------------------------------------
            (LlvmValue::I8(value), Type::F64) => Self::signed_int_to_float(builder, value, span),

            (LlvmValue::I16(value), Type::F64) => Self::signed_int_to_float(builder, value, span),

            (LlvmValue::I32(value), Type::F64) => Self::signed_int_to_float(builder, value, span),

            (LlvmValue::I64(value), Type::F64) => Self::signed_int_to_float(builder, value, span),

            (LlvmValue::U8(value), Type::F64) => Self::unsigned_int_to_float(builder, value, span),

            (LlvmValue::U16(value), Type::F64) => Self::unsigned_int_to_float(builder, value, span),

            (LlvmValue::U32(value), Type::F64) => Self::unsigned_int_to_float(builder, value, span),

            (LlvmValue::U64(value), Type::F64) => Self::unsigned_int_to_float(builder, value, span),

            // -----------------------------------------------------------------
            // F64 -> Integer
            // -----------------------------------------------------------------
            (LlvmValue::F64(value), Type::I8) => Self::float_to_signed_int(builder, value, 8, span),

            (LlvmValue::F64(value), Type::I16) => Self::float_to_signed_int(builder, value, 16, span),

            (LlvmValue::F64(value), Type::I32) => Self::float_to_signed_int(builder, value, 32, span),

            (LlvmValue::F64(value), Type::I64) => Self::float_to_signed_int(builder, value, 64, span),

            (LlvmValue::F64(value), Type::U8) => Self::float_to_unsigned_int(builder, value, 8, span),

            (LlvmValue::F64(value), Type::U16) => Self::float_to_unsigned_int(builder, value, 16, span),

            (LlvmValue::F64(value), Type::U32) => Self::float_to_unsigned_int(builder, value, 32, span),

            (LlvmValue::F64(value), Type::U64) => Self::float_to_unsigned_int(builder, value, 64, span),

            // -----------------------------------------------------------------
            // Integer -> Bool
            // -----------------------------------------------------------------
            (LlvmValue::I8(value), Type::Bool) => Self::int_to_bool(builder, value, true, span),

            (LlvmValue::I16(value), Type::Bool) => Self::int_to_bool(builder, value, true, span),

            (LlvmValue::I32(value), Type::Bool) => Self::int_to_bool(builder, value, true, span),

            (LlvmValue::I64(value), Type::Bool) => Self::int_to_bool(builder, value, true, span),

            (LlvmValue::U8(value), Type::Bool) => Self::int_to_bool(builder, value, false, span),

            (LlvmValue::U16(value), Type::Bool) => Self::int_to_bool(builder, value, false, span),

            (LlvmValue::U32(value), Type::Bool) => Self::int_to_bool(builder, value, false, span),

            (LlvmValue::U64(value), Type::Bool) => Self::int_to_bool(builder, value, false, span),

            // -----------------------------------------------------------------
            // F64 -> Bool
            // -----------------------------------------------------------------
            (LlvmValue::F64(value), Type::Bool) => {
                let zero = value.get_type().const_float(0.0);

                builder
                    .build_float_compare(FloatPredicate::ONE, value, zero, "f64_to_bool")
                    .map(LlvmValue::Bool)
                    .map_err(|err| Self::map_err(err, span))
            }

            // -----------------------------------------------------------------
            // String -> Integer
            // -----------------------------------------------------------------
            (LlvmValue::Str(value), Type::I8) => Self::string_to_signed_int(builder, libc, value, 8, span),

            (LlvmValue::Str(value), Type::I16) => Self::string_to_signed_int(builder, libc, value, 16, span),

            (LlvmValue::Str(value), Type::I32) => Self::string_to_signed_int(builder, libc, value, 32, span),

            (LlvmValue::Str(value), Type::I64) => Self::string_to_signed_int(builder, libc, value, 64, span),

            (LlvmValue::Str(value), Type::U8) => Self::string_to_unsigned_int(builder, libc, value, 8, span),

            (LlvmValue::Str(value), Type::U16) => Self::string_to_unsigned_int(builder, libc, value, 16, span),

            (LlvmValue::Str(value), Type::U32) => Self::string_to_unsigned_int(builder, libc, value, 32, span),

            (LlvmValue::Str(value), Type::U64) => Self::string_to_unsigned_int(builder, libc, value, 64, span),

            // -----------------------------------------------------------------
            // String -> F64
            // -----------------------------------------------------------------
            (LlvmValue::Str(value), Type::F64) => {
                let call = builder
                    .build_call(libc.atof_fn, &[value.into()], "atof_call")
                    .map_err(|err| Self::map_err(err, span))?;

                let value = call.try_as_basic_value().basic().expect("atof should return a value").into_float_value();

                Ok(LlvmValue::F64(value))
            }

            // -----------------------------------------------------------------
            // String -> Bool
            // -----------------------------------------------------------------
            (LlvmValue::Str(value), Type::Bool) => {
                let call = builder
                    .build_call(libc.strlen_fn, &[value.into()], "strlen_call")
                    .map_err(|err| Self::map_err(err, span))?;

                let len = call.try_as_basic_value().basic().expect("strlen should return a value").into_int_value();

                let zero = len.get_type().const_zero();

                builder
                    .build_int_compare(IntPredicate::NE, len, zero, "str_to_bool")
                    .map(LlvmValue::Bool)
                    .map_err(|err| Self::map_err(err, span))
            }

            // -----------------------------------------------------------------
            // Integer -> String
            // -----------------------------------------------------------------
            (LlvmValue::I8(value), Type::Str) => Self::signed_int_to_str(builder, libc, value, 8, span),

            (LlvmValue::I16(value), Type::Str) => Self::signed_int_to_str(builder, libc, value, 16, span),

            (LlvmValue::I32(value), Type::Str) => Self::signed_int_to_str(builder, libc, value, 32, span),

            (LlvmValue::I64(value), Type::Str) => Self::signed_int_to_str(builder, libc, value, 64, span),

            (LlvmValue::U8(value), Type::Str) => Self::unsigned_int_to_str(builder, libc, value, 8, span),

            (LlvmValue::U16(value), Type::Str) => Self::unsigned_int_to_str(builder, libc, value, 16, span),

            (LlvmValue::U32(value), Type::Str) => Self::unsigned_int_to_str(builder, libc, value, 32, span),

            (LlvmValue::U64(value), Type::Str) => Self::unsigned_int_to_str(builder, libc, value, 64, span),

            // -----------------------------------------------------------------
            // F64 -> String
            // -----------------------------------------------------------------
            (LlvmValue::F64(value), Type::Str) => Self::float_to_str(builder, libc, value, span),

            // -----------------------------------------------------------------
            // Bool -> String
            // -----------------------------------------------------------------
            (LlvmValue::Bool(value), Type::Str) => Self::bool_to_str(builder, value, span),

            // -----------------------------------------------------------------
            // Bool -> Integer
            // -----------------------------------------------------------------
            (LlvmValue::Bool(value), Type::I8) => Self::bool_to_int(builder, value, 8, span),

            (LlvmValue::Bool(value), Type::I16) => Self::bool_to_int(builder, value, 16, span),

            (LlvmValue::Bool(value), Type::I32) => Self::bool_to_int(builder, value, 32, span),

            (LlvmValue::Bool(value), Type::I64) => Self::bool_to_int(builder, value, 64, span),

            (LlvmValue::Bool(value), Type::U8) => Self::bool_to_int(builder, value, 8, span),

            (LlvmValue::Bool(value), Type::U16) => Self::bool_to_int(builder, value, 16, span),

            (LlvmValue::Bool(value), Type::U32) => Self::bool_to_int(builder, value, 32, span),

            (LlvmValue::Bool(value), Type::U64) => Self::bool_to_int(builder, value, 64, span),

            // -----------------------------------------------------------------
            // Bool -> F64
            // -----------------------------------------------------------------
            (LlvmValue::Bool(value), Type::F64) => builder
                .build_unsigned_int_to_float(value, Self::context(builder).f64_type(), "bool_to_f64")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, span)),

            // -----------------------------------------------------------------
            // Unsupported
            // -----------------------------------------------------------------
            (value, target_type) => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Cannot cast '{:?}' to '{:?}'.", value.to_type(), target_type),
                span,
            ))),
        }
    }

    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    fn context<'ctx>(builder: &Builder<'ctx>) -> inkwell::context::ContextRef<'ctx> {
        builder
            .get_insert_block()
            .expect("builder should be positioned inside a block")
            .get_context()
    }

    fn int_type<'ctx>(builder: &Builder<'ctx>, bits: u32) -> Result<inkwell::types::IntType<'ctx>, Box<dyn IError>> {
        let bits = std::num::NonZeroU32::new(bits).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                "Integer type width cannot be zero.".to_string(),
                Span::default(), // albo przekazany span
            )) as Box<dyn IError>
        })?;

        Self::context(builder).custom_width_int_type(bits).map_err(|err| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Cannot create integer type: {}.", err),
                Span::default(), // albo przekazany span
            )) as Box<dyn IError>
        })
    }

    fn int_extend_signed<'ctx>(builder: &Builder<'ctx>, value: IntValue<'ctx>, bits: u32, span: Span) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let target = Self::int_type(builder, bits)?;

        builder
            .build_int_s_extend(value, target, "sext")
            .map(|value| match bits {
                8 => LlvmValue::I8(value),
                16 => LlvmValue::I16(value),
                32 => LlvmValue::I32(value),
                64 => LlvmValue::I64(value),
                _ => unreachable!(),
            })
            .map_err(|err| Self::map_err(err, span))
    }

    fn int_extend_unsigned<'ctx>(builder: &Builder<'ctx>, value: IntValue<'ctx>, bits: u32, span: Span) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let target = Self::int_type(builder, bits)?;

        builder
            .build_int_z_extend(value, target, "zext")
            .map(|value| match bits {
                8 => LlvmValue::U8(value),
                16 => LlvmValue::U16(value),
                32 => LlvmValue::U32(value),
                64 => LlvmValue::U64(value),
                _ => unreachable!(),
            })
            .map_err(|err| Self::map_err(err, span))
    }

    fn int_cast<'ctx>(
        builder: &Builder<'ctx>,
        value: IntValue<'ctx>,
        bits: u32,
        signed: bool,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let target = Self::int_type(builder, bits)?;
        let source_bits = value.get_type().get_bit_width();

        if source_bits == bits {
            return Ok(match bits {
                8 if signed => LlvmValue::I8(value),
                8 => LlvmValue::U8(value),
                16 if signed => LlvmValue::I16(value),
                16 => LlvmValue::U16(value),
                32 if signed => LlvmValue::I32(value),
                32 => LlvmValue::U32(value),
                64 if signed => LlvmValue::I64(value),
                64 => LlvmValue::U64(value),
                _ => unreachable!(),
            });
        }

        let casted = if source_bits > bits {
            builder
                .build_int_truncate(value, target, "trunc")
                .map_err(|err| Self::map_err(err, span))?
        } else if signed {
            builder
                .build_int_s_extend(value, target, "sext")
                .map_err(|err| Self::map_err(err, span))?
        } else {
            builder
                .build_int_z_extend(value, target, "zext")
                .map_err(|err| Self::map_err(err, span))?
        };

        Ok(match bits {
            8 if signed => LlvmValue::I8(casted),
            8 => LlvmValue::U8(casted),
            16 if signed => LlvmValue::I16(casted),
            16 => LlvmValue::U16(casted),
            32 if signed => LlvmValue::I32(casted),
            32 => LlvmValue::U32(casted),
            64 if signed => LlvmValue::I64(casted),
            64 => LlvmValue::U64(casted),
            _ => unreachable!(),
        })
    }

    fn signed_int_to_float<'ctx>(builder: &Builder<'ctx>, value: IntValue<'ctx>, span: Span) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        builder
            .build_signed_int_to_float(value, Self::context(builder).f64_type(), "int_to_f64")
            .map(LlvmValue::F64)
            .map_err(|err| Self::map_err(err, span))
    }

    fn unsigned_int_to_float<'ctx>(builder: &Builder<'ctx>, value: IntValue<'ctx>, span: Span) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        builder
            .build_unsigned_int_to_float(value, Self::context(builder).f64_type(), "uint_to_f64")
            .map(LlvmValue::F64)
            .map_err(|err| Self::map_err(err, span))
    }

    fn float_to_signed_int<'ctx>(
        builder: &Builder<'ctx>,
        value: FloatValue<'ctx>,
        bits: u32,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let target = Self::int_type(builder, bits)?;

        builder
            .build_float_to_signed_int(value, target, "f64_to_int")
            .map(|value| match bits {
                8 => LlvmValue::I8(value),
                16 => LlvmValue::I16(value),
                32 => LlvmValue::I32(value),
                64 => LlvmValue::I64(value),
                _ => unreachable!(),
            })
            .map_err(|err| Self::map_err(err, span))
    }

    fn float_to_unsigned_int<'ctx>(
        builder: &Builder<'ctx>,
        value: FloatValue<'ctx>,
        bits: u32,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let target = Self::int_type(builder, bits)?;

        builder
            .build_float_to_unsigned_int(value, target, "f64_to_uint")
            .map(|value| match bits {
                8 => LlvmValue::U8(value),
                16 => LlvmValue::U16(value),
                32 => LlvmValue::U32(value),
                64 => LlvmValue::U64(value),
                _ => unreachable!(),
            })
            .map_err(|err| Self::map_err(err, span))
    }

    fn int_to_bool<'ctx>(builder: &Builder<'ctx>, value: IntValue<'ctx>, signed: bool, span: Span) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let zero = value.get_type().const_zero();

        let predicate = if signed { IntPredicate::NE } else { IntPredicate::NE };

        builder
            .build_int_compare(predicate, value, zero, "int_to_bool")
            .map(LlvmValue::Bool)
            .map_err(|err| Self::map_err(err, span))
    }

    fn bool_to_int<'ctx>(builder: &Builder<'ctx>, value: IntValue<'ctx>, bits: u32, span: Span) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let target = Self::int_type(builder, bits)?;

        builder
            .build_int_z_extend(value, target, "bool_to_int")
            .map(|value| match bits {
                8 => LlvmValue::I8(value),
                16 => LlvmValue::I16(value),
                32 => LlvmValue::I32(value),
                64 => LlvmValue::I64(value),
                _ => unreachable!(),
            })
            .map_err(|err| Self::map_err(err, span))
    }

    fn string_to_signed_int<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: inkwell::values::PointerValue<'ctx>,
        bits: u32,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let call = builder
            .build_call(libc.atoll_fn, &[value.into()], "string_to_int")
            .map_err(|err| Self::map_err(err, span))?;

        let parsed = call.try_as_basic_value().basic().expect("atoll should return a value").into_int_value();

        if bits == 64 {
            return Ok(LlvmValue::I64(parsed));
        }

        Self::int_cast_signed(builder, parsed, bits, span)
    }

    fn string_to_unsigned_int<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: inkwell::values::PointerValue<'ctx>,
        bits: u32,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let call = builder
            .build_call(libc.atoll_fn, &[value.into()], "string_to_uint")
            .map_err(|err| Self::map_err(err, span))?;

        let parsed = call.try_as_basic_value().basic().expect("atoll should return a value").into_int_value();

        if bits == 64 {
            return Ok(LlvmValue::U64(parsed));
        }

        Self::int_cast_unsigned(builder, parsed, bits, span)
    }

    fn int_cast_signed<'ctx>(builder: &Builder<'ctx>, value: IntValue<'ctx>, bits: u32, span: Span) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let target = Self::int_type(builder, bits)?;

        let casted = if value.get_type().get_bit_width() > bits {
            builder
                .build_int_truncate(value, target, "int_trunc")
                .map_err(|err| Self::map_err(err, span))?
        } else {
            builder
                .build_int_s_extend(value, target, "int_sext")
                .map_err(|err| Self::map_err(err, span))?
        };

        Ok(match bits {
            8 => LlvmValue::I8(casted),
            16 => LlvmValue::I16(casted),
            32 => LlvmValue::I32(casted),
            64 => LlvmValue::I64(casted),
            _ => unreachable!(),
        })
    }

    fn int_cast_unsigned<'ctx>(builder: &Builder<'ctx>, value: IntValue<'ctx>, bits: u32, span: Span) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let target = Self::int_type(builder, bits)?;

        let casted = if value.get_type().get_bit_width() > bits {
            builder
                .build_int_truncate(value, target, "uint_trunc")
                .map_err(|err| Self::map_err(err, span))?
        } else {
            builder
                .build_int_z_extend(value, target, "uint_zext")
                .map_err(|err| Self::map_err(err, span))?
        };

        Ok(match bits {
            8 => LlvmValue::U8(casted),
            16 => LlvmValue::U16(casted),
            32 => LlvmValue::U32(casted),
            64 => LlvmValue::U64(casted),
            _ => unreachable!(),
        })
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

    fn signed_int_to_str<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: IntValue<'ctx>,
        bits: u32,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let context = Self::context(builder);

        // C variadic promotion: sub-i32 signed ints must be sign-extended
        // to i32 before being passed as a vararg to snprintf.
        let (promoted, fmt) = if bits < 32 {
            let i32_type = context.i32_type();
            let extended = builder
                .build_int_s_extend(value, i32_type, "vararg_sext")
                .map_err(|err| Self::map_err(err, span))?;
            (extended, "%d")
        } else if bits == 32 {
            (value, "%d")
        } else {
            (value, "%lld")
        };

        Self::int_to_str_via_snprintf(builder, libc, promoted, fmt, span)
    }

    fn unsigned_int_to_str<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: IntValue<'ctx>,
        bits: u32,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let context = Self::context(builder);

        // C variadic promotion: sub-i32 unsigned ints must be zero-extended
        // to i32 before being passed as a vararg to snprintf.
        let (promoted, fmt) = if bits < 32 {
            let i32_type = context.i32_type();
            let extended = builder
                .build_int_z_extend(value, i32_type, "vararg_zext")
                .map_err(|err| Self::map_err(err, span))?;
            (extended, "%u")
        } else if bits == 32 {
            (value, "%u")
        } else {
            (value, "%llu")
        };

        Self::int_to_str_via_snprintf(builder, libc, promoted, fmt, span)
    }

    /// Formats an already-promoted integer into a heap-allocated buffer via
    /// `snprintf`. The caller of the generated code owns the returned buffer.
    fn int_to_str_via_snprintf<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: IntValue<'ctx>,
        fmt: &str,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let context = Self::context(builder);
        let i64_type = context.i64_type();

        // 21 bytes comfortably fits any 64-bit integer, including sign and NUL.
        let buffer_size = i64_type.const_int(21, false);

        let buffer = builder
            .build_call(libc.malloc_fn, &[buffer_size.into()], "int_to_str_buf")
            .map_err(|err| Self::map_err(err, span))?
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();

        let format = builder.build_global_string_ptr(fmt, "int_fmt").map_err(|err| Self::map_err(err, span))?;

        builder
            .build_call(
                libc.snprintf_fn,
                &[buffer.into(), buffer_size.into(), format.as_pointer_value().into(), value.into()],
                "snprintf_call",
            )
            .map_err(|err| Self::map_err(err, span))?;

        Ok(LlvmValue::Str(buffer))
    }

    fn float_to_str<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: FloatValue<'ctx>,
        span: Span,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let context = Self::context(builder);
        let i64_type = context.i64_type();

        // 64 bytes comfortably fits a %f-formatted f64 (default 6 decimal places).
        let buffer_size = i64_type.const_int(64, false);

        let buffer = builder
            .build_call(libc.malloc_fn, &[buffer_size.into()], "f64_to_str_buf")
            .map_err(|err| Self::map_err(err, span))?
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();

        let format = builder.build_global_string_ptr("%f", "f64_fmt").map_err(|err| Self::map_err(err, span))?;

        builder
            .build_call(
                libc.snprintf_fn,
                &[buffer.into(), buffer_size.into(), format.as_pointer_value().into(), value.into()],
                "snprintf_call",
            )
            .map_err(|err| Self::map_err(err, span))?;

        Ok(LlvmValue::Str(buffer))
    }
}
