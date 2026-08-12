use inkwell::builder::{Builder, BuilderError};
use inkwell::{FloatPredicate, IntPredicate};

use crate::libc_functions::LibcFunctions;
use crate::{
    errors::{CompilerError, ErrorSeverity, IError},
    lazy_stream_reader::Position,
    llvm_value::LlvmValue,
};

pub struct LlvmAlu<'a, 'ctx> {
    builder: &'a Builder<'ctx>,
    libc: &'a LibcFunctions<'ctx>,
}

impl<'a, 'ctx> LlvmAlu<'a, 'ctx> {
    pub fn new(builder: &'a Builder<'ctx>, libc: &'a LibcFunctions<'ctx>) -> Self {
        LlvmAlu { builder, libc }
    }

    fn map_err(&self, err: BuilderError, position: Position) -> Box<dyn IError> {
        Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), position))
    }

    fn type_error(&self, op_name: &str, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Box<dyn IError> {
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

    fn unary_type_error(&self, op_name: &str, value: LlvmValue<'ctx>, position: Position) -> Box<dyn IError> {
        Box::new(CompilerError::at(
            ErrorSeverity::HIGH,
            format!("Cannot perform {} on type '{:?}'.", op_name, value.to_type()),
            position,
        ))
    }

    fn concat_strings(
        &self,
        left: inkwell::values::PointerValue<'ctx>,
        right: inkwell::values::PointerValue<'ctx>,
        position: Position,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let len_l_call = self
            .builder
            .build_call(self.libc.strlen_fn, &[left.into()], "strlen_l")
            .map_err(|err| self.map_err(err, position))?;
        let len_l = len_l_call.try_as_basic_value().unwrap_basic().into_int_value();

        let len_r_call = self
            .builder
            .build_call(self.libc.strlen_fn, &[right.into()], "strlen_r")
            .map_err(|err| self.map_err(err, position))?;
        let len_r = len_r_call.try_as_basic_value().unwrap_basic().into_int_value();

        let sum_len = self
            .builder
            .build_int_add(len_l, len_r, "concat_len")
            .map_err(|err| self.map_err(err, position))?;

        let one = len_l.get_type().const_int(1, false);
        let total_len = self
            .builder
            .build_int_add(sum_len, one, "concat_total_len")
            .map_err(|err| self.map_err(err, position))?;

        let malloc_call = self
            .builder
            .build_call(self.libc.malloc_fn, &[total_len.into()], "concat_buf")
            .map_err(|err| self.map_err(err, position))?;
        let buf = malloc_call.try_as_basic_value().unwrap_basic().into_pointer_value();

        self.builder
            .build_call(self.libc.strcpy_fn, &[buf.into(), left.into()], "strcpy_call")
            .map_err(|err| self.map_err(err, position))?;

        self.builder
            .build_call(self.libc.strcat_fn, &[buf.into(), right.into()], "strcat_call")
            .map_err(|err| self.map_err(err, position))?;

        Ok(LlvmValue::Str(buf))
    }

    pub fn boolean_negate(&self, value: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match value {
            LlvmValue::Bool(v) => self
                .builder
                .build_not(v, "bnottmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            other => Err(self.unary_type_error("boolean negation", other, position)),
        }
    }

    pub fn arithmetic_negate(&self, value: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match value {
            LlvmValue::I64(v) => self
                .builder
                .build_int_neg(v, "negtmp")
                .map(LlvmValue::I64)
                .map_err(|err| self.map_err(err, position)),
            LlvmValue::F64(v) => self
                .builder
                .build_float_neg(v, "negtmp")
                .map(LlvmValue::F64)
                .map_err(|err| self.map_err(err, position)),
            other => Err(self.unary_type_error("arithmetic negation", other, position)),
        }
    }

    pub fn add(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => self
                .builder
                .build_int_add(l, r, "addtmp")
                .map(LlvmValue::I64)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => self
                .builder
                .build_float_add(l, r, "addtmp")
                .map(LlvmValue::F64)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::Str(l), LlvmValue::Str(r)) => self.concat_strings(l, r, position),
            (l, r) => Err(self.type_error("addition", l, r, position)),
        }
    }

    pub fn subtract(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => self
                .builder
                .build_int_sub(l, r, "subtmp")
                .map(LlvmValue::I64)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => self
                .builder
                .build_float_sub(l, r, "subtmp")
                .map(LlvmValue::F64)
                .map_err(|err| self.map_err(err, position)),
            (l, r) => Err(self.type_error("subtraction", l, r, position)),
        }
    }

    pub fn multiplication(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => self
                .builder
                .build_int_mul(l, r, "multmp")
                .map(LlvmValue::I64)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => self
                .builder
                .build_float_mul(l, r, "multmp")
                .map(LlvmValue::F64)
                .map_err(|err| self.map_err(err, position)),
            (l, r) => Err(self.type_error("multiplication", l, r, position)),
        }
    }

    pub fn division(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => self
                .builder
                .build_int_signed_div(l, r, "divtmp")
                .map(LlvmValue::I64)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => self
                .builder
                .build_float_div(l, r, "divtmp")
                .map(LlvmValue::F64)
                .map_err(|err| self.map_err(err, position)),
            (l, r) => Err(self.type_error("division", l, r, position)),
        }
    }

    pub fn modulo(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => self
                .builder
                .build_int_signed_rem(l, r, "remtmp")
                .map(LlvmValue::I64)
                .map_err(|err| self.map_err(err, position)),
            (l, r) => Err(self.type_error("modulo", l, r, position)),
        }
    }

    pub fn concatenation(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::Bool(l), LlvmValue::Bool(r)) => self
                .builder
                .build_and(l, r, "andtmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (l, r) => Err(self.type_error("concatenation", l, r, position)),
        }
    }

    pub fn alternative(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::Bool(l), LlvmValue::Bool(r)) => self
                .builder
                .build_or(l, r, "ortmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (l, r) => Err(self.type_error("alternative", l, r, position)),
        }
    }

    pub fn greater(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => self
                .builder
                .build_int_compare(IntPredicate::SGT, l, r, "gttmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => self
                .builder
                .build_float_compare(FloatPredicate::OGT, l, r, "gttmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (l, r) => Err(self.type_error("greater", l, r, position)),
        }
    }

    pub fn greater_or_equal(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => self
                .builder
                .build_int_compare(IntPredicate::SGE, l, r, "getmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => self
                .builder
                .build_float_compare(FloatPredicate::OGE, l, r, "getmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (l, r) => Err(self.type_error("greater or equal", l, r, position)),
        }
    }

    pub fn less(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => self
                .builder
                .build_int_compare(IntPredicate::SLT, l, r, "lttmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => self
                .builder
                .build_float_compare(FloatPredicate::OLT, l, r, "lttmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (l, r) => Err(self.type_error("less", l, r, position)),
        }
    }

    pub fn less_or_equal(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => self
                .builder
                .build_int_compare(IntPredicate::SLE, l, r, "letmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => self
                .builder
                .build_float_compare(FloatPredicate::OLE, l, r, "letmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (l, r) => Err(self.type_error("less or equal", l, r, position)),
        }
    }

    pub fn equal(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => self
                .builder
                .build_int_compare(IntPredicate::EQ, l, r, "eqtmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => self
                .builder
                .build_float_compare(FloatPredicate::OEQ, l, r, "eqtmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::Bool(l), LlvmValue::Bool(r)) => self
                .builder
                .build_int_compare(IntPredicate::EQ, l, r, "eqtmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::Str(l), LlvmValue::Str(r)) => {
                let cmp = self.strcmp(l, r, position)?;
                self.builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        cmp,
                        self.builder.get_insert_block().unwrap().get_context().i32_type().const_int(0, false),
                        "eqtmp",
                    )
                    .map(LlvmValue::Bool)
                    .map_err(|err| self.map_err(err, position))
            }
            (l, r) => Err(self.type_error("equal", l, r, position)),
        }
    }

    pub fn not_equal(&self, left: LlvmValue<'ctx>, right: LlvmValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (left, right) {
            (LlvmValue::I64(l), LlvmValue::I64(r)) => self
                .builder
                .build_int_compare(IntPredicate::NE, l, r, "netmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::F64(l), LlvmValue::F64(r)) => self
                .builder
                .build_float_compare(FloatPredicate::ONE, l, r, "netmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::Bool(l), LlvmValue::Bool(r)) => self
                .builder
                .build_int_compare(IntPredicate::NE, l, r, "netmp")
                .map(LlvmValue::Bool)
                .map_err(|err| self.map_err(err, position)),
            (LlvmValue::Str(l), LlvmValue::Str(r)) => {
                let cmp = self.strcmp(l, r, position)?;
                let zero = cmp.get_type().const_int(0, false);
                self.builder
                    .build_int_compare(IntPredicate::NE, cmp, zero, "netmp")
                    .map(LlvmValue::Bool)
                    .map_err(|err| self.map_err(err, position))
            }
            (l, r) => Err(self.type_error("not equal", l, r, position)),
        }
    }

    fn strcmp(
        &self,
        left: inkwell::values::PointerValue<'ctx>,
        right: inkwell::values::PointerValue<'ctx>,
        position: Position,
    ) -> Result<inkwell::values::IntValue<'ctx>, Box<dyn IError>> {
        let call = self
            .builder
            .build_call(self.libc.strcmp_fn, &[left.into(), right.into()], "strcmp_call")
            .map_err(|err| self.map_err(err, position))?;

        Ok(call.try_as_basic_value().unwrap_basic().into_int_value())
    }

    pub fn cast_to_type(&self, value: LlvmValue<'ctx>, to_type: &crate::ast::Type, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        use crate::ast::Type;

        match (value, to_type) {
            (LlvmValue::I64(v), Type::Str) => self.int_to_str(v, position),
            (LlvmValue::F64(v), Type::Str) => self.float_to_str(v, position),
            (LlvmValue::Bool(v), Type::Str) => self.bool_to_str(v, position),

            (LlvmValue::I64(v), Type::F64) => self
                .builder
                .build_signed_int_to_float(v, self.builder.get_insert_block().unwrap().get_context().f64_type(), "i64_to_f64")
                .map(LlvmValue::F64)
                .map_err(|err| self.map_err(err, position)),

            (LlvmValue::F64(v), Type::I64) => self
                .builder
                .build_float_to_signed_int(v, self.builder.get_insert_block().unwrap().get_context().i64_type(), "f64_to_i64")
                .map(LlvmValue::I64)
                .map_err(|err| self.map_err(err, position)),

            (LlvmValue::I64(v), Type::Bool) => {
                let zero = v.get_type().const_int(0, false);
                self.builder
                    .build_int_compare(IntPredicate::SGT, v, zero, "i64_to_bool")
                    .map(LlvmValue::Bool)
                    .map_err(|err| self.map_err(err, position))
            }

            (LlvmValue::F64(v), Type::Bool) => {
                let zero = v.get_type().const_float(0.0);
                self.builder
                    .build_float_compare(FloatPredicate::OGT, v, zero, "f64_to_bool")
                    .map(LlvmValue::Bool)
                    .map_err(|err| self.map_err(err, position))
            }

            (LlvmValue::Str(v), Type::I64) => {
                let call = self
                    .builder
                    .build_call(self.libc.atoll_fn, &[v.into()], "atoll_call")
                    .map_err(|err| self.map_err(err, position))?;
                Ok(LlvmValue::I64(call.try_as_basic_value().unwrap_basic().into_int_value()))
            }

            (LlvmValue::Str(v), Type::F64) => {
                let call = self
                    .builder
                    .build_call(self.libc.atof_fn, &[v.into()], "atof_call")
                    .map_err(|err| self.map_err(err, position))?;
                Ok(LlvmValue::F64(call.try_as_basic_value().unwrap_basic().into_float_value()))
            }

            (LlvmValue::Str(v), Type::Bool) => {
                let call = self
                    .builder
                    .build_call(self.libc.strlen_fn, &[v.into()], "strlen_call")
                    .map_err(|err| self.map_err(err, position))?;
                let len = call.try_as_basic_value().unwrap_basic().into_int_value();
                let zero = len.get_type().const_int(0, false);
                self.builder
                    .build_int_compare(IntPredicate::NE, len, zero, "str_to_bool")
                    .map(LlvmValue::Bool)
                    .map_err(|err| self.map_err(err, position))
            }

            (LlvmValue::Bool(v), Type::I64) => self
                .builder
                .build_int_z_extend(v, self.builder.get_insert_block().unwrap().get_context().i64_type(), "bool_to_i64")
                .map(LlvmValue::I64)
                .map_err(|err| self.map_err(err, position)),

            (LlvmValue::Bool(v), Type::F64) => self
                .builder
                .build_unsigned_int_to_float(v, self.builder.get_insert_block().unwrap().get_context().f64_type(), "bool_to_f64")
                .map(LlvmValue::F64)
                .map_err(|err| self.map_err(err, position)),

            (value, target_type) => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Cannot cast '{:?}' to '{:?}'.", value.to_type(), target_type),
                position,
            ))),
        }
    }

    fn int_to_str(&self, value: inkwell::values::IntValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let context = self.builder.get_insert_block().unwrap().get_context();
        let buffer_type = context.i8_type().array_type(24);
        let buffer_ptr = self
            .builder
            .build_alloca(buffer_type, "int_to_str_buf")
            .map_err(|err| self.map_err(err, position))?;

        let format_str = self
            .builder
            .build_global_string_ptr("%lld", "int_fmt")
            .map_err(|err| self.map_err(err, position))?;

        let size = context.i64_type().const_int(24, false);

        self.builder
            .build_call(
                self.libc.snprintf_fn,
                &[buffer_ptr.into(), size.into(), format_str.as_pointer_value().into(), value.into()],
                "snprintf_call",
            )
            .map_err(|err| self.map_err(err, position))?;

        Ok(LlvmValue::Str(buffer_ptr))
    }

    fn float_to_str(&self, value: inkwell::values::FloatValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let context = self.builder.get_insert_block().unwrap().get_context();
        let buffer_type = context.i8_type().array_type(32);
        let buffer_ptr = self
            .builder
            .build_alloca(buffer_type, "float_to_str_buf")
            .map_err(|err| self.map_err(err, position))?;

        let format_str = self
            .builder
            .build_global_string_ptr("%g", "float_fmt")
            .map_err(|err| self.map_err(err, position))?;

        let size = context.i64_type().const_int(32, false);

        self.builder
            .build_call(
                self.libc.snprintf_fn,
                &[buffer_ptr.into(), size.into(), format_str.as_pointer_value().into(), value.into()],
                "snprintf_call",
            )
            .map_err(|err| self.map_err(err, position))?;

        Ok(LlvmValue::Str(buffer_ptr))
    }

    fn bool_to_str(&self, value: inkwell::values::IntValue<'ctx>, position: Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let true_str = self
            .builder
            .build_global_string_ptr("true", "true_str")
            .map_err(|err| self.map_err(err, position))?;
        let false_str = self
            .builder
            .build_global_string_ptr("false", "false_str")
            .map_err(|err| self.map_err(err, position))?;

        self.builder
            .build_select(value, true_str.as_pointer_value(), false_str.as_pointer_value(), "bool_to_str")
            .map(|v| LlvmValue::Str(v.into_pointer_value()))
            .map_err(|err| self.map_err(err, position))
    }
}
