use inkwell::values::PointerValue;
use inkwell::IntPredicate;

use super::Compiler;
use crate::{
    backend::llvm::llvm_alu::llvm_value::{LlvmValue, ENUM_REFCOUNT, STR_REFCOUNT, VEC_REFCOUNT},
    common::{
        errors::{CompilerError, ErrorSeverity, IError},
        span::Span,
        types::Type,
    },
    frontend::ast::Expression,
};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    // ========================================================================
    // Scope tracking
    // ========================================================================

    pub(in crate::backend::llvm::compiler) fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    pub(in crate::backend::llvm::compiler) fn declare_scoped_variable(&mut self, name: String, ptr: PointerValue<'ctx>, ty: Type) {
        self.variables.insert(name.clone(), (ptr, ty.clone()));

        if let Some(scope) = self.scopes.last_mut() {
            scope.push((name, ptr, ty));
        }
    }

    pub(in crate::backend::llvm::compiler) fn pop_scope_and_release(&mut self, span: Span) -> Result<(), Box<dyn IError>> {
        let scope = self.scopes.pop().unwrap_or_default();

        let has_terminator = self.builder.get_insert_block().and_then(|block| block.get_terminator()).is_some();

        for (name, ptr, ty) in scope.iter().rev() {
            self.variables.remove(name);

            if !has_terminator {
                let value = self.load_owned_variable(*ptr, ty, span)?;
                self.release_value(&value, span)?;
            }
        }

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn release_scopes_from(&mut self, depth: usize, span: Span) -> Result<(), Box<dyn IError>> {
        for scope_index in (depth..self.scopes.len()).rev() {
            let vars = self.scopes[scope_index].clone();

            for (_, ptr, ty) in vars.iter().rev() {
                let value = self.load_owned_variable(*ptr, ty, span)?;
                self.release_value(&value, span)?;
            }
        }

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn release_all_scopes(&mut self, span: Span) -> Result<(), Box<dyn IError>> {
        self.release_scopes_from(0, span)
    }

    fn load_owned_variable(&mut self, ptr: PointerValue<'ctx>, ty: &Type, span: Span) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        let err = Self::builder_err(span);

        let llvm_type = LlvmValue::type_to_basic_type_enum(ty, self.context).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Compiling values of type '{}' is not yet supported.", ty),
                span,
            )) as Box<dyn IError>
        })?;

        let raw = self.builder.build_load(llvm_type, ptr, "scope.drop.load").map_err(&err)?;

        Ok(LlvmValue::from_basic_value_enum(raw, ty))
    }

    pub(in crate::backend) fn expr_needs_retain(expr: &Expression) -> bool {
        matches!(expr, Expression::Variable(_))
    }

    // ========================================================================
    // Retain / release
    // ========================================================================

    pub(in crate::backend) fn expr_needs_release(expr: &Expression) -> bool {
        !matches!(expr, Expression::Variable(_))
    }

    pub(in crate::backend) fn expr_needs_release_in_function_call(expr: &Expression) -> bool {
        matches!(
            expr,
            Expression::Literal(_)
                | Expression::FunctionCall { .. }
                | Expression::Index { .. }
                | Expression::FieldAccess { .. }
                | Expression::Casting { .. }
        )
    }

    /// Increments the refcount of a heap-allocated value. No-op for
    /// primitives.
    pub(in crate::backend) fn retain_value(&mut self, value: &LlvmValue<'ctx>, span: Span) -> Result<(), Box<dyn IError>> {
        let err = Self::builder_err(span);

        match value {
            LlvmValue::Str(ptr) => {
                let header_type = LlvmValue::str_header_type(self.context);
                self.bump_refcount(header_type, *ptr, STR_REFCOUNT, 1, &err)?;
            }

            LlvmValue::Vector(ptr, _) => {
                let header_type = LlvmValue::vector_struct_type(self.context);
                self.bump_refcount(header_type, *ptr, VEC_REFCOUNT, 1, &err)?;
            }

            LlvmValue::Struct(ptr, ty) => {
                let Type::Struct { identifier, .. } = ty.as_ref() else {
                    return Ok(());
                };

                let (struct_type, _) = self.struct_llvm_type(identifier, span)?;
                let rc_index = Self::struct_refcount_field_index(struct_type);

                self.bump_refcount(struct_type, *ptr, rc_index, 1, &err)?;
            }

            // BUG FIX: this arm was missing, so enum values were never
            // retained - the second owner's refcount never went up, so the
            // first `release` could free the enum while someone else still
            // held it (use-after-free / segfault).
            LlvmValue::Enum(ptr, ty) => {
                let Type::Enum { identifier, .. } = ty.as_ref() else {
                    return Ok(());
                };

                let (header_type, _variant_indices, _ordered_variants) = self.enum_llvm_type(identifier, span)?;

                self.bump_refcount(header_type, *ptr, ENUM_REFCOUNT, 1, &err)?;
            }

            _ => {}
        }

        Ok(())
    }

    fn bump_refcount(
        &self,
        header_type: inkwell::types::StructType<'ctx>,
        ptr: PointerValue<'ctx>,
        field_index: u32,
        delta: i64,
        err: &impl Fn(inkwell::builder::BuilderError) -> Box<dyn IError>,
    ) -> Result<(), Box<dyn IError>> {
        let i64_type = self.context.i64_type();

        let rc_field = self.builder.build_struct_gep(header_type, ptr, field_index, "rc.field").map_err(err)?;

        let rc = self.builder.build_load(i64_type, rc_field, "rc.val").map_err(err)?.into_int_value();

        let updated = if delta >= 0 {
            self.builder
                .build_int_add(rc, i64_type.const_int(delta as u64, false), "rc.inc")
                .map_err(err)?
        } else {
            self.builder
                .build_int_sub(rc, i64_type.const_int((-delta) as u64, false), "rc.dec")
                .map_err(err)?
        };

        self.builder.build_store(rc_field, updated).map_err(err)?;

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn with_last_reference<F>(
        &mut self,
        header_type: inkwell::types::StructType<'ctx>,
        ptr: PointerValue<'ctx>,
        refcount_index: u32,
        span: Span,
        block_prefix: &str,
        body: F,
    ) -> Result<(), Box<dyn IError>>
    where
        F: FnOnce(&mut Self) -> Result<(), Box<dyn IError>>,
    {
        let err = Self::builder_err(span);
        let i64_type = self.context.i64_type();
        let function = self.current_function();

        let rc_field = self
            .builder
            .build_struct_gep(header_type, ptr, refcount_index, "rc.field")
            .map_err(&err)?;

        let rc = self.builder.build_load(i64_type, rc_field, "rc.val").map_err(&err)?.into_int_value();

        let one = i64_type.const_int(1, false);

        let is_last = self.builder.build_int_compare(IntPredicate::EQ, rc, one, "rc.is_last").map_err(&err)?;

        let free_block = self.context.append_basic_block(function, &format!("{}.free", block_prefix));
        let dec_block = self.context.append_basic_block(function, &format!("{}.dec", block_prefix));
        let merge_block = self.context.append_basic_block(function, &format!("{}.merge", block_prefix));

        self.builder.build_conditional_branch(is_last, free_block, dec_block).map_err(&err)?;

        self.builder.position_at_end(free_block);
        body(self)?;
        self.branch_if_no_terminator(merge_block, span)?;

        self.builder.position_at_end(dec_block);
        let decremented = self.builder.build_int_sub(rc, one, "rc.dec").map_err(&err)?;
        self.builder.build_store(rc_field, decremented).map_err(&err)?;
        self.branch_if_no_terminator(merge_block, span)?;

        self.builder.position_at_end(merge_block);

        Ok(())
    }
}
