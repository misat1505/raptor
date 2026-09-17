use inkwell::{
    values::{FunctionValue, PointerValue},
    AddressSpace,
};

use crate::{
    backend::llvm::{
        compiler::ReleaseKey,
        llvm_alu::llvm_value::{LlvmValue, ENUM_PAYLOAD, ENUM_REFCOUNT, ENUM_TAG, STR_DATA, STR_REFCOUNT, VEC_DATA, VEC_LENGTH, VEC_REFCOUNT},
    },
    common::{errors::IError, span::Span, types::Type},
};

use super::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    // Emits a release operation for a heap-managed value.
    //
    // IMPORTANT:
    // This function does NOT recursively generate release code.
    //
    // Instead it:
    //
    // 1. obtains/caches the appropriate release function,
    // 2. emits a call to that function.
    //
    // This is what makes recursive types such as:
    //
    //     Node -> NodeKind -> Node[] -> Node
    //
    // safe during compilation.
    pub(in crate::backend) fn release_value(&mut self, value: &LlvmValue<'ctx>, span: Span) -> Result<(), Box<dyn IError>> {
        match value {
            LlvmValue::Str(ptr) => {
                let function = self.get_or_create_release_str(span)?;
                self.emit_release_call(function, *ptr, span)?;
            }

            LlvmValue::Vector(ptr, inner) => {
                let function = self.get_or_create_release_vector(inner, span)?;
                self.emit_release_call(function, *ptr, span)?;
            }

            LlvmValue::Struct(ptr, ty) => {
                let Type::Struct { identifier, .. } = ty.as_ref() else {
                    return Ok(());
                };

                let function = self.get_or_create_release_struct(identifier, span)?;

                self.emit_release_call(function, *ptr, span)?;
            }

            LlvmValue::Enum(ptr, ty) => {
                let Type::Enum { identifier, .. } = ty.as_ref() else {
                    return Ok(());
                };

                let function = self.get_or_create_release_enum(identifier, span)?;

                self.emit_release_call(function, *ptr, span)?;
            }

            _ => {}
        }

        Ok(())
    }

    fn emit_release_call(&mut self, function: FunctionValue<'ctx>, ptr: PointerValue<'ctx>, span: Span) -> Result<(), Box<dyn IError>> {
        let err = Self::builder_err(span);

        self.builder.build_call(function, &[ptr.into()], "release.call").map_err(&err)?;

        Ok(())
    }

    // ========================================================================
    // Release function cache
    // ========================================================================

    fn get_or_create_release_str(&mut self, span: Span) -> Result<FunctionValue<'ctx>, Box<dyn IError>> {
        let key = ReleaseKey::Str;

        if let Some(function) = self.release_functions.get(&key) {
            return Ok(*function);
        }

        let function = self.create_release_function("__release_str");

        // IMPORTANT:
        // Insert before generating the body.
        self.release_functions.insert(key, function);

        self.generate_release_str_body(function, span)?;

        Ok(function)
    }

    fn get_or_create_release_vector(&mut self, inner: &Type, span: Span) -> Result<FunctionValue<'ctx>, Box<dyn IError>> {
        let inner = self.resolve_type(inner);

        /*
         * Type itself does not implement Eq + Hash in this project.
         *
         * Use its canonical textual name as the cache key instead.
         */
        let key = ReleaseKey::Vector(Self::sanitize_type_name(&inner));

        if let Some(function) = self.release_functions.get(&key) {
            return Ok(*function);
        }

        let name = format!("__release_vector_{}", Self::sanitize_type_name(&inner));

        let function = self.create_release_function(&name);

        /*
         * IMPORTANT:
         *
         * Cache BEFORE generating the body.
         *
         * For:
         *
         *     Node -> NodeKind -> Node[]
         *
         * generating __release_struct_Node eventually asks for
         * __release_vector_Node.
         *
         * While generating that vector function, Node is encountered again.
         * The already cached __release_struct_Node is then used instead of
         * recursively generating another function body.
         */
        self.release_functions.insert(key, function);

        self.generate_release_vector_body(function, &inner, span)?;

        Ok(function)
    }

    fn get_or_create_release_struct(&mut self, identifier: &str, span: Span) -> Result<FunctionValue<'ctx>, Box<dyn IError>> {
        let key = ReleaseKey::Struct(identifier.to_string());

        if let Some(function) = self.release_functions.get(&key) {
            return Ok(*function);
        }

        let name = format!("__release_struct_{}", identifier);

        let function = self.create_release_function(&name);

        // Cache before body generation.
        self.release_functions.insert(key, function);

        self.generate_release_struct_body(function, identifier, span)?;

        Ok(function)
    }

    fn get_or_create_release_enum(&mut self, identifier: &str, span: Span) -> Result<FunctionValue<'ctx>, Box<dyn IError>> {
        let key = ReleaseKey::Enum(identifier.to_string());

        if let Some(function) = self.release_functions.get(&key) {
            return Ok(*function);
        }

        let name = format!("__release_enum_{}", identifier);

        let function = self.create_release_function(&name);

        // Cache before body generation.
        self.release_functions.insert(key, function);

        self.generate_release_enum_body(function, identifier, span)?;

        Ok(function)
    }

    fn create_release_function(&self, name: &str) -> FunctionValue<'ctx> {
        let ptr_type = self.context.ptr_type(AddressSpace::default());

        let fn_type = self.context.void_type().fn_type(&[ptr_type.into()], false);

        self.module.add_function(name, fn_type, None)
    }

    // ========================================================================
    // String
    // ========================================================================

    fn generate_release_str_body(&mut self, function: FunctionValue<'ctx>, span: Span) -> Result<(), Box<dyn IError>> {
        let entry = self.context.append_basic_block(function, "entry");

        let old_block = self.builder.get_insert_block();

        self.builder.position_at_end(entry);

        let ptr = function.get_first_param().unwrap().into_pointer_value();

        /*
         * The string header layout is defined by LlvmValue.
         *
         * StrHeader {
         *     refcount: i64,
         *     data: i8*
         * }
         */
        let header_type = LlvmValue::str_header_type(self.context);

        self.with_last_reference(header_type, ptr, STR_REFCOUNT, span, "str.release", move |compiler| {
            let err = Self::builder_err(span);

            let data_field = compiler
                .builder
                .build_struct_gep(header_type, ptr, STR_DATA, "str.release.data")
                .map_err(&err)?;

            let data = compiler
                .builder
                .build_load(compiler.context.ptr_type(AddressSpace::default()), data_field, "str.release.data.val")
                .map_err(&err)?
                .into_pointer_value();

            compiler
                .builder
                .build_call(compiler.libc.free_fn, &[data.into()], "str.data.free")
                .map_err(&err)?;

            compiler
                .builder
                .build_call(compiler.libc.free_fn, &[ptr.into()], "str.header.free")
                .map_err(&err)?;

            Ok(())
        })?;

        self.builder.build_return(None).map_err(Self::builder_err(span))?;

        if let Some(block) = old_block {
            self.builder.position_at_end(block);
        }

        Ok(())
    }

    // ========================================================================
    // Vector
    // ========================================================================

    fn generate_release_vector_body(&mut self, function: FunctionValue<'ctx>, inner: &Type, span: Span) -> Result<(), Box<dyn IError>> {
        let entry = self.context.append_basic_block(function, "entry");

        let old_block = self.builder.get_insert_block();

        self.builder.position_at_end(entry);

        let ptr = function.get_first_param().unwrap().into_pointer_value();

        let inner = self.resolve_type(inner);

        /*
         * Vector header layout is defined by LlvmValue:
         *
         * VecHeader {
         *     refcount: i64,
         *     data: T*,
         *     length: i64,
         *     capacity: i64
         * }
         */
        let header_type = LlvmValue::vector_struct_type(self.context);

        let i64_type = self.context.i64_type();

        /*
         * Only heap-managed elements need to be released.
         */
        let owned = matches!(inner, Type::Str | Type::Vector(_) | Type::Struct { .. } | Type::Enum { .. });

        self.with_last_reference(header_type, ptr, VEC_REFCOUNT, span, "vector.release", move |compiler| {
            let err = Self::builder_err(span);

            if owned {
                // --------------------------------------------------------
                // data
                // --------------------------------------------------------

                let data_field = compiler
                    .builder
                    .build_struct_gep(header_type, ptr, VEC_DATA, "vec.release.data.field")
                    .map_err(&err)?;

                let data = compiler
                    .builder
                    .build_load(compiler.context.ptr_type(AddressSpace::default()), data_field, "vec.release.data")
                    .map_err(&err)?
                    .into_pointer_value();

                // --------------------------------------------------------
                // length
                // --------------------------------------------------------

                let len_field = compiler
                    .builder
                    .build_struct_gep(header_type, ptr, VEC_LENGTH, "vec.release.length.field")
                    .map_err(&err)?;

                let len = compiler
                    .builder
                    .build_load(i64_type, len_field, "vec.release.length")
                    .map_err(&err)?
                    .into_int_value();

                // --------------------------------------------------------
                // loop blocks
                // --------------------------------------------------------

                let element_type = LlvmValue::type_to_basic_type_enum(&inner, compiler.context).expect("owned vector elements must be LLVM values");

                let current_function = compiler.current_function();

                let loop_block = compiler.context.append_basic_block(current_function, "vec.release.loop");

                let body_block = compiler.context.append_basic_block(current_function, "vec.release.body");

                let after_block = compiler.context.append_basic_block(current_function, "vec.release.after");

                /*
                 * IMPORTANT:
                 *
                 * with_last_reference() positioned the builder at
                 * the "last reference" block.
                 *
                 * That block is the real predecessor of loop_block.
                 *
                 * It is NOT the function's original `entry` block.
                 */
                let loop_entry = compiler
                    .builder
                    .get_insert_block()
                    .expect("vector release loop must have an insertion block");

                compiler.builder.build_unconditional_branch(loop_block).map_err(&err)?;

                // --------------------------------------------------------
                // loop condition
                // --------------------------------------------------------

                compiler.builder.position_at_end(loop_block);

                let index = compiler.builder.build_phi(i64_type, "vec.release.index").map_err(&err)?;

                let zero = i64_type.const_zero();

                /*
                 * First iteration:
                 *
                 *     index = 0
                 *
                 * The incoming block must be `loop_entry`, because
                 * that is the actual predecessor of loop_block.
                 */
                index.add_incoming(&[(&zero, loop_entry)]);

                let index_value = index.as_basic_value().into_int_value();

                let condition = compiler
                    .builder
                    .build_int_compare(inkwell::IntPredicate::ULT, index_value, len, "vec.release.has_next")
                    .map_err(&err)?;

                compiler
                    .builder
                    .build_conditional_branch(condition, body_block, after_block)
                    .map_err(&err)?;

                // --------------------------------------------------------
                // loop body
                // --------------------------------------------------------

                compiler.builder.position_at_end(body_block);

                let elem_ptr = unsafe {
                    compiler
                        .builder
                        .build_gep(element_type, data, &[index_value], "vec.release.elem")
                        .map_err(&err)?
                };

                let elem_raw = compiler
                    .builder
                    .build_load(element_type, elem_ptr, "vec.release.elem.value")
                    .map_err(&err)?;

                let elem_value = LlvmValue::from_basic_value_enum(elem_raw, &inner);

                /*
                 * IMPORTANT:
                 *
                 * release_value() only emits a call to the cached
                 * release function.
                 *
                 * For example:
                 *
                 *     Node[]
                 *
                 * becomes:
                 *
                 *     call void @__release_struct_Node(...)
                 *
                 * instead of recursively generating Node's release
                 * function here.
                 */
                compiler.release_value(&elem_value, span)?;

                // --------------------------------------------------------
                // index++
                // --------------------------------------------------------

                let next = compiler
                    .builder
                    .build_int_add(index_value, i64_type.const_int(1, false), "vec.release.next")
                    .map_err(&err)?;

                compiler.builder.build_unconditional_branch(loop_block).map_err(&err)?;

                /*
                 * Second incoming edge of the PHI:
                 *
                 *     body -> loop
                 *
                 * therefore:
                 *
                 *     index = next
                 */
                index.add_incoming(&[(&next, body_block)]);

                compiler.builder.position_at_end(after_block);
            }

            // ------------------------------------------------------------
            // free vector data
            // ------------------------------------------------------------

            let data_field = compiler
                .builder
                .build_struct_gep(header_type, ptr, VEC_DATA, "vec.release.data.field")
                .map_err(&err)?;

            let data = compiler
                .builder
                .build_load(compiler.context.ptr_type(AddressSpace::default()), data_field, "vec.release.data")
                .map_err(&err)?
                .into_pointer_value();

            compiler
                .builder
                .build_call(compiler.libc.free_fn, &[data.into()], "vec.data.free")
                .map_err(&err)?;

            // ------------------------------------------------------------
            // free vector header
            // ------------------------------------------------------------

            compiler
                .builder
                .build_call(compiler.libc.free_fn, &[ptr.into()], "vec.header.free")
                .map_err(&err)?;

            Ok(())
        })?;

        self.builder.build_return(None).map_err(Self::builder_err(span))?;

        if let Some(block) = old_block {
            self.builder.position_at_end(block);
        }

        Ok(())
    }

    // ========================================================================
    // Struct
    // ========================================================================

    fn generate_release_struct_body(&mut self, function: FunctionValue<'ctx>, identifier: &str, span: Span) -> Result<(), Box<dyn IError>> {
        let entry = self.context.append_basic_block(function, "entry");

        let old_block = self.builder.get_insert_block();

        self.builder.position_at_end(entry);

        let ptr = function.get_first_param().unwrap().into_pointer_value();

        let declaration = self.struct_declaration(identifier, span)?;
        let members = declaration.members.clone();
        let (struct_type, field_indices) = self.struct_llvm_type(identifier, span)?;

        let mut owned_fields = Vec::new();

        for member in &members {
            let name = member.value.identifier.value.clone();
            let ty = self.resolve_type(&member.value.member_type.value);

            if matches!(ty, Type::Str | Type::Vector(_) | Type::Struct { .. } | Type::Enum { .. }) {
                if let Some(index) = field_indices.get(&name) {
                    owned_fields.push((*index, ty));
                }
            }
        }

        let refcount_field = Compiler::struct_refcount_field_index(struct_type);

        self.with_last_reference(struct_type, ptr, refcount_field, span, "struct.release", move |compiler| {
            let err = Self::builder_err(span);

            for (index, ty) in &owned_fields {
                let field = compiler
                    .builder
                    .build_struct_gep(struct_type, ptr, *index, "struct.release.field")
                    .map_err(&err)?;

                let llvm_type = LlvmValue::type_to_basic_type_enum(ty, compiler.context).expect("owned struct field must be LLVM value");

                let raw = compiler.builder.build_load(llvm_type, field, "struct.release.value").map_err(&err)?;

                let value = LlvmValue::from_basic_value_enum(raw, ty);

                compiler.release_value(&value, span)?;
            }

            compiler
                .builder
                .build_call(compiler.libc.free_fn, &[ptr.into()], "struct.header.free")
                .map_err(&err)?;

            Ok(())
        })?;

        self.builder.build_return(None).map_err(Self::builder_err(span))?;

        if let Some(block) = old_block {
            self.builder.position_at_end(block);
        }

        Ok(())
    }

    // ========================================================================
    // Enum
    // ========================================================================

    fn generate_release_enum_body(&mut self, function: FunctionValue<'ctx>, identifier: &str, span: Span) -> Result<(), Box<dyn IError>> {
        let entry = self.context.append_basic_block(function, "entry");

        let old_block = self.builder.get_insert_block();

        self.builder.position_at_end(entry);

        let ptr = function.get_first_param().unwrap().into_pointer_value();

        let (header_type, _, ordered_variants) = self.enum_llvm_type(identifier, span)?;

        let i64_type = self.context.i64_type();

        let mut owned_variants = Vec::new();

        for (tag, (_, payload_type)) in ordered_variants.iter().enumerate() {
            if let Some(payload_type) = payload_type {
                let resolved = self.resolve_type(payload_type);

                if matches!(resolved, Type::Str | Type::Vector(_) | Type::Struct { .. } | Type::Enum { .. }) {
                    owned_variants.push((tag as u32, resolved));
                }
            }
        }

        self.with_last_reference(header_type, ptr, ENUM_REFCOUNT, span, "enum.release", move |compiler| {
            let err = Self::builder_err(span);

            if !owned_variants.is_empty() {
                // --------------------------------------------------------
                // tag
                // --------------------------------------------------------

                let tag_field = compiler
                    .builder
                    .build_struct_gep(header_type, ptr, ENUM_TAG, "enum.release.tag")
                    .map_err(&err)?;

                let tag = compiler
                    .builder
                    .build_load(i64_type, tag_field, "enum.release.tag.value")
                    .map_err(&err)?
                    .into_int_value();

                // --------------------------------------------------------
                // payload
                // --------------------------------------------------------

                let payload_field = compiler
                    .builder
                    .build_struct_gep(header_type, ptr, ENUM_PAYLOAD, "enum.release.payload")
                    .map_err(&err)?;

                let current_function = compiler.current_function();

                let after = compiler.context.append_basic_block(current_function, "enum.release.after");

                let mut cases = Vec::new();
                let mut blocks = Vec::new();

                for (tag_value, ty) in &owned_variants {
                    let block = compiler
                        .context
                        .append_basic_block(current_function, &format!("enum.release.variant_{}", tag_value));

                    cases.push((i64_type.const_int(*tag_value as u64, false), block));

                    blocks.push((block, ty.clone()));
                }

                compiler.builder.build_switch(tag, after, &cases).map_err(&err)?;

                // --------------------------------------------------------
                // variant bodies
                // --------------------------------------------------------

                for (block, ty) in blocks {
                    compiler.builder.position_at_end(block);

                    let llvm_type = LlvmValue::type_to_basic_type_enum(&ty, compiler.context).expect("owned enum payload must be LLVM value");

                    let raw = compiler
                        .builder
                        .build_load(llvm_type, payload_field, "enum.release.payload.value")
                        .map_err(&err)?;

                    let value = LlvmValue::from_basic_value_enum(raw, &ty);

                    /*
                     * Again, recursive types are safe here.
                     *
                     * NodeKind::Array(Node[])
                     *
                     * becomes:
                     *
                     *     call @__release_vector_Node
                     *
                     * and does not generate the whole Node release
                     * implementation recursively.
                     */
                    compiler.release_value(&value, span)?;

                    compiler.branch_if_no_terminator(after, span)?;
                }

                compiler.builder.position_at_end(after);
            }

            // ------------------------------------------------------------
            // free enum header
            // ------------------------------------------------------------

            compiler
                .builder
                .build_call(compiler.libc.free_fn, &[ptr.into()], "enum.header.free")
                .map_err(&err)?;

            Ok(())
        })?;

        self.builder.build_return(None).map_err(Self::builder_err(span))?;

        if let Some(block) = old_block {
            self.builder.position_at_end(block);
        }

        Ok(())
    }

    // ========================================================================
    // Type names used for release-function names/cache keys
    // ========================================================================

    fn sanitize_type_name(ty: &Type) -> String {
        match ty {
            Type::Str => "str".into(),

            Type::Vector(inner) => {
                format!("vec_{}", Self::sanitize_type_name(inner))
            }

            Type::Struct { identifier, .. } => identifier.clone(),

            Type::Enum { identifier, .. } => identifier.clone(),

            other => format!("{:?}", other).replace(|c: char| !c.is_ascii_alphanumeric(), "_"),
        }
    }
}
