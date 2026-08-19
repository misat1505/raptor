use inkwell::AddressSpace;

use super::{Compiler, ControlFrame};
use crate::common::visitor::Visitor;
use crate::{
    backend::llvm::llvm_alu::llvm_value::LlvmValue,
    common::{
        errors::{CompilerError, ErrorSeverity, IError},
        types::Type,
    },
    frontend::ast::{Expression, Node, Statement},
};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(in crate::backend::llvm::compiler) fn compile_statement(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        let span = statement.span;

        match &statement.value {
            Statement::FunctionCall { identifier, arguments } => {
                let name = identifier.value.as_str();

                if let Some(std_function) = self.program.std_functions.get(name) {
                    return (std_function.compile)(self, arguments, span);
                }

                self.build_function_call(identifier, arguments, span)
            }

            Statement::Declaration { var_type, identifier, value } => {
                let llvm_type = LlvmValue::type_to_basic_type_enum(&var_type.value, self.context).ok_or_else(|| {
                    Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("Compiling declarations of type '{:?}' is not yet supported.", var_type.value),
                        span,
                    )) as Box<dyn IError>
                })?;

                let ptr = self
                    .builder
                    .build_alloca(llvm_type, identifier.value.as_str())
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

                match value {
                    Some(val_expr) => match (&var_type.value, &val_expr.value) {
                        (Type::Vector(inner), Expression::Vector(elements)) if elements.is_empty() => {
                            let vector_ptr = self.build_empty_vector(inner, span)?;

                            self.builder
                                .build_store(ptr, vector_ptr)
                                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
                        }

                        (Type::Vector(inner), Expression::Vector(elements)) => {
                            let vector_ptr = if elements.is_empty() {
                                self.build_empty_vector(inner, span)?
                            } else {
                                self.build_vector_from_elements(inner, elements, None, span)?
                            };

                            self.builder
                                .build_store(ptr, vector_ptr)
                                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
                        }

                        _ => {
                            self.visit_expression(val_expr)?;

                            let init_value = self.read_last_value()?;

                            self.builder
                                .build_store(ptr, init_value.as_basic_value_enum())
                                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
                        }
                    },

                    None => {
                        self.build_default_value(ptr, &var_type.value, span)?;
                    }
                }

                self.variables.insert(identifier.value.clone(), (ptr, var_type.value.clone()));

                Ok(())
            }

            Statement::Assignment { identifier, value, indices } => {
                let (var_ptr, var_type) = self.get_variable(identifier.value.as_str())?;

                if indices.is_empty() {
                    self.visit_expression(value)?;

                    let new_value = self.read_last_value()?;

                    self.builder
                        .build_store(var_ptr, new_value.as_basic_value_enum())
                        .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

                    return Ok(());
                }

                let ptr_type = self.context.ptr_type(AddressSpace::default());

                let vector_ptr = self
                    .builder
                    .build_load(ptr_type, var_ptr, "assign.vec")
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?
                    .into_pointer_value();

                let (element_ptr, element_type) = self.resolve_indexed_element(vector_ptr, &var_type, indices, span)?;

                self.visit_expression(value)?;

                let new_value = self.read_last_value()?;

                if new_value.to_type() != element_type {
                    return Err(Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!(
                            "Type mismatch in indexed assignment: expected '{:?}', got '{:?}'.",
                            element_type,
                            new_value.to_type()
                        ),
                        span,
                    )));
                }

                self.builder
                    .build_store(element_ptr, new_value.as_basic_value_enum())
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

                Ok(())
            }

            Statement::ForLoop {
                declaration,
                condition,
                assignment,
                block,
            } => {
                let function = self.current_function();

                if let Some(decl) = declaration {
                    self.visit_statement(decl)?;
                }

                let cond_block = self.context.append_basic_block(function, "for.cond");

                let body_block = self.context.append_basic_block(function, "for.body");

                let continue_block = self.context.append_basic_block(function, "for.continue");

                let after_block = self.context.append_basic_block(function, "for.after");

                self.builder
                    .build_unconditional_branch(cond_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

                self.builder.position_at_end(cond_block);

                self.visit_expression(condition)?;

                let cond_value = self.read_last_value()?.into_int_value(span)?;

                self.builder
                    .build_conditional_branch(cond_value, body_block, after_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

                self.builder.position_at_end(body_block);

                self.control_stack.push(ControlFrame::Loop {
                    continue_block,
                    break_block: after_block,
                });

                self.visit_block(block)?;

                self.control_stack.pop();

                self.branch_if_no_terminator(continue_block, span)?;

                self.builder.position_at_end(continue_block);

                if let Some(assign) = assignment {
                    self.visit_statement(assign)?;
                }

                self.branch_if_no_terminator(cond_block, span)?;

                self.builder.position_at_end(after_block);

                Ok(())
            }

            Statement::Conditional {
                condition,
                if_block,
                else_block,
            } => {
                let function = self.current_function();

                let cond_block = self.context.append_basic_block(function, "if.cond");

                let true_block = self.context.append_basic_block(function, "if.true");

                let false_block = match else_block {
                    Some(_) => Some(self.context.append_basic_block(function, "if.false")),
                    None => None,
                };

                let after_block = self.context.append_basic_block(function, "if.after");

                self.builder
                    .build_unconditional_branch(cond_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

                self.builder.position_at_end(cond_block);

                self.visit_expression(condition)?;

                let cond_value = self.read_last_value()?.into_int_value(span)?;

                self.builder
                    .build_conditional_branch(cond_value, true_block, false_block.unwrap_or(after_block))
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

                self.builder.position_at_end(true_block);

                self.visit_block(if_block)?;

                self.branch_if_no_terminator(after_block, span)?;

                if let Some(b) = false_block {
                    self.builder.position_at_end(b);

                    self.visit_block(else_block.as_ref().expect("else block should exist"))?;

                    self.branch_if_no_terminator(after_block, span)?;
                }

                self.builder.position_at_end(after_block);

                Ok(())
            }

            Statement::WhileLoop { condition, block } => {
                let function = self.current_function();

                let cond_block = self.context.append_basic_block(function, "while.cond");

                let body_block = self.context.append_basic_block(function, "while.block");

                let after_block = self.context.append_basic_block(function, "while.after");

                self.builder
                    .build_unconditional_branch(cond_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

                self.builder.position_at_end(cond_block);

                self.visit_expression(condition)?;

                let cond_value = self.read_last_value()?.into_int_value(span)?;

                self.builder
                    .build_conditional_branch(cond_value, body_block, after_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

                self.builder.position_at_end(body_block);

                self.control_stack.push(ControlFrame::Loop {
                    continue_block: cond_block,
                    break_block: after_block,
                });

                self.visit_block(block)?;

                self.control_stack.pop();

                self.branch_if_no_terminator(cond_block, span)?;

                self.builder.position_at_end(after_block);

                Ok(())
            }

            Statement::Return(value) => {
                match value {
                    Some(expr) => {
                        self.visit_expression(expr)?;

                        let return_value = self.read_last_value()?;

                        self.builder
                            .build_return(Some(&return_value.as_basic_value_enum()))
                            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
                    }

                    None => {
                        self.builder
                            .build_return(None)
                            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
                    }
                }

                Ok(())
            }

            Statement::Break => {
                let target = self.find_break_target(span)?;

                self.branch_if_no_terminator(target, span)?;

                Ok(())
            }

            Statement::Continue => {
                let target = self.find_continue_target(span)?;

                self.branch_if_no_terminator(target, span)?;

                Ok(())
            }

            Statement::Switch { expressions, cases } => self.compile_switch(expressions, cases),
        }
    }
}
