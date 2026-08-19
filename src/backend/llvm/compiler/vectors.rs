use inkwell::values::PointerValue;
use inkwell::{AddressSpace, IntPredicate};

use super::Compiler;
use crate::common::visitor::Visitor;
use crate::{
    backend::llvm::llvm_alu::llvm_value::LlvmValue,
    common::{
        errors::{CompilerError, ErrorSeverity, IError},
        span::Span,
        types::Type,
    },
    frontend::ast::{Expression, Node},
};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(in crate::backend::llvm::compiler) fn build_empty_vector(
        &mut self,
        inner_type: &Type,
        span: Span,
    ) -> Result<PointerValue<'ctx>, Box<dyn IError>> {
        let struct_type = LlvmValue::vector_struct_type(self.context);

        let struct_size = self.context.i64_type().const_int(24, false);
        let struct_ptr_raw = self
            .builder
            .build_call(self.libc.malloc_fn, &[struct_size.into()], "vector.header.malloc")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();
        let struct_ptr = struct_ptr_raw;

        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        // data = null
        let data_field = self
            .builder
            .build_struct_gep(struct_type, struct_ptr, 0, "vector.data")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
        self.builder
            .build_store(data_field, ptr_type.const_null())
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

        // length = 0
        let length_field = self
            .builder
            .build_struct_gep(struct_type, struct_ptr, 1, "vector.length")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
        self.builder
            .build_store(length_field, i64_type.const_int(0, false))
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

        // capacity = 0
        let capacity_field = self
            .builder
            .build_struct_gep(struct_type, struct_ptr, 2, "vector.capacity")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
        self.builder
            .build_store(capacity_field, i64_type.const_int(0, false))
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

        let _ = inner_type;

        Ok(struct_ptr)
    }

    pub(in crate::backend::llvm::compiler) fn build_vector_from_elements(
        &mut self,
        inner_type: &Type,
        elements: &'a Vec<Box<Node<Expression>>>,
        precomputed_first: Option<LlvmValue<'ctx>>,
        span: Span,
    ) -> Result<PointerValue<'ctx>, Box<dyn IError>> {
        let element_llvm_type = LlvmValue::type_to_basic_type_enum(inner_type, self.context).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Compiling vectors of type '{:?}' is not yet supported.", inner_type),
                span,
            )) as Box<dyn IError>
        })?;

        let count = elements.len() as u64;
        let i64_type = self.context.i64_type();

        let element_size = LlvmValue::element_byte_size(inner_type, i64_type, span)?;

        let total_size = self
            .builder
            .build_int_mul(element_size, i64_type.const_int(count, false), "vector.bytes")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

        let data_ptr = self
            .builder
            .build_call(self.libc.malloc_fn, &[total_size.into()], "vector.malloc")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();

        for (index, element) in elements.iter().enumerate() {
            // element 0 mógł już zostać policzony wcześniej (np. do wywnioskowania inner_type) —
            // wtedy nie liczymy go drugi raz, żeby nie zdublować efektów ubocznych (np. wywołań funkcji)
            let element_value = if index == 0 {
                if let Some(value) = precomputed_first.clone() {
                    value
                } else {
                    self.evaluate_vector_element(inner_type, element)?
                }
            } else {
                self.evaluate_vector_element(inner_type, element)?
            };

            if element_value.to_type() != *inner_type {
                return Err(Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    format!(
                        "Vector element type mismatch: expected '{:?}', got '{:?}'.",
                        inner_type,
                        element_value.to_type()
                    ),
                    element.span,
                )));
            }

            let element_ptr = unsafe {
                self.builder
                    .build_gep(element_llvm_type, data_ptr, &[i64_type.const_int(index as u64, false)], "vector.elem")
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), element.span)) as Box<dyn IError>)?
            };

            self.builder
                .build_store(element_ptr, element_value.as_basic_value_enum())
                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), element.span)) as Box<dyn IError>)?;
        }

        let struct_type = LlvmValue::vector_struct_type(self.context);
        let struct_size = self.context.i64_type().const_int(24, false);
        let struct_ptr = self
            .builder
            .build_call(self.libc.malloc_fn, &[struct_size.into()], "vector.header.malloc")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();

        let data_field = self
            .builder
            .build_struct_gep(struct_type, struct_ptr, 0, "vector.data.field")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
        self.builder
            .build_store(data_field, data_ptr)
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

        let length_field = self
            .builder
            .build_struct_gep(struct_type, struct_ptr, 1, "vector.length.field")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
        self.builder
            .build_store(length_field, i64_type.const_int(count, false))
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

        let capacity_field = self
            .builder
            .build_struct_gep(struct_type, struct_ptr, 2, "vector.capacity.field")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;
        self.builder
            .build_store(capacity_field, i64_type.const_int(count, false))
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>)?;

        Ok(struct_ptr)
    }

    pub(in crate::backend::llvm::compiler) fn evaluate_vector_element(
        &mut self,
        inner_type: &Type,
        element: &'a Node<Expression>,
    ) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        match (inner_type, &element.value) {
            (Type::Vector(nested_inner), Expression::Vector(nested_elements)) => {
                let nested_ptr = if nested_elements.is_empty() {
                    self.build_empty_vector(nested_inner, element.span)?
                } else {
                    self.build_vector_from_elements(nested_inner, nested_elements, None, element.span)?
                };
                Ok(LlvmValue::Vector(nested_ptr, nested_inner.clone()))
            }
            _ => {
                self.visit_expression(element)?;
                self.read_last_value()
            }
        }
    }

    pub(in crate::backend::llvm::compiler) fn build_vector_expression(
        &mut self,
        elements: &'a Vec<Box<Node<Expression>>>,
        span: Span,
    ) -> Result<(PointerValue<'ctx>, Type), Box<dyn IError>> {
        if elements.is_empty() {
            return Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("Cannot infer the element type of an empty vector literal in this context; declare it with an explicit type instead."),
                span,
            )) as Box<dyn IError>);
        }

        self.visit_expression(&elements[0])?;
        let first_value = self.read_last_value()?;
        let inner_type = first_value.to_type();

        let vector_ptr = self.build_vector_from_elements(&inner_type, elements, Some(first_value), span)?;

        Ok((vector_ptr, inner_type))
    }

    pub(in crate::backend::llvm::compiler) fn build_shallow_copy_vector(
        &mut self,
        vector_ptr: PointerValue<'ctx>,
        inner_type: &Type,
        span: Span,
    ) -> Result<PointerValue<'ctx>, Box<dyn IError>> {
        let err = Self::builder_err(span);
        let function = self.current_function();
        let struct_type = LlvmValue::vector_struct_type(self.context);
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        let data_field = self.builder.build_struct_gep(struct_type, vector_ptr, 0, "copy.src.data").map_err(&err)?;
        let length_field = self
            .builder
            .build_struct_gep(struct_type, vector_ptr, 1, "copy.src.length")
            .map_err(&err)?;

        let old_data = self
            .builder
            .build_load(ptr_type, data_field, "copy.data.old")
            .map_err(&err)?
            .into_pointer_value();
        let old_length = self
            .builder
            .build_load(i64_type, length_field, "copy.length.old")
            .map_err(&err)?
            .into_int_value();

        let element_size = LlvmValue::element_byte_size(inner_type, i64_type, span)?;
        let bytes = self.builder.build_int_mul(old_length, element_size, "copy.bytes").map_err(&err)?;

        // dla pustego wektora (data == null) unikamy malloc(0)/memcpy z null jako src — to UB
        let new_data_alloca = self.builder.build_alloca(ptr_type, "copy.data.slot").map_err(&err)?;
        self.builder.build_store(new_data_alloca, ptr_type.const_null()).map_err(&err)?;

        let is_empty = self
            .builder
            .build_int_compare(IntPredicate::EQ, old_length, i64_type.const_int(0, false), "copy.is_empty")
            .map_err(&err)?;

        let copy_block = self.context.append_basic_block(function, "copy.data");
        let merge_block = self.context.append_basic_block(function, "copy.merge");

        self.builder.build_conditional_branch(is_empty, merge_block, copy_block).map_err(&err)?;

        self.builder.position_at_end(copy_block);
        let new_data = self
            .builder
            .build_call(self.libc.malloc_fn, &[bytes.into()], "copy.data.malloc")
            .map_err(&err)?
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();

        self.builder
            .build_call(self.libc.memcpy_fn, &[new_data.into(), old_data.into(), bytes.into()], "copy.memcpy")
            .map_err(&err)?;

        self.builder.build_store(new_data_alloca, new_data).map_err(&err)?;

        self.builder.build_unconditional_branch(merge_block).map_err(&err)?;

        self.builder.position_at_end(merge_block);
        let final_data = self
            .builder
            .build_load(ptr_type, new_data_alloca, "copy.data.final")
            .map_err(&err)?
            .into_pointer_value();

        let struct_size = i64_type.const_int(24, false);
        let new_struct_ptr = self
            .builder
            .build_call(self.libc.malloc_fn, &[struct_size.into()], "copy.header.malloc")
            .map_err(&err)?
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();

        let new_data_field = self
            .builder
            .build_struct_gep(struct_type, new_struct_ptr, 0, "copy.dst.data")
            .map_err(&err)?;
        self.builder.build_store(new_data_field, final_data).map_err(&err)?;

        let new_length_field = self
            .builder
            .build_struct_gep(struct_type, new_struct_ptr, 1, "copy.dst.length")
            .map_err(&err)?;
        self.builder.build_store(new_length_field, old_length).map_err(&err)?;

        let new_capacity_field = self
            .builder
            .build_struct_gep(struct_type, new_struct_ptr, 2, "copy.dst.capacity")
            .map_err(&err)?;
        self.builder.build_store(new_capacity_field, old_length).map_err(&err)?; // po kopii capacity == length

        Ok(new_struct_ptr)
    }

    pub(in crate::backend::llvm::compiler) fn resolve_indexed_element(
        &mut self,
        mut current_ptr: PointerValue<'ctx>,
        current_type: &Type,
        indices: &'a [Node<Expression>],
        span: Span,
    ) -> Result<(PointerValue<'ctx>, Type), Box<dyn IError>> {
        if indices.is_empty() {
            return Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("Indexing requires at least one index."),
                span,
            )) as Box<dyn IError>);
        }

        let err = Self::builder_err(span);
        let struct_type = LlvmValue::vector_struct_type(self.context);
        let ptr_type = self.context.ptr_type(AddressSpace::default());

        let mut current_element_type = current_type.clone();

        for (i, index_expr) in indices.iter().enumerate() {
            let inner_type = match &current_element_type {
                Type::Vector(inner) => (**inner).clone(),
                other => {
                    return Err(Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("Cannot index into type '{:?}'.", other),
                        index_expr.span,
                    )) as Box<dyn IError>)
                }
            };

            let data_field = self.builder.build_struct_gep(struct_type, current_ptr, 0, "idx.data").map_err(&err)?;

            let data = self
                .builder
                .build_load(ptr_type, data_field, "idx.data.val")
                .map_err(&err)?
                .into_pointer_value();

            self.visit_expression(index_expr)?;
            let index_value = self.read_last_value()?.into_i64_value(index_expr.span)?;

            let element_llvm_type = LlvmValue::type_to_basic_type_enum(&inner_type, self.context).ok_or_else(|| {
                Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    format!("Compiling vectors of type '{:?}' is not yet supported.", inner_type),
                    index_expr.span,
                )) as Box<dyn IError>
            })?;

            let element_ptr = unsafe {
                self.builder
                    .build_gep(element_llvm_type, data, &[index_value], "idx.elem")
                    .map_err(&err)?
            };

            if i == indices.len() - 1 {
                return Ok((element_ptr, inner_type));
            }

            current_ptr = self
                .builder
                .build_load(ptr_type, element_ptr, "idx.next")
                .map_err(&err)?
                .into_pointer_value();

            current_element_type = inner_type;
        }

        Err(Box::new(CompilerError::at(
            ErrorSeverity::HIGH,
            String::from("Indexing requires at least one index."),
            span,
        )) as Box<dyn IError>)
    }

    pub(in crate::backend::llvm::compiler) fn build_default_value(
        &mut self,
        ptr: PointerValue<'ctx>,
        var_type: &Type,
        span: Span,
    ) -> Result<(), Box<dyn IError>> {
        let err = Self::builder_err(span);

        match var_type {
            Type::I64 => {
                let zero = self.context.i64_type().const_int(0, true);
                self.builder.build_store(ptr, zero).map_err(&err)?;
            }

            Type::F64 => {
                let zero = self.context.f64_type().const_float(0.0);
                self.builder.build_store(ptr, zero).map_err(&err)?;
            }

            Type::Bool => {
                let zero = self.context.bool_type().const_int(0, false);
                self.builder.build_store(ptr, zero).map_err(&err)?;
            }

            Type::Str => {
                let empty = self.builder.build_global_string_ptr("", "str.default").map_err(&err)?.as_pointer_value();

                self.builder.build_store(ptr, empty).map_err(&err)?;
            }

            Type::Vector(inner) => {
                let vector_ptr = self.build_empty_vector(inner, span)?;
                self.builder.build_store(ptr, vector_ptr).map_err(&err)?;
            }

            other => {
                return Err(Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    format!("Compiling default values of type '{:?}' is not yet supported.", other),
                    span,
                )) as Box<dyn IError>)
            }
        }

        Ok(())
    }
}
