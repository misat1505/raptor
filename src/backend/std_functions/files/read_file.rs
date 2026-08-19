use std::{cell::RefCell, fs, rc::Rc, vec};

use crate::{
    backend::{
        interpreter::Value,
        llvm::LlvmValue,
        std_functions::std_functions::{build_usage_error, LlvmCompileFn, StdFunction},
    },
    common::{
        errors::{CompilerError, ErrorSeverity, IError, StdFunctionError},
        span::Span,
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::PassedBy,
};

pub fn read_file() -> StdFunction {
    let params = vec![Type::Str];
    let execute = |params: &Vec<Rc<RefCell<Value>>>, span: Span| -> Result<Option<Value>, StdFunctionError> {
        let fn_name = "read_file";
        let expected_types = vec![Type::Str];
        let mut actual_types: Vec<Type> = vec![];

        if let Some(filepath) = params.get(0) {
            actual_types.push(filepath.borrow().to_type());
            let filepath = filepath.borrow();

            match &*filepath {
                Value::String(path) => match fs::read_to_string(path) {
                    Ok(content) => Ok(Some(Value::String(content))),
                    Err(_) => Err(StdFunctionError::new(ErrorSeverity::HIGH, String::from("Failed to read file."), span)),
                },
                _ => Err(build_usage_error(fn_name, expected_types, actual_types, span)),
            }
        } else {
            Err(build_usage_error(fn_name, expected_types, actual_types, span))
        }
    };

    let compile: LlvmCompileFn = |compiler, arguments, span| {
        let err = |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), span)) as Box<dyn IError>;

        let arg = arguments.get(0).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'read_file' expects exactly one argument."),
                span,
            )) as Box<dyn IError>
        })?;

        compiler.visit_expression(&arg.value.value)?;
        let path_ptr = compiler.read_last_value()?.into_str_value(span)?;

        let context = compiler.context();
        let i64_type = context.i64_type();
        let i32_type = context.i32_type();

        let mode = compiler
            .builder()
            .build_global_string_ptr("rb", "mode.r")
            .map_err(err)?
            .as_pointer_value();

        let fopen_fn = compiler.libc().fopen_fn;
        let file = compiler
            .builder()
            .build_call(fopen_fn, &[path_ptr.into(), mode.into()], "fopen.call")
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("fopen should return a value")
            .into_pointer_value();

        // fseek(file, 0, SEEK_END=2) -> ftell -> fseek(file, 0, SEEK_SET=0)
        let fseek_fn = compiler.libc().fseek_fn;
        let ftell_fn = compiler.libc().ftell_fn;

        compiler
            .builder()
            .build_call(
                fseek_fn,
                &[file.into(), i64_type.const_int(0, false).into(), i32_type.const_int(2, false).into()],
                "fseek.end",
            )
            .map_err(err)?;

        let size = compiler
            .builder()
            .build_call(ftell_fn, &[file.into()], "ftell.call")
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("ftell should return a value")
            .into_int_value();

        compiler
            .builder()
            .build_call(
                fseek_fn,
                &[file.into(), i64_type.const_int(0, false).into(), i32_type.const_int(0, false).into()],
                "fseek.start",
            )
            .map_err(err)?;

        let size_plus_nul = compiler
            .builder()
            .build_int_add(size, i64_type.const_int(1, false), "size.nul")
            .map_err(err)?;

        let malloc_fn = compiler.libc().malloc_fn;
        let buf = compiler
            .builder()
            .build_call(malloc_fn, &[size_plus_nul.into()], "read.buf")
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();

        let fread_fn = compiler.libc().fread_fn;
        compiler
            .builder()
            .build_call(
                fread_fn,
                &[buf.into(), i64_type.const_int(1, false).into(), size.into(), file.into()],
                "fread.call",
            )
            .map_err(err)?;

        let fclose_fn = compiler.libc().fclose_fn;
        compiler.builder().build_call(fclose_fn, &[file.into()], "fclose.call").map_err(err)?;

        // NUL-terminator na końcu bufora
        let end_ptr = unsafe { compiler.builder().build_gep(context.i8_type(), buf, &[size], "read.end").map_err(err)? };

        compiler
            .builder()
            .build_store(end_ptr, context.i8_type().const_int(0, false))
            .map_err(err)?;

        compiler.set_last_value(LlvmValue::Str(buf));
        Ok(())
    };

    StdFunction {
        params,
        execute,
        passed_by: vec![PassedBy::Value],
        return_type: Type::Str,
        type_check: None,
        compile,
    }
}
