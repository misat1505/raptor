use inkwell::builder::{Builder, BuilderError};
use inkwell::{FloatPredicate, IntPredicate};

use crate::common::position::Position;
use crate::frontend::ast::Type;
use crate::{
    backend::llvm::{libc_functions::LibcFunctions, llvm_value::LlvmValue},
    common::errors::{CompilerError, ErrorSeverity, IError},
};

pub struct LlvmAlu;

impl LlvmAlu {
    fn map_err(err: BuilderError, position: Position) -> Box<dyn IError> {
        Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), position))
    }

    fn type_error<'ctx>(op_name: &str, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Box<dyn IError> {
        Box::new(CompilerError::at(
            ErrorSeverity::HIGH,
            format!(
                "Cannot perform {} between values of type '{:?}' and '{:?}'.",
                op_name,
                left.to_type(),
                right.to_type()
            ),
            position,
        ))
    }

    fn unary_type_error<'ctx>(op_name: &str, value: LlvmValue<'ctx>, position: Position) -> Box<dyn IError> {
        Box::new(CompilerError::at(
            ErrorSeverity::HIGH,
            format!("Cannot perform {} on type '{:?}'.", op_name, value.to_type()),
            position,
        ))
    }

    fn concat_strings<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        left: inkwell::values::PointerValue<'ctx>,
        right: inkwell::values::PointerValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let len_l_call = builder
            .build_call(libc.strlen_fn, &[left.into()], "strlen_l")
            .map_err(|err| Self::map_err(err, position))?;
        let len_l = len_l_call.try_as_basic_value().unwrap_basic().into_int_value();

        let len_r_call = builder
            .build_call(libc.strlen_fn, &[right.into()], "strlen_r")
            .map_err(|err| Self::map_err(err, position))?;
        let len_r = len_r_call.try_as_basic_value().unwrap_basic().into_int_value();

        let sum_len = builder
            .build_int_add(len_l, len_r, "concat_len")
            .map_err(|err| Self::map_err(err, position))?;

        let one = len_l.get_type().const_int(1, false);
        let total_len = builder
            .build_int_add(sum_len, one, "concat_total_len")
            .map_err(|err| Self::map_err(err, position))?;

        let malloc_call = builder
            .build_call(libc.malloc_fn, &[total_len.into()], "concat_buf")
            .map_err(|err| Self::map_err(err, position))?;
        let buf = malloc_call.try_as_basic_value().unwrap_basic().into_pointer_value();

        builder
            .build_call(libc.strcpy_fn, &[buf.into(), left.into()], "strcpy_call")
            .map_err(|err| Self::map_err(err, position))?;

        builder
            .build_call(libc.strcat_fn, &[buf.into(), right.into()], "strcat_call")
            .map_err(|err| Self::map_err(err, position))?;

        Ok(LlvmValue::Str(buf))
    }

    pub fn boolean_negate<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        value: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match value {
            LlvmValue::Bool(v) => builder
                .build_not(v, "bnottmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            other => Err(Self::unary_type_error("boolean negation", other, position)),
        }
    }

    pub fn arithmetic_negate<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        value: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match value {
            LlvmValue::I64(v) => builder
                .build_int_neg(v, "negtmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, position)),
            LlvmValue::F64(v) => builder
                .build_float_neg(v, "negtmp")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, position)),
            other => Err(Self::unary_type_error("arithmetic negation", other, position)),
        }
    }

    pub fn add<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_add(l, r, "addtmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_add(l, r, "addtmp")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::Str(l), LlvmValue::Str(r)) => Self::concat_strings(builder, libc, l, r, position),
            (l, r) => Err(Self::type_error("addition", l, r, position)),
        }
    }

    pub fn subtract<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_sub(l, r, "subtmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_sub(l, r, "subtmp")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, position)),
            (l, r) => Err(Self::type_error("subtraction", l, r, position)),
        }
    }

    pub fn multiplication<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_mul(l, r, "multmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_mul(l, r, "multmp")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, position)),
            (l, r) => Err(Self::type_error("multiplication", l, r, position)),
        }
    }

    pub fn division<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_signed_div(l, r, "divtmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => builder
                .build_float_div(l, r, "divtmp")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, position)),
            (l, r) => Err(Self::type_error("division", l, r, position)),
        }
    }

    pub fn modulo<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => builder
                .build_int_signed_rem(l, r, "remtmp")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, position)),
            (l, r) => Err(Self::type_error("modulo", l, r, position)),
        }
    }

    pub fn concatenation<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::Bool(l), LlvmValue::Bool(r)) => builder
                .build_and(l, r, "andtmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (l, r) => Err(Self::type_error("concatenation", l, r, position)),
        }
    }

    pub fn alternative<'ctx>(
        builder: &Builder<'ctx>,
        _libc: &LibcFunctions<'ctx>,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::Bool(l), LlvmValue::Bool(r)) => builder
                .build_or(l, r, "ortmp")
                .map(LlvmValue::Bool)
                .map_err(|err| Self::map_err(err, position)),
            (l, r) => Err(Self::type_error("alternative", l, r, position)),
        }
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

    fn strcmp<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        left: inkwell::values::PointerValue<'ctx>,
        right: inkwell::values::PointerValue<'ctx>,
        position: Position,
    ) -> Result<inkwell::values::IntValue<'ctx>, Box<dyn IError>> {
        let call = builder
            .build_call(libc.strcmp_fn, &[left.into(), right.into()], "strcmp_call")
            .map_err(|err| Self::map_err(err, position))?;

        Ok(call.try_as_basic_value().unwrap_basic().into_int_value())
    }

    pub fn cast_to_type<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: LlvmValue<'ctx>,
        to_type: &Type,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (value, to_type) {
            (LlvmValue::I64(v), Type::Str) => Self::int_to_str(builder, libc, v, position),
            (LlvmValue::F64(v), Type::Str) => Self::float_to_str(builder, libc, v, position),
            (LlvmValue::Bool(v), Type::Str) => Self::bool_to_str(builder, v, position),

            (LlvmValue::I64(v), Type::F64) => builder
                .build_signed_int_to_float(v, builder.get_insert_block().unwrap().get_context().f64_type(), "i64_to_f64")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, position)),

            (LlvmValue::F64(v), Type::I64) => builder
                .build_float_to_signed_int(v, builder.get_insert_block().unwrap().get_context().i64_type(), "f64_to_i64")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, position)),

            (LlvmValue::I64(v), Type::Bool) => {
                let zero = v.get_type().const_int(0, false);
                builder
                    .build_int_compare(IntPredicate::SGT, v, zero, "i64_to_bool")
                    .map(LlvmValue::Bool)
                    .map_err(|err| Self::map_err(err, position))
            }

            (LlvmValue::F64(v), Type::Bool) => {
                let zero = v.get_type().const_float(0.0);
                builder
                    .build_float_compare(FloatPredicate::OGT, v, zero, "f64_to_bool")
                    .map(LlvmValue::Bool)
                    .map_err(|err| Self::map_err(err, position))
            }

            (LlvmValue::Str(v), Type::I64) => {
                let call = builder
                    .build_call(libc.atoll_fn, &[v.into()], "atoll_call")
                    .map_err(|err| Self::map_err(err, position))?;
                Ok(LlvmValue::I64(call.try_as_basic_value().unwrap_basic().into_int_value()))
            }

            (LlvmValue::Str(v), Type::F64) => {
                let call = builder
                    .build_call(libc.atof_fn, &[v.into()], "atof_call")
                    .map_err(|err| Self::map_err(err, position))?;
                Ok(LlvmValue::F64(call.try_as_basic_value().unwrap_basic().into_float_value()))
            }

            (LlvmValue::Str(v), Type::Bool) => {
                let call = builder
                    .build_call(libc.strlen_fn, &[v.into()], "strlen_call")
                    .map_err(|err| Self::map_err(err, position))?;
                let len = call.try_as_basic_value().unwrap_basic().into_int_value();
                let zero = len.get_type().const_int(0, false);
                builder
                    .build_int_compare(IntPredicate::NE, len, zero, "str_to_bool")
                    .map(LlvmValue::Bool)
                    .map_err(|err| Self::map_err(err, position))
            }

            (LlvmValue::Bool(v), Type::I64) => builder
                .build_int_z_extend(v, builder.get_insert_block().unwrap().get_context().i64_type(), "bool_to_i64")
                .map(LlvmValue::I64)
                .map_err(|err| Self::map_err(err, position)),

            (LlvmValue::Bool(v), Type::F64) => builder
                .build_unsigned_int_to_float(v, builder.get_insert_block().unwrap().get_context().f64_type(), "bool_to_f64")
                .map(LlvmValue::F64)
                .map_err(|err| Self::map_err(err, position)),

            (value, target_type) => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Cannot cast '{:?}' to '{:?}'.", value.to_type(), target_type),
                position,
            ))),
        }
    }

    fn int_to_str<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: inkwell::values::IntValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let context = builder.get_insert_block().unwrap().get_context();
        let buffer_type = context.i8_type().array_type(24);
        let buffer_ptr = builder
            .build_alloca(buffer_type, "int_to_str_buf")
            .map_err(|err| Self::map_err(err, position))?;

        let format_str = builder
            .build_global_string_ptr("%lld", "int_fmt")
            .map_err(|err| Self::map_err(err, position))?;
        let size = context.i64_type().const_int(24, false);

        builder
            .build_call(
                libc.snprintf_fn,
                &[buffer_ptr.into(), size.into(), format_str.as_pointer_value().into(), value.into()],
                "snprintf_call",
            )
            .map_err(|err| Self::map_err(err, position))?;

        Ok(LlvmValue::Str(buffer_ptr))
    }

    fn float_to_str<'ctx>(
        builder: &Builder<'ctx>,
        libc: &LibcFunctions<'ctx>,
        value: inkwell::values::FloatValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let context = builder.get_insert_block().unwrap().get_context();
        let buffer_type = context.i8_type().array_type(32);
        let buffer_ptr = builder
            .build_alloca(buffer_type, "float_to_str_buf")
            .map_err(|err| Self::map_err(err, position))?;

        let format_str = builder
            .build_global_string_ptr("%g", "float_fmt")
            .map_err(|err| Self::map_err(err, position))?;
        let size = context.i64_type().const_int(32, false);

        builder
            .build_call(
                libc.snprintf_fn,
                &[buffer_ptr.into(), size.into(), format_str.as_pointer_value().into(), value.into()],
                "snprintf_call",
            )
            .map_err(|err| Self::map_err(err, position))?;

        Ok(LlvmValue::Str(buffer_ptr))
    }

    fn bool_to_str<'ctx>(
        builder: &Builder<'ctx>,
        value: inkwell::values::IntValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let true_str = builder
            .build_global_string_ptr("true", "true_str")
            .map_err(|err| Self::map_err(err, position))?;
        let false_str = builder
            .build_global_string_ptr("false", "false_str")
            .map_err(|err| Self::map_err(err, position))?;

        builder
            .build_select(value, true_str.as_pointer_value(), false_str.as_pointer_value(), "bool_to_str")
            .map(|v| LlvmValue::Str(v.into_pointer_value()))
            .map_err(|err| Self::map_err(err, position))
    }
}
