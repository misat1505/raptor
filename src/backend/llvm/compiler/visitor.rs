use super::Compiler;
use crate::common::visitor::Visitor;
use crate::{
    backend::llvm::llvm_alu::llvm_value::LlvmValue,
    common::{
        errors::{CompilerError, ErrorSeverity, IError},
        span::Span,
        types::Type,
    },
    frontend::ast::{Argument, Block, Expression, Literal, Node, Parameter, Program, Statement, SwitchCase, SwitchExpression},
};

/// Jedyne miejsce w projekcie z `impl Visitor for Compiler` — Rust nie pozwala
/// implementować jednego trait-a dla typu w kilku osobnych blokach `impl`, więc
/// `visit_expression`/`visit_statement` tylko delegują do `compile_expression`
/// (expressions.rs) i `compile_statement` (statements.rs), gdzie faktycznie
/// żyje ich logika.
impl<'a, 'ctx> Visitor<'a> for Compiler<'a, 'ctx> {
    fn visit_program(&mut self, program: &'a Program) -> Result<(), Box<dyn IError>> {
        for statement in &program.statements {
            self.visit_statement(statement)?;
        }

        Ok(())
    }

    fn visit_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        self.compile_expression(expression)
    }

    fn visit_statement(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        self.compile_statement(statement)
    }

    fn visit_parameter(&mut self, _parameter: &'a Node<Parameter>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_argument(&mut self, _argument: &'a Node<Argument>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_type(&mut self, _node_type: &'a Node<Type>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_block(&mut self, block: &'a Node<Block>) -> Result<(), Box<dyn IError>> {
        for statement in &block.value.0 {
            self.visit_statement(statement)?;
        }

        Ok(())
    }

    fn visit_switch_expression(&mut self, _switch_expression: &'a Node<SwitchExpression>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_switch_case(&mut self, _switch_case: &'a Node<SwitchCase>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_literal(&mut self, literal: &'a Literal) -> Result<(), Box<dyn IError>> {
        match literal {
            Literal::I64(value) => {
                let const_value = self.i64_type().const_int(*value as u64, true);
                self.last_value = Some(LlvmValue::I64(const_value));
                Ok(())
            }
            Literal::F64(value) => {
                let const_value = self.f64_type().const_float(*value);
                self.last_value = Some(LlvmValue::F64(const_value));
                Ok(())
            }
            Literal::String(value) => {
                let string_value = self
                    .builder
                    .build_global_string_ptr(value.as_str(), "str")
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), self.span)) as Box<dyn IError>)?;

                self.last_value = Some(LlvmValue::Str(string_value.as_pointer_value()));

                Ok(())
            }

            Literal::True => {
                self.last_value = Some(LlvmValue::Bool(self.context.bool_type().const_int(1, false)));
                Ok(())
            }

            Literal::False => {
                self.last_value = Some(LlvmValue::Bool(self.context.bool_type().const_int(0, false)));
                Ok(())
            }
        }
    }

    fn visit_vector_literal(&mut self, _vector: &'a Vec<Box<Node<Expression>>>) -> Result<(), Box<dyn IError>> {
        Err(Box::new(CompilerError::at(
            ErrorSeverity::HIGH,
            String::from("Compiling vector literals is not yet supported."),
            self.span,
        )))
    }

    fn visit_variable(&mut self, variable: &'a String, span: Span) -> Result<(), Box<dyn IError>> {
        let (ptr, var_type) = self.get_variable(variable.as_str())?;
        let llvm_type = LlvmValue::type_to_basic_type_enum(&var_type, self.context).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Compiling variables of type '{:?}' is not yet supported.", var_type),
                span,
            )) as Box<dyn IError>
        })?;

        let raw_value = self
            .builder
            .build_load(llvm_type, ptr, variable.as_str())
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

        self.last_value = Some(LlvmValue::from_basic_value_enum(raw_value, &var_type));

        Ok(())
    }
}
