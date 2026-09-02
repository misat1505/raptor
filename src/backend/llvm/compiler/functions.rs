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
                        let resolved_type = self.resolve_type(&parameter.value.parameter_type.value);

                        let llvm_type = LlvmValue::type_to_basic_type_enum(&resolved_type, self.context).ok_or_else(|| {
                            Box::new(CompilerError::at(
                                ErrorSeverity::HIGH,
                                format!("Compiling parameters of type '{:?}' is not yet supported.", resolved_type),
                                parameter.span,
                            )) as Box<dyn IError>
                        })?;

                        llvm_type.into()
                    }
                };

                param_types.push(param_type);
            }

            let resolved_return_type = self.resolve_type(&function_decl.return_type.value);

            let fn_type = match &resolved_return_type {
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
                        let resolved_type = self.resolve_type(&parameter.value.parameter_type.value);

                        let llvm_type = LlvmValue::type_to_basic_type_enum(&resolved_type, self.context).ok_or_else(|| {
                            Box::new(CompilerError::at(
                                ErrorSeverity::HIGH,
                                format!("Compiling extern parameters of type '{:?}' is not yet supported.", resolved_type),
                                parameter.span,
                            )) as Box<dyn IError>
                        })?;

                        llvm_type.into()
                    }
                };

                param_types.push(param_type);
            }

            let resolved_return_type = self.resolve_type(&function_decl.return_type.value);

            let fn_type = match &resolved_return_type {
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

            let symbol_name = function_decl.identifier.value.as_str();

            let function = self.module.add_function(symbol_name, fn_type, None);

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
        let saved_scopes = std::mem::take(&mut self.scopes);

        // Parameters live in their own scope, opened before the function
        // body's own block scope and closed (released) after it, on the
        // fall-through path. `return` releases every active scope
        // (parameters included) before jumping away, so this scope's own
        // release below is skipped whenever the body always returns.
        self.push_scope();

        for (index, parameter) in function_decl.parameters.iter().enumerate() {
            let identifier = parameter.value.identifier.value.as_str();

            let param_type = self.resolve_type(&parameter.value.parameter_type.value);

            let param_value = function
                .get_nth_param(index as u32)
                .expect("parameter index should be valid, matches signature built in declare_functions");

            match parameter.value.passed_by {
                PassedBy::Value => {
                    let llvm_type = LlvmValue::type_to_basic_type_enum(&param_type, self.context).ok_or_else(|| {
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

                    // The caller already handed us an owned +1 reference
                    // (see `build_function_call`), so the parameter is
                    // tracked like any other owned local and released when
                    // the function returns / falls off the end.
                    self.declare_scoped_variable(identifier.to_string(), ptr, param_type);
                }

                PassedBy::Reference => {
                    let ptr = param_value.into_pointer_value();

                    // Reference parameters alias the caller's storage - not
                    // owned here, so they're just a lookup entry, never
                    // released.
                    self.variables.insert(identifier.to_string(), (ptr, param_type));
                }
            }
        }

        self.visit_block(&function_decl.block)?;

        self.pop_scope_and_release(function_decl.return_type.span)?;

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
        self.scopes = saved_scopes;

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

                    // The callee takes ownership of this argument (it will
                    // release it like any other local when it returns).
                    // Strings are always deep-copied. Vector/Struct values
                    // only need an explicit retain if the argument
                    // expression was a bare variable read (a "borrow") -
                    // anything else already evaluates to an owned +1
                    // reference and can be handed over as-is.
                    let value = match value {
                        LlvmValue::Str(ptr) => {
                            let copy_ptr = self.build_string_copy(ptr, span)?;
                            if Self::expr_needs_release_in_function_call(&argument.value.value.value) {
                                self.release_value(&value, span)?;
                            }
                            LlvmValue::Str(copy_ptr)
                        }

                        LlvmValue::Vector(_, _) | LlvmValue::Struct(_, _) => {
                            if Self::expr_needs_retain(&argument.value.value.value) {
                                self.retain_value(&value, span)?;
                            }
                            value
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
            &self.resolve_type(&function.value.return_type.value)
        } else if let Some(function) = self.program.extern_functions.get(name) {
            &self.resolve_type(&function.value.return_type.value)
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

    pub(in crate::backend) fn resolve_reference(&mut self, expression: &'a Node<Expression>) -> Result<PointerValue<'ctx>, Box<dyn IError>> {
        match &expression.value {
            Expression::Variable(name) => {
                let (ptr, _) = self.get_variable(name.as_str())?;

                Ok(ptr)
            }

            Expression::FieldAccess { instance, field } => {
                self.visit_expression(instance)?;
                let instance_value = self.read_last_value()?;

                let (struct_ptr, struct_type_info) = match &instance_value {
                    LlvmValue::Struct(ptr, ty) => (*ptr, ty.clone()),

                    other => {
                        return Err(Box::new(CompilerError::at(
                            ErrorSeverity::HIGH,
                            format!("Cannot access field '{}' on type '{:?}'.", field.value, other.to_type()),
                            expression.span,
                        )));
                    }
                };

                let Type::Struct { identifier, .. } = struct_type_info.as_ref() else {
                    return Err(Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("Cannot access field '{}' on a non-struct type.", field.value),
                        expression.span,
                    )));
                };

                let (struct_type, field_indices) = self.struct_llvm_type(identifier, expression.span)?;

                let field_index = *field_indices.get(&field.value).ok_or_else(|| {
                    Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("Struct '{}' has no field '{}'.", identifier, field.value),
                        field.span,
                    )) as Box<dyn IError>
                })?;

                let field_ptr = self
                    .builder
                    .build_struct_gep(struct_type, struct_ptr, field_index, "field.ref")
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), field.span)) as Box<dyn IError>)?;

                Ok(field_ptr)
            }

            Expression::Index { collection, index } => {
                self.visit_expression(collection)?;
                let collection_value = self.read_last_value()?;

                match &collection_value {
                    LlvmValue::Vector(vector_ptr, inner) => {
                        let inner = self.resolve_type(inner);

                        let struct_type = LlvmValue::vector_struct_type(self.context);
                        let ptr_type = self.context.ptr_type(AddressSpace::default());

                        let data_field = self
                            .builder
                            .build_struct_gep(
                                struct_type,
                                *vector_ptr,
                                crate::backend::llvm::llvm_alu::llvm_value::VEC_DATA,
                                "idx.ref.data",
                            )
                            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), expression.span)) as Box<dyn IError>)?;

                        let data = self
                            .builder
                            .build_load(ptr_type, data_field, "idx.ref.data.val")
                            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), expression.span)) as Box<dyn IError>)?
                            .into_pointer_value();

                        self.visit_expression(index)?;
                        let index_value = self.read_last_value()?;
                        let index_int = index_value.into_i64_value(index.span)?;

                        let element_llvm_type = LlvmValue::type_to_basic_type_enum(&inner, self.context).ok_or_else(|| {
                            Box::new(CompilerError::at(
                                ErrorSeverity::HIGH,
                                format!("Compiling vectors of type '{:?}' is not yet supported.", inner),
                                index.span,
                            )) as Box<dyn IError>
                        })?;

                        let element_ptr = unsafe {
                            self.builder
                                .build_gep(element_llvm_type, data, &[index_int], "idx.ref.elem")
                                .map_err(|err| {
                                    Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), expression.span)) as Box<dyn IError>
                                })?
                        };

                        Ok(element_ptr)
                    }

                    LlvmValue::Str(str_ptr) => {
                        self.visit_expression(index)?;
                        let index_value = self.read_last_value()?;
                        let index_int = index_value.into_i64_value(index.span)?;

                        let data = self.str_data_ptr(*str_ptr, expression.span)?;

                        let i8_type = self.context.i8_type();

                        let element_ptr = unsafe {
                            self.builder.build_gep(i8_type, data, &[index_int], "str.ref.ptr").map_err(|err| {
                                Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), expression.span)) as Box<dyn IError>
                            })?
                        };

                        Ok(element_ptr)
                    }

                    other => Err(Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("Cannot index into type '{:?}'.", other.to_type()),
                        expression.span,
                    ))),
                }
            }

            other => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Cannot pass expression '{:?}' by reference.", other),
                expression.span,
            ))),
        }
    }

    /// Deep-copies a string: allocates a brand new `StrHeader` with its own
    /// heap-allocated character buffer, refcount 1, independent from `ptr`.
    /// This is what gives strings value semantics even though they're
    /// heap-allocated and (internally) refcounted.
    pub(in crate::backend) fn build_string_copy(&mut self, ptr: PointerValue<'ctx>, span: Span) -> Result<PointerValue<'ctx>, Box<dyn IError>> {
        let err = Self::builder_err(span);

        let i64_type = self.context.i64_type();

        let source_data = self.str_data_ptr(ptr, span)?;

        // strlen(source_data)
        let strlen = self
            .builder()
            .build_call(self.libc().strlen_fn, &[source_data.into()], "str.copy.len")
            .map_err(&err)?
            .try_as_basic_value()
            .basic()
            .expect("strlen should return a value")
            .into_int_value();

        // strlen + 1 for '\0'
        let size = self
            .builder()
            .build_int_add(strlen, i64_type.const_int(1, false), "str.copy.size")
            .map_err(&err)?;

        // malloc(size)
        let new_data = self
            .builder()
            .build_call(self.libc().malloc_fn, &[size.into()], "str.copy.malloc")
            .map_err(&err)?
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();

        // strcpy(new_data, source_data)
        self.builder()
            .build_call(self.libc().strcpy_fn, &[new_data.into(), source_data.into()], "str.copy")
            .map_err(&err)?;

        self.build_str_header(new_data, span)
    }

    #[allow(dead_code)]
    pub(in crate::backend::llvm::compiler) fn build_shallow_copy_struct(
        &mut self,
        ptr: PointerValue<'ctx>,
        ty: &Type,
        span: Span,
    ) -> Result<PointerValue<'ctx>, Box<dyn IError>> {
        let Type::Struct { identifier, .. } = ty else {
            return Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Cannot build a shallow copy of non-struct type '{:?}'.", ty),
                span,
            )));
        };

        let (struct_type, field_indices) = self.struct_llvm_type(identifier, span)?;

        let size = struct_type.size_of().expect("struct type should be sized");
        let new_ptr = self
            .builder
            .build_call(self.libc.malloc_fn, &[size.into()], "struct.malloc")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();

        let mut sorted_fields: Vec<(&String, &u32)> = field_indices.iter().collect();
        sorted_fields.sort_by_key(|(_, idx)| **idx);

        for (field_name, field_index) in sorted_fields {
            let field_llvm_type = struct_type
                .get_field_type_at_index(*field_index)
                .expect("field index should be valid, built from the same struct_type");

            let src_field_ptr = self
                .builder
                .build_struct_gep(struct_type, ptr, *field_index, format!("{}.src", field_name).as_str())
                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

            let field_value = self
                .builder
                .build_load(field_llvm_type, src_field_ptr, format!("{}.load", field_name).as_str())
                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

            let dst_field_ptr = self
                .builder
                .build_struct_gep(struct_type, new_ptr, *field_index, format!("{}.dst", field_name).as_str())
                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

            self.builder
                .build_store(dst_field_ptr, field_value)
                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
        }

        let refcount_index = Self::struct_refcount_field_index(struct_type);
        let refcount_field = self
            .builder
            .build_struct_gep(struct_type, new_ptr, refcount_index, "struct.copy.refcount")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
        self.builder
            .build_store(refcount_field, self.context.i64_type().const_int(1, false))
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

        Ok(new_ptr)
    }
}
