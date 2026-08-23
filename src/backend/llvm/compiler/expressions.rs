use inkwell::builder::Builder;
use inkwell::AddressSpace;

use super::Compiler;
use crate::common::types::Type;
use crate::common::visitor::Visitor;
use crate::{
    backend::llvm::{
        libc_functions::LibcFunctions,
        llvm_alu::{llvm_value::LlvmValue, LlvmAlu},
    },
    common::{
        errors::{CompilerError, ErrorSeverity, IError},
        span::Span,
    },
    frontend::ast::{Expression, Node},
};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(in crate::backend::llvm::compiler) fn build_binary_op<F>(
        &mut self,
        lhs: &'a Node<Expression>,
        rhs: &'a Node<Expression>,
        op: F,
        span: Span,
    ) -> Result<(), Box<dyn IError>>
    where
        F: Fn(&Builder<'ctx>, &LibcFunctions<'ctx>, LlvmValue<'ctx>, LlvmValue<'ctx>, Span) -> Result<LlvmValue<'ctx>, Box<dyn IError>>,
    {
        self.visit_expression(lhs)?;
        let left_value = self.read_last_value()?;

        self.visit_expression(rhs)?;
        let right_value = self.read_last_value()?;

        let value = op(&self.builder, &self.libc, left_value, right_value, span)?;

        self.last_value = Some(value);

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn build_unary_op<F>(
        &mut self,
        value: &'a Node<Expression>,
        op: F,
        span: Span,
    ) -> Result<(), Box<dyn IError>>
    where
        F: Fn(&Builder<'ctx>, &LibcFunctions<'ctx>, LlvmValue<'ctx>, Span) -> Result<LlvmValue<'ctx>, Box<dyn IError>>,
    {
        self.visit_expression(value)?;
        let computed_value = self.read_last_value()?;

        let value = op(&self.builder, &self.libc, computed_value, span)?;

        self.last_value = Some(value);

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn compile_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        let span = expression.span;

        match &expression.value {
            Expression::FunctionCall { identifier, arguments } => {
                let name = identifier.value.as_str();

                if let Some(std_function) = self.program.std_functions.get(name) {
                    return (std_function.compile)(self, arguments, span);
                }

                self.build_function_call(identifier, arguments, span)
            }

            Expression::Literal(literal) => self.visit_literal(literal),

            Expression::Variable(variable) => self.visit_variable(variable, span),

            Expression::BooleanNegation(expr) => self.build_unary_op(expr, LlvmAlu::boolean_negate, span),

            Expression::ArithmeticNegation(expr) => self.build_unary_op(expr, LlvmAlu::arithmetic_negate, span),

            Expression::Addition(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::add, span),

            Expression::Subtraction(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::subtract, span),

            Expression::Multiplication(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::multiplication, span),

            Expression::Division(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::division, span),

            Expression::Modulo(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::modulo, span),

            Expression::Concatenation(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::concatenation, span),

            Expression::Alternative(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::alternative, span),

            Expression::Greater(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::greater, span),

            Expression::GreaterEqual(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::greater_or_equal, span),

            Expression::Less(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::less, span),

            Expression::LessEqual(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::less_or_equal, span),

            Expression::Equal(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::equal, span),

            Expression::NotEqual(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::not_equal, span),

            Expression::Casting { value, to_type } => {
                self.visit_expression(value)?;
                let source_value = self.read_last_value()?;

                self.last_value = Some(LlvmAlu::cast_to_type(&self.builder, &self.libc, source_value, &to_type.value, span)?);

                Ok(())
            }

            Expression::Index { collection, index } => {
                self.visit_expression(collection)?;
                let collection_value = self.read_last_value()?;

                match &collection_value {
                    LlvmValue::Vector(vector_ptr, _) => {
                        let collection_type = collection_value.to_type();

                        let struct_type = LlvmValue::vector_struct_type(self.context);
                        let ptr_type = self.context.ptr_type(AddressSpace::default());

                        let inner_type = match &collection_type {
                            Type::Vector(inner) => (**inner).clone(),

                            other => {
                                return Err(Box::new(CompilerError::at(
                                    ErrorSeverity::HIGH,
                                    format!("Cannot index into type '{:?}'.", other),
                                    span,
                                )));
                            }
                        };

                        let data_field = self
                            .builder
                            .build_struct_gep(struct_type, *vector_ptr, 0, "idx.data")
                            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

                        let data = self
                            .builder
                            .build_load(ptr_type, data_field, "idx.data.val")
                            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?
                            .into_pointer_value();

                        self.visit_expression(index)?;

                        let index_value = self.read_last_value()?;
                        let index_int = index_value.into_i64_value(index.span)?;

                        let element_llvm_type = LlvmValue::type_to_basic_type_enum(&inner_type, self.context).ok_or_else(|| {
                            Box::new(CompilerError::at(
                                ErrorSeverity::HIGH,
                                format!("Compiling vectors of type '{:?}' is not yet supported.", inner_type),
                                index.span,
                            )) as Box<dyn IError>
                        })?;

                        let element_ptr = unsafe {
                            self.builder
                                .build_gep(element_llvm_type, data, &[index_int], "idx.elem")
                                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?
                        };

                        let raw_value = self
                            .builder
                            .build_load(element_llvm_type, element_ptr, "idx.load")
                            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

                        self.last_value = Some(LlvmValue::from_basic_value_enum(raw_value, &inner_type));

                        Ok(())
                    }

                    LlvmValue::Str(str_ptr) => {
                        self.visit_expression(index)?;

                        let index_value = self.read_last_value()?;
                        let index_int = index_value.into_i64_value(index.span)?;

                        let i8_type = self.context.i8_type();

                        let element_ptr = unsafe {
                            self.builder
                                .build_gep(i8_type, *str_ptr, &[index_int], "str.idx.ptr")
                                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?
                        };

                        let raw_value = self
                            .builder
                            .build_load(i8_type, element_ptr, "str.idx.load")
                            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

                        self.last_value = Some(LlvmValue::from_basic_value_enum(raw_value, &Type::Char));

                        Ok(())
                    }

                    other => Err(Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("Cannot index into type '{:?}'.", other.to_type()),
                        span,
                    )) as Box<dyn IError>),
                }
            }

            Expression::Vector(elements) => {
                let (vector_ptr, inner_type) = self.build_vector_expression(elements, span)?;

                self.last_value = Some(LlvmValue::Vector(vector_ptr, Box::new(inner_type)));

                Ok(())
            }

            Expression::StructLiteral { .. } => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("Visiting struct literal in compiler is not implemented yet."),
                expression.span,
            )) as Box<dyn IError>)?,
            Expression::FieldAccess { .. } => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("Visiting field access in compiler is not implemented yet."),
                expression.span,
            )) as Box<dyn IError>)?,
        }
    }
}
