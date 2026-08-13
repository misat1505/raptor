use inkwell::context::Context;
use inkwell::types::{BasicTypeEnum, IntType, StructType};
use inkwell::values::{BasicValueEnum, FloatValue, IntValue, PointerValue};
use inkwell::AddressSpace;

use crate::ast::Type;
use crate::errors::{CompilerError, ErrorSeverity, IError};
use crate::lazy_stream_reader::Position;

#[derive(Debug, Clone)]
pub enum LlvmValue<'ctx> {
    I64(IntValue<'ctx>),
    F64(FloatValue<'ctx>),
    Str(PointerValue<'ctx>),
    Bool(IntValue<'ctx>),
    Vector(PointerValue<'ctx>, Box<Type>),
}

impl<'ctx> LlvmValue<'ctx> {
    pub fn to_type(&self) -> Type {
        match self {
            LlvmValue::I64(_) => Type::I64,
            LlvmValue::F64(_) => Type::F64,
            LlvmValue::Str(_) => Type::Str,
            LlvmValue::Bool(_) => Type::Bool,
            LlvmValue::Vector(_, inner) => Type::Vector(inner.clone()),
        }
    }

    pub fn as_basic_value_enum(&self) -> BasicValueEnum<'ctx> {
        match self {
            LlvmValue::I64(v) => (*v).into(),
            LlvmValue::F64(v) => (*v).into(),
            LlvmValue::Str(v) => (*v).into(),
            LlvmValue::Bool(v) => (*v).into(),
            LlvmValue::Vector(v, _) => (*v).into(),
        }
    }

    pub fn into_int_value(self, position: Position) -> Result<IntValue<'ctx>, Box<dyn IError>> {
        match self {
            LlvmValue::Bool(v) => Ok(v),
            other => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Expected a boolean condition, got '{:?}'.", other.to_type()),
                position,
            ))),
        }
    }

    pub fn from_basic_value_enum(value: BasicValueEnum<'ctx>, target_type: &Type) -> Self {
        match (target_type, value) {
            (Type::I64, BasicValueEnum::IntValue(v)) => LlvmValue::I64(v),
            (Type::F64, BasicValueEnum::FloatValue(v)) => LlvmValue::F64(v),
            (Type::Str, BasicValueEnum::PointerValue(v)) => LlvmValue::Str(v),
            (Type::Bool, BasicValueEnum::IntValue(v)) => LlvmValue::Bool(v),
            (Type::Vector(inner), BasicValueEnum::PointerValue(v)) => LlvmValue::Vector(v, inner.clone()),
            _ => unreachable!("BasicValueEnum variant should always match the declared Type"),
        }
    }

    pub fn type_to_basic_type_enum(ty: &Type, context: &'ctx Context) -> Option<BasicTypeEnum<'ctx>> {
        match ty {
            Type::I64 => Some(context.i64_type().into()),
            Type::F64 => Some(context.f64_type().into()),
            Type::Str => Some(context.ptr_type(AddressSpace::default()).into()),
            Type::Bool => Some(context.bool_type().into()),
            Type::Vector(_) => Some(context.ptr_type(AddressSpace::default()).into()),
            _ => None,
        }
    }

    pub fn vector_struct_type(context: &'ctx Context) -> StructType<'ctx> {
        let ptr_type = context.ptr_type(AddressSpace::default());
        let i64_type = context.i64_type();
        context.struct_type(&[ptr_type.into(), i64_type.into(), i64_type.into()], false)
    }

    pub fn element_byte_size(inner_type: &Type, i64_type: IntType<'ctx>) -> Result<inkwell::values::IntValue<'ctx>, Box<dyn IError>> {
        let size: u64 = match inner_type {
            Type::I64 => 8,
            Type::F64 => 8,
            Type::Bool => 1,
            Type::Str => 8,       // TODO: 64-bit platform only
            Type::Vector(_) => 8, // TODO: 64-bit platform only
            other => {
                return Err(Box::new(CompilerError::new(
                    ErrorSeverity::HIGH,
                    format!("Compiling vectors of type '{:?}' is not yet supported.", other),
                )))
            }
        };
        Ok(i64_type.const_int(size, false))
    }
}
