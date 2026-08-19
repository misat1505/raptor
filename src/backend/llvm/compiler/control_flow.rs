use inkwell::basic_block::BasicBlock;

use super::{Compiler, ControlFrame};
use crate::common::visitor::Visitor;
use crate::{
    backend::llvm::llvm_alu::llvm_value::LlvmValue,
    common::{
        errors::{CompilerError, ErrorSeverity, IError},
        span::Span,
    },
    frontend::ast::{Node, SwitchCase, SwitchExpression},
};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(in crate::backend::llvm::compiler) fn branch_if_no_terminator(
        &mut self,
        target: BasicBlock<'ctx>,
        span: Span,
    ) -> Result<(), Box<dyn IError>> {
        let current_block = self.builder.get_insert_block().expect("builder should be positioned inside a block");

        if current_block.get_terminator().is_none() {
            self.builder
                .build_unconditional_branch(target)
                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
        }

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn find_break_target(&self, span: Span) -> Result<BasicBlock<'ctx>, Box<dyn IError>> {
        self.control_stack
            .iter()
            .rev()
            .find_map(|frame| match frame {
                ControlFrame::Loop { break_block, .. } => Some(*break_block),
                ControlFrame::Switch { break_block } => Some(*break_block),
            })
            .ok_or_else(|| {
                Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    String::from("'break' used outside of a loop or switch."),
                    span,
                )) as Box<dyn IError>
            })
    }

    pub(in crate::backend::llvm::compiler) fn find_continue_target(&self, span: Span) -> Result<BasicBlock<'ctx>, Box<dyn IError>> {
        self.control_stack
            .iter()
            .rev()
            .find_map(|frame| match frame {
                ControlFrame::Loop { continue_block, .. } => Some(*continue_block),
                ControlFrame::Switch { .. } => None,
            })
            .ok_or_else(|| {
                Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    String::from("'continue' used outside of a loop."),
                    span,
                )) as Box<dyn IError>
            })
    }

    pub(in crate::backend::llvm::compiler) fn compile_switch(
        &mut self,
        expressions: &'a Vec<Node<SwitchExpression>>,
        cases: &'a Vec<Node<SwitchCase>>,
    ) -> Result<(), Box<dyn IError>> {
        let function = self.current_function();

        let saved_variables = self.variables.clone();

        for switch_expr in expressions {
            if let Some(alias) = &switch_expr.value.alias {
                self.visit_expression(&switch_expr.value.expression)?;
                let value = self.read_last_value()?;

                let var_type = value.to_type();

                let llvm_type = LlvmValue::type_to_basic_type_enum(&var_type, self.context).ok_or_else(|| {
                    Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("Compiling switch bindings of type '{:?}' is not yet supported.", var_type),
                        switch_expr.span,
                    )) as Box<dyn IError>
                })?;

                let ptr = self
                    .builder
                    .build_alloca(llvm_type, alias.value.as_str())
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), switch_expr.span)) as Box<dyn IError>)?;

                self.builder
                    .build_store(ptr, value.as_basic_value_enum())
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), switch_expr.span)) as Box<dyn IError>)?;

                self.variables.insert(alias.value.clone(), (ptr, var_type));
            }
        }

        let after_block = self.context.append_basic_block(function, "switch.after");

        self.control_stack.push(ControlFrame::Switch { break_block: after_block });

        for (index, case) in cases.iter().enumerate() {
            let case_block = self.context.append_basic_block(function, &format!("switch.case{}", index));

            let next_check_block = if index + 1 < cases.len() {
                self.context.append_basic_block(function, &format!("switch.check{}", index + 1))
            } else {
                after_block
            };

            self.visit_expression(&case.value.condition)?;

            let cond_value = self.read_last_value()?.into_int_value(case.span)?;

            self.builder
                .build_conditional_branch(cond_value, case_block, next_check_block)
                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), case.span)) as Box<dyn IError>)?;

            self.builder.position_at_end(case_block);

            self.visit_block(&case.value.block)?;

            self.branch_if_no_terminator(next_check_block, case.span)?;

            self.builder.position_at_end(next_check_block);
        }

        self.control_stack.pop();

        self.variables = saved_variables;

        self.builder.position_at_end(after_block);

        Ok(())
    }
}
