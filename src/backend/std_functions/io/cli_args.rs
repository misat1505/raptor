use std::{cell::RefCell, env, rc::Rc, vec};

use inkwell::AddressSpace;

use crate::{
    backend::{
        interpreter::Value,
        llvm::{
            llvm_alu::llvm_value::{VEC_CAPACITY, VEC_DATA, VEC_LENGTH, VEC_REFCOUNT},
            LlvmValue,
        },
        std_functions::std_functions::{LlvmCompileFn, StdFunction},
    },
    common::{
        errors::{CompilerError, ErrorSeverity, IError, StdFunctionError},
        span::Span,
        types::Type,
    },
};

pub fn cli_args() -> StdFunction {
    let params: Vec<Type> = vec![];

    let execute = |_params: &Vec<Rc<RefCell<Value>>>, _span: Span| -> Result<Option<Value>, StdFunctionError> {
        let args: Vec<Rc<RefCell<Value>>> = env::args().map(|s| Rc::new(RefCell::new(Value::String(s)))).collect();

        Ok(Some(Value::Vector {
            kind: Box::new(Type::Vector(Box::new(Type::Str))),
            values: Rc::new(RefCell::new(args)),
        }))
    };

    let compile: LlvmCompileFn = |compiler, arguments, span| {
        let err = |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), span)) as Box<dyn IError>;

        if !arguments.is_empty() {
            return Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'cli_args' expects no arguments."),
                span,
            )));
        }

        let context = compiler.context();
        let i32_type = context.i32_type();
        let i64_type = context.i64_type();
        let ptr_type = context.ptr_type(AddressSpace::default());

        let argc_global = compiler.module().get_global("raptor_argc").ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("Internal error: raptor_argc global is missing (main not declared?)."),
                span,
            )) as Box<dyn IError>
        })?;
        let argv_global = compiler.module().get_global("raptor_argv").ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("Internal error: raptor_argv global is missing (main not declared?)."),
                span,
            )) as Box<dyn IError>
        })?;

        let argc_i32 = compiler
            .builder()
            .build_load(i32_type, argc_global.as_pointer_value(), "cli.argc")
            .map_err(err)?
            .into_int_value();
        let argc = compiler.builder().build_int_s_extend(argc_i32, i64_type, "cli.argc.i64").map_err(err)?;
        let argv = compiler
            .builder()
            .build_load(ptr_type, argv_global.as_pointer_value(), "cli.argv")
            .map_err(err)?
            .into_pointer_value();

        // VecHeader { rc, data, length, capacity }
        let vector_ty = LlvmValue::vector_struct_type(context);
        let vector_size = vector_ty.size_of().ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("Cannot compute size of vector header."),
                span,
            )) as Box<dyn IError>
        })?;
        let vector_ptr = compiler
            .builder()
            .build_call(compiler.libc().malloc_fn, &[vector_size.into()], "cli.vec.malloc")
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("malloc returns pointer")
            .into_pointer_value();

        let ptr_size = i64_type.const_int(8, false);
        let bytes = compiler.builder().build_int_mul(argc, ptr_size, "cli.data.bytes").map_err(err)?;
        let data_ptr = compiler
            .builder()
            .build_call(compiler.libc().malloc_fn, &[bytes.into()], "cli.data.malloc")
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("malloc returns pointer")
            .into_pointer_value();

        let rc_field = compiler
            .builder()
            .build_struct_gep(vector_ty, vector_ptr, VEC_REFCOUNT, "cli.vec.rc")
            .map_err(err)?;
        compiler.builder().build_store(rc_field, i64_type.const_int(1, false)).map_err(err)?;

        let data_field = compiler
            .builder()
            .build_struct_gep(vector_ty, vector_ptr, VEC_DATA, "cli.vec.data")
            .map_err(err)?;
        compiler.builder().build_store(data_field, data_ptr).map_err(err)?;

        let len_field = compiler
            .builder()
            .build_struct_gep(vector_ty, vector_ptr, VEC_LENGTH, "cli.vec.len")
            .map_err(err)?;
        compiler.builder().build_store(len_field, argc).map_err(err)?;

        let cap_field = compiler
            .builder()
            .build_struct_gep(vector_ty, vector_ptr, VEC_CAPACITY, "cli.vec.cap")
            .map_err(err)?;
        compiler.builder().build_store(cap_field, argc).map_err(err)?;

        let function = compiler
            .builder()
            .get_insert_block()
            .expect("builder positioned")
            .get_parent()
            .expect("parent fn");

        let i_ptr = compiler.builder().build_alloca(i64_type, "cli.i").map_err(err)?;
        compiler.builder().build_store(i_ptr, i64_type.const_int(0, false)).map_err(err)?;

        let loop_cond = context.append_basic_block(function, "cli.loop.cond");
        let loop_body = context.append_basic_block(function, "cli.loop.body");
        let loop_end = context.append_basic_block(function, "cli.loop.end");

        compiler.builder().build_unconditional_branch(loop_cond).map_err(err)?;

        compiler.builder().position_at_end(loop_cond);
        let i_val = compiler
            .builder()
            .build_load(i64_type, i_ptr, "cli.i.load")
            .map_err(err)?
            .into_int_value();
        let cmp = compiler
            .builder()
            .build_int_compare(inkwell::IntPredicate::SLT, i_val, argc, "cli.i.lt")
            .map_err(err)?;
        compiler.builder().build_conditional_branch(cmp, loop_body, loop_end).map_err(err)?;

        compiler.builder().position_at_end(loop_body);

        let argv_slot = unsafe { compiler.builder().build_in_bounds_gep(ptr_type, argv, &[i_val], "cli.argv.slot") }.map_err(err)?;
        let cstr = compiler
            .builder()
            .build_load(ptr_type, argv_slot, "cli.cstr")
            .map_err(err)?
            .into_pointer_value();

        let header = compiler.build_str_from_cstr(cstr, span)?;

        let data_slot = unsafe { compiler.builder().build_in_bounds_gep(ptr_type, data_ptr, &[i_val], "cli.data.slot") }.map_err(err)?;
        compiler.builder().build_store(data_slot, header).map_err(err)?;

        let i_next = compiler
            .builder()
            .build_int_add(i_val, i64_type.const_int(1, false), "cli.i.next")
            .map_err(err)?;
        compiler.builder().build_store(i_ptr, i_next).map_err(err)?;
        compiler.builder().build_unconditional_branch(loop_cond).map_err(err)?;

        compiler.builder().position_at_end(loop_end);

        compiler.set_last_value(LlvmValue::Vector(vector_ptr, Box::new(Type::Str)));
        Ok(())
    };

    StdFunction {
        params,
        passed_by: vec![],
        execute,
        return_type: Type::Vector(Box::new(Type::Str)),
        type_check: None,
        compile,
    }
}
