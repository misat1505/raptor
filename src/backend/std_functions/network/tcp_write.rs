use std::{cell::RefCell, io::Write, rc::Rc, vec};

use crate::{
    backend::{
        interpreter::Value,
        std_functions::{
            network::state::STREAMS,
            std_functions::{build_usage_error, LlvmCompileFn, StdFunction},
        },
    },
    common::{
        errors::{CompilerError, ErrorSeverity, IError, StdFunctionError},
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::PassedBy,
};

pub fn tcp_write() -> StdFunction {
    let params = vec![Type::I64, Type::Str];

    let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
        let fn_name = "tcp_write";
        let expected_types = vec![Type::I64, Type::Str];

        let mut actual_types: Vec<Type> = vec![];

        let handle_param = params.get(0);
        let data_param = params.get(1);

        if let (Some(handle), Some(data)) = (handle_param, data_param) {
            actual_types.push(handle.borrow().to_type());
            actual_types.push(data.borrow().to_type());

            let handle = handle.borrow();
            let data = data.borrow();

            match (&*handle, &*data) {
                (Value::I64(stream_handle), Value::String(payload)) => {
                    let mut streams = STREAMS.lock().unwrap();
                    let stream = streams
                        .as_mut()
                        .and_then(|m| m.get_mut(stream_handle))
                        .ok_or_else(|| StdFunctionError::new(ErrorSeverity::HIGH, format!("Invalid stream handle {}", stream_handle)))?;

                    stream
                        .write_all(payload.as_bytes())
                        .map_err(|e| StdFunctionError::new(ErrorSeverity::HIGH, format!("Write failed: {}", e)))?;

                    Ok(None)
                }
                _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
            }
        } else {
            Err(build_usage_error(fn_name, expected_types, actual_types))
        }
    };

    let compile: LlvmCompileFn = |compiler, arguments, position| {
        let err = |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

        let fd_arg = arguments.get(0).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'tcp_write' expects exactly two arguments."),
                position,
            )) as Box<dyn IError>
        })?;
        let data_arg = arguments.get(1).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'tcp_write' expects exactly two arguments."),
                position,
            )) as Box<dyn IError>
        })?;

        compiler.visit_expression(&fd_arg.value.value)?;
        let fd = compiler.read_last_value()?.into_i64_value(position)?;

        compiler.visit_expression(&data_arg.value.value)?;
        let data_ptr = compiler.read_last_value()?.into_str_value(position)?;

        let context = compiler.context();
        let i32_type = context.i32_type();
        let i64_type = context.i64_type();

        let fd_i32 = compiler.builder().build_int_truncate(fd, i32_type, "fd.i32").map_err(err)?;

        let strlen_fn = compiler.libc().strlen_fn;
        let len = compiler
            .builder()
            .build_call(strlen_fn, &[data_ptr.into()], "data.len")
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("strlen should return a value")
            .into_int_value();

        let send_fn = compiler.libc().send_fn;
        compiler
            .builder()
            .build_call(
                send_fn,
                &[fd_i32.into(), data_ptr.into(), len.into(), i32_type.const_int(0, false).into()],
                "send.call",
            )
            .map_err(err)?;

        let _ = i64_type;
        Ok(())
    };

    StdFunction {
        params,
        passed_by: vec![PassedBy::Value, PassedBy::Value],
        execute,
        return_type: Type::Void,
        type_check: None,
        compile,
    }
}
