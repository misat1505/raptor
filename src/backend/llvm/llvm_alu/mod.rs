mod arithmetic;
mod cast;
mod comparison;
pub mod llvm_value;
mod logical;
mod unary;

use inkwell::builder::BuilderError;

use crate::backend::llvm::llvm_alu::llvm_value::LlvmValue;
use crate::common::{
    errors::{CompilerError, ErrorSeverity, IError},
    position::Position,
};

pub struct LlvmAlu;

impl LlvmAlu {
    pub(in crate::backend::llvm::llvm_alu) fn map_err(err: BuilderError, position: Position) -> Box<dyn IError> {
        Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), position))
    }

    pub(in crate::backend::llvm::llvm_alu) fn type_error<'ctx>(
        op_name: &str,
        left: LlvmValue<'ctx>,
        right: LlvmValue<'ctx>,
        position: Position,
    ) -> Box<dyn IError> {
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

    pub(in crate::backend::llvm::llvm_alu) fn unary_type_error<'ctx>(op_name: &str, value: LlvmValue<'ctx>, position: Position) -> Box<dyn IError> {
        Box::new(CompilerError::at(
            ErrorSeverity::HIGH,
            format!("Cannot perform {} on type '{:?}'.", op_name, value.to_type()),
            position,
        ))
    }
}
