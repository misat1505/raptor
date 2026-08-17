use inkwell::builder::Builder;

use super::Compiler;
use crate::common::visitor::Visitor;
use crate::{
    backend::llvm::{libc_functions::LibcFunctions, llvm_alu::llvm_value::LlvmValue, llvm_alu::LlvmAlu},
    common::{
        errors::{CompilerError, ErrorSeverity, IError},
        position::Position,
    },
    frontend::ast::{Expression, Node},
};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(in crate::backend::llvm::compiler) fn build_binary_op<F>(
        &mut self,
        lhs: &'a Node<Expression>,
        rhs: &'a Node<Expression>,
        op: F,
        position: Position,
    ) -> Result<(), Box<dyn IError>>
    where
        F: Fn(&Builder<'ctx>, &LibcFunctions<'ctx>, LlvmValue<'ctx>, LlvmValue<'ctx>, Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>>,
    {
        self.visit_expression(lhs)?;
        let left_value = self.read_last_value()?;

        self.visit_expression(rhs)?;
        let right_value = self.read_last_value()?;

        let value = op(&self.builder, &self.libc, left_value, right_value, position)?;

        self.last_value = Some(value);

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn build_unary_op<F>(
        &mut self,
        value: &'a Node<Expression>,
        op: F,
        position: Position,
    ) -> Result<(), Box<dyn IError>>
    where
        F: Fn(&Builder<'ctx>, &LibcFunctions<'ctx>, LlvmValue<'ctx>, Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>>,
    {
        self.visit_expression(value)?;
        let computed_value = self.read_last_value()?;

        let value = op(&self.builder, &self.libc, computed_value, position)?;

        self.last_value = Some(value);

        Ok(())
    }

    /// Treść dawnego `Visitor::visit_expression`. `visit_expression` w `visitor.rs`
    /// tylko woła tę metodę — pojedynczy `impl Visitor for Compiler` musi być
    /// w jednym miejscu, więc logika żyje tutaj, a delegacja w visitor.rs.
    pub(in crate::backend::llvm::compiler) fn compile_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        self.position = expression.position;

        match &expression.value {
            Expression::FunctionCall { identifier, arguments } => {
                let name = identifier.value.as_str();

                if let Some(std_function) = self.program.std_functions.get(name) {
                    return (std_function.compile)(self, arguments, expression.position);
                }

                return self.build_function_call(identifier, arguments, expression.position);
            }
            Expression::Literal(literal) => self.visit_literal(literal),
            Expression::Variable(variable) => self.visit_variable(variable, expression.position),
            Expression::BooleanNegation(expr) => self.build_unary_op(expr, LlvmAlu::boolean_negate, expression.position),
            Expression::ArithmeticNegation(expr) => self.build_unary_op(expr, LlvmAlu::arithmetic_negate, expression.position),
            Expression::Addition(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::add, expression.position),
            Expression::Subtraction(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::subtract, expression.position),
            Expression::Multiplication(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::multiplication, expression.position),
            Expression::Division(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::division, expression.position),
            Expression::Modulo(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::modulo, expression.position),
            Expression::Concatenation(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::concatenation, expression.position),
            Expression::Alternative(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::alternative, expression.position),
            Expression::Greater(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::greater, expression.position),
            Expression::GreaterEqual(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::greater_or_equal, expression.position),
            Expression::Less(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::less, expression.position),
            Expression::LessEqual(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::less_or_equal, expression.position),
            Expression::Equal(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::equal, expression.position),
            Expression::NotEqual(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::not_equal, expression.position),
            Expression::Casting { value, to_type } => {
                self.visit_expression(value)?;
                let source_value = self.read_last_value()?;

                self.last_value = Some(LlvmAlu::cast_to_type(
                    &self.builder,
                    &self.libc,
                    source_value,
                    &to_type.value,
                    expression.position,
                )?);

                Ok(())
            }
            Expression::Index { collection, index } => {
                self.visit_expression(collection)?;
                let collection_value = self.read_last_value()?;

                let vector_ptr = match &collection_value {
                    LlvmValue::Vector(ptr, _) => *ptr,
                    other => {
                        return Err(Box::new(CompilerError::at(
                            ErrorSeverity::HIGH,
                            format!("Cannot index into type '{:?}'.", other.to_type()),
                            expression.position,
                        )) as Box<dyn IError>)
                    }
                };
                let collection_type = collection_value.to_type();

                let (element_ptr, element_type) =
                    self.resolve_indexed_element(vector_ptr, &collection_type, std::slice::from_ref(index.as_ref()), expression.position)?;

                let element_llvm_type = LlvmValue::type_to_basic_type_enum(&element_type, self.context).ok_or_else(|| {
                    Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("Compiling vectors of type '{:?}' is not yet supported.", element_type),
                        expression.position,
                    )) as Box<dyn IError>
                })?;

                let raw_value = self
                    .builder
                    .build_load(element_llvm_type, element_ptr, "idx.load")
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), expression.position)) as Box<dyn IError>)?;

                self.last_value = Some(LlvmValue::from_basic_value_enum(raw_value, &element_type));

                Ok(())
            }

            Expression::Vector(elements) => {
                let (vector_ptr, inner_type) = self.build_vector_expression(elements, expression.position)?;
                self.last_value = Some(LlvmValue::Vector(vector_ptr, Box::new(inner_type)));
                Ok(())
            }
        }
    }
}
