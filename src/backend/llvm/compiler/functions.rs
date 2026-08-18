use inkwell::types::{BasicMetadataTypeEnum, BasicType};
use inkwell::values::{BasicMetadataValueEnum, FunctionValue, PointerValue};
use inkwell::AddressSpace;

use super::Compiler;
use crate::common::visitor::Visitor;
use crate::{
    backend::llvm::llvm_alu::llvm_value::LlvmValue,
    common::{
        errors::{CompilerError, ErrorSeverity, IError},
        span::Span,
        types::Type,
    },
    frontend::ast::{Argument, Expression, FunctionDeclaration, Node, PassedBy},
};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(in crate::backend::llvm::compiler) fn declare_functions(&mut self) -> Result<(), Box<dyn IError>> {
        for (name, declaration) in &self.program.functions {
            let function_decl = &declaration.value;

            let mut param_types: Vec<BasicMetadataTypeEnum> = Vec::with_capacity(function_decl.parameters.len());

            for parameter in &function_decl.parameters {
                let param_type: BasicMetadataTypeEnum = match parameter.value.passed_by {
                    PassedBy::Reference => self.context.ptr_type(AddressSpace::default()).into(),

                    PassedBy::Value => {
                        let llvm_type = LlvmValue::type_to_basic_type_enum(&parameter.value.parameter_type.value, self.context).ok_or_else(|| {
                            Box::new(CompilerError::at(
                                ErrorSeverity::HIGH,
                                format!(
                                    "Compiling parameters of type '{:?}' is not yet supported.",
                                    parameter.value.parameter_type.value
                                ),
                                parameter.span,
                            )) as Box<dyn IError>
                        })?;

                        llvm_type.into()
                    }
                };

                param_types.push(param_type);
            }

            let fn_type = match &function_decl.return_type.value {
                Type::Void => self.context.void_type().fn_type(&param_types, false),

                return_type => {
                    let llvm_return_type = LlvmValue::type_to_basic_type_enum(return_type, self.context).ok_or_else(|| {
                        Box::new(CompilerError::at(
                            ErrorSeverity::HIGH,
                            format!("Compiling functions returning '{:?}' is not yet supported.", return_type),
                            function_decl.return_type.span,
                        )) as Box<dyn IError>
                    })?;

                    llvm_return_type.fn_type(&param_types, false)
                }
            };

            let function = self.module.add_function(name, fn_type, None);

            self.functions.insert(name.clone(), function);
        }

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn declare_extern_functions(&mut self) -> Result<(), Box<dyn IError>> {
        for (name, declaration) in &self.program.extern_functions {
            let function_decl = &declaration.value;

            let mut param_types: Vec<BasicMetadataTypeEnum> = Vec::with_capacity(function_decl.parameters.len());

            for parameter in &function_decl.parameters {
                let param_type: BasicMetadataTypeEnum = match parameter.value.passed_by {
                    PassedBy::Reference => self.context.ptr_type(AddressSpace::default()).into(),

                    PassedBy::Value => {
                        let llvm_type = LlvmValue::type_to_basic_type_enum(&parameter.value.parameter_type.value, self.context).ok_or_else(|| {
                            Box::new(CompilerError::at(
                                ErrorSeverity::HIGH,
                                format!(
                                    "Compiling extern parameters of type '{:?}' is not yet supported.",
                                    parameter.value.parameter_type.value
                                ),
                                parameter.span,
                            )) as Box<dyn IError>
                        })?;

                        llvm_type.into()
                    }
                };

                param_types.push(param_type);
            }

            let fn_type = match &function_decl.return_type.value {
                Type::Void => self.context.void_type().fn_type(&param_types, false),

                return_type => {
                    let llvm_return_type = LlvmValue::type_to_basic_type_enum(return_type, self.context).ok_or_else(|| {
                        Box::new(CompilerError::at(
                            ErrorSeverity::HIGH,
                            format!("Compiling extern functions returning '{:?}' is not yet supported.", return_type),
                            function_decl.return_type.span,
                        )) as Box<dyn IError>
                    })?;

                    llvm_return_type.fn_type(&param_types, false)
                }
            };

            // Symbol LLVM musi być prawdziwą nazwą funkcji w bibliotece C
            // (np. "InitWindow"), niezależnie od aliasu używanego w źródle.
            let symbol_name = function_decl.identifier.value.as_str();

            let function = self.module.add_function(symbol_name, fn_type, None);

            // Klucz pozostaje aliasem używanym przez build_function_call.
            self.functions.insert(name.clone(), function);
        }

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn compile_functions(&mut self) -> Result<(), Box<dyn IError>> {
        for (name, declaration) in &self.program.functions {
            let function = *self
                .functions
                .get(name)
                .expect("function should have been predeclared by declare_functions");

            self.compile_function_body(function, &declaration.value)?;
        }

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn compile_function_body(
        &mut self,
        function: FunctionValue<'ctx>,
        function_decl: &'a FunctionDeclaration,
    ) -> Result<(), Box<dyn IError>> {
        let entry_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(entry_block);

        let saved_variables = std::mem::take(&mut self.variables);

        for (index, parameter) in function_decl.parameters.iter().enumerate() {
            let identifier = parameter.value.identifier.value.as_str();

            let param_type = &parameter.value.parameter_type.value;

            let param_value = function
                .get_nth_param(index as u32)
                .expect("parameter index should be valid, matches signature built in declare_functions");

            match parameter.value.passed_by {
                PassedBy::Value => {
                    let llvm_type = LlvmValue::type_to_basic_type_enum(param_type, self.context).ok_or_else(|| {
                        Box::new(CompilerError::at(
                            ErrorSeverity::HIGH,
                            format!("Compiling parameters of type '{:?}' is not yet supported.", param_type),
                            parameter.span,
                        )) as Box<dyn IError>
                    })?;

                    let ptr = self
                        .builder
                        .build_alloca(llvm_type, identifier)
                        .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), parameter.span)) as Box<dyn IError>)?;

                    self.builder
                        .build_store(ptr, param_value)
                        .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), parameter.span)) as Box<dyn IError>)?;

                    self.variables.insert(identifier.to_string(), (ptr, param_type.clone()));
                }

                PassedBy::Reference => {
                    let ptr = param_value.into_pointer_value();

                    self.variables.insert(identifier.to_string(), (ptr, param_type.clone()));
                }
            }
        }

        self.visit_block(&function_decl.block)?;

        let current_block = self.builder.get_insert_block().expect("builder should be positioned inside the function");

        if current_block.get_terminator().is_none() {
            match &function_decl.return_type.value {
                Type::Void => {
                    self.builder.build_return(None).map_err(|err| {
                        Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), function_decl.return_type.span)) as Box<dyn IError>
                    })?;
                }

                _ => {
                    self.builder.build_unreachable().map_err(|err| {
                        Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), function_decl.return_type.span)) as Box<dyn IError>
                    })?;
                }
            }
        }

        self.variables = saved_variables;

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn build_function_call(
        &mut self,
        identifier: &'a Node<String>,
        arguments: &'a Vec<Box<Node<Argument>>>,
        span: Span,
    ) -> Result<(), Box<dyn IError>> {
        let name = identifier.value.as_str();

        let function = *self.functions.get(name).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Compiling calls to '{}' is not yet supported.", name),
                span,
            )) as Box<dyn IError>
        })?;

        let mut compiled_args: Vec<BasicMetadataValueEnum> = Vec::with_capacity(arguments.len());

        for argument in arguments {
            match argument.value.passed_by {
                PassedBy::Value => {
                    self.visit_expression(&argument.value.value)?;

                    let value = self.read_last_value()?;

                    let value = match value {
                        LlvmValue::Vector(ptr, inner) => {
                            let copy_ptr = self.build_shallow_copy_vector(ptr, &inner, span)?;

                            LlvmValue::Vector(copy_ptr, inner)
                        }

                        other => other,
                    };

                    compiled_args.push(value.as_basic_value_enum().into());
                }

                PassedBy::Reference => {
                    let ptr = self.resolve_reference(&argument.value.value)?;

                    compiled_args.push(ptr.into());
                }
            }
        }

        let call_site = self
            .builder
            .build_call(function, &compiled_args, "call")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

        let return_type = if let Some(function) = self.program.functions.get(name) {
            &function.value.return_type.value
        } else if let Some(function) = self.program.extern_functions.get(name) {
            &function.value.return_type.value
        } else {
            return Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Unknown function '{}'.", name),
                span,
            )));
        };

        self.last_value = match call_site.try_as_basic_value().basic() {
            Some(return_value) => Some(LlvmValue::from_basic_value_enum(return_value, return_type)),
            None => None,
        };

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn resolve_reference(
        &mut self,
        expression: &'a Node<Expression>,
    ) -> Result<PointerValue<'ctx>, Box<dyn IError>> {
        match &expression.value {
            Expression::Variable(name) => {
                let (ptr, _) = self.get_variable(name.as_str())?;

                Ok(ptr)
            }

            other => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Cannot pass expression '{:?}' by reference.", other),
                expression.span,
            ))),
        }
    }
}
