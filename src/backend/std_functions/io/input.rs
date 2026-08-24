use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
    vec,
};

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

pub fn input() -> StdFunction {
    let params = vec![Type::Str];

    let execute = |params: &Vec<Rc<RefCell<Value>>>, span: Span| -> Result<Option<Value>, StdFunctionError> {
        let fn_name = "input";
        let expected_types = vec![Type::Str];
        let mut actual_types: Vec<Type> = vec![];

        if let Some(value) = params.get(0) {
            actual_types.push(value.borrow().to_type());
            let value = value.borrow();

            match &*value {
                Value::String(prompt) => {
                    print!("{}", prompt);
                    io::stdout().flush().unwrap();

                    let mut input = String::new();

                    match io::stdin().read_line(&mut input) {
                        Ok(_) => Ok(Some(Value::String(input.trim().to_string()))),
                        Err(_) => Err(StdFunctionError::new(ErrorSeverity::HIGH, String::from("Failed to read input."), span)),
                    }
                }

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
                String::from("'input' expects exactly one argument."),
                span,
            )) as Box<dyn IError>
        })?;

        compiler.visit_expression(&arg.value.value)?;
        let prompt_ptr = compiler.read_last_value()?.into_str_value(span)?;

        // printf("%s", prompt)
        let format_str = compiler
            .builder()
            .build_global_string_ptr("%s", "fmt.prompt")
            .map_err(err)?
            .as_pointer_value();

        let printf_fn = compiler.libc().printf_fn;

        compiler
            .builder()
            .build_call(printf_fn, &[format_str.into(), prompt_ptr.into()], "printf.prompt")
            .map_err(err)?;

        let free_fn = compiler.libc().free_fn;

        compiler.builder().build_call(free_fn, &[prompt_ptr.into()], "free.prompt").map_err(err)?;

        let context = compiler.context();
        let i64_type = context.i64_type();
        let i32_type = context.i32_type();

        const BUF_SIZE: u64 = 4096;

        // malloc(4096)
        let malloc_fn = compiler.libc().malloc_fn;

        let buf = compiler
            .builder()
            .build_call(malloc_fn, &[i64_type.const_int(BUF_SIZE, false).into()], "input.buf")
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("malloc should return a value")
            .into_pointer_value();

        // read(0, buf, 4095)
        let read_fn = compiler.libc().read_fn;

        let n = compiler
            .builder()
            .build_call(
                read_fn,
                &[
                    i32_type.const_int(0, false).into(),
                    buf.into(),
                    i64_type.const_int(BUF_SIZE - 1, false).into(),
                ],
                "read.call",
            )
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("read should return a value")
            .into_int_value();

        // buf[n] = '\0'
        let end_ptr = unsafe { compiler.builder().build_gep(context.i8_type(), buf, &[n], "input.end").map_err(err)? };

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
