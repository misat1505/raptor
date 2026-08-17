use std::{cell::RefCell, collections::HashMap, net::TcpListener, rc::Rc, vec};

use crate::{
    backend::{
        interpreter::Value,
        llvm::LlvmValue,
        std_functions::{
            network::state::{next_handle, LISTENERS},
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

pub fn tcp_listen() -> StdFunction {
    let params = vec![Type::I64];

    let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
        let fn_name = "tcp_listen";
        let expected_types = vec![Type::I64];

        let mut actual_types: Vec<Type> = vec![];

        if let Some(port) = params.get(0) {
            actual_types.push(port.borrow().to_type());

            let port = port.borrow();

            match &*port {
                Value::I64(p) => {
                    let listener = TcpListener::bind(format!("0.0.0.0:{}", p))
                        .map_err(|e| StdFunctionError::new(ErrorSeverity::HIGH, format!("Cannot bind to port {}: {}", p, e)))?;

                    let handle = next_handle();
                    LISTENERS.lock().unwrap().get_or_insert_with(HashMap::new).insert(handle, listener);

                    Ok(Some(Value::I64(handle)))
                }
                _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
            }
        } else {
            Err(build_usage_error(fn_name, expected_types, actual_types))
        }
    };

    let compile: LlvmCompileFn = |compiler, arguments, position| {
        let err = |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

        let arg = arguments.get(0).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'tcp_listen' expects exactly one argument."),
                position,
            )) as Box<dyn IError>
        })?;

        compiler.visit_expression(&arg.value.value)?;
        let port = compiler.read_last_value()?.into_i64_value(position)?;

        let context = compiler.context();
        let i16_type = context.i16_type();
        let i32_type = context.i32_type();
        let i8_type = context.i8_type();

        // socket(AF_INET=2, SOCK_STREAM=1, 0)
        let socket_fn = compiler.libc().socket_fn;
        let fd = compiler
            .builder()
            .build_call(
                socket_fn,
                &[
                    i32_type.const_int(2, false).into(),
                    i32_type.const_int(1, false).into(),
                    i32_type.const_int(0, false).into(),
                ],
                "socket.call",
            )
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("socket should return a value")
            .into_int_value();

        // budujemy sockaddr_in (16 bajtów) na stosie
        let sockaddr_type = context.struct_type(&[i16_type.into(), i16_type.into(), i32_type.into(), i8_type.array_type(8).into()], false);
        let sockaddr_ptr = compiler.builder().build_alloca(sockaddr_type, "sockaddr").map_err(err)?;

        let family_field = compiler
            .builder()
            .build_struct_gep(sockaddr_type, sockaddr_ptr, 0, "sockaddr.family")
            .map_err(err)?;
        compiler.builder().build_store(family_field, i16_type.const_int(2, false)).map_err(err)?; // AF_INET

        // port trzeba zapisać w big-endian (htons) — port ma zakres 0-65535, więc rzutujemy na i16 po zamianie bajtów
        let port_i32 = compiler.builder().build_int_truncate(port, i32_type, "port.i32").map_err(err)?;
        let port_lo = compiler
            .builder()
            .build_and(port_i32, i32_type.const_int(0xFF, false), "port.lo")
            .map_err(err)?;
        let port_hi = compiler
            .builder()
            .build_right_shift(
                compiler
                    .builder()
                    .build_and(port_i32, i32_type.const_int(0xFF00, false), "port.hi.mask")
                    .map_err(err)?,
                i32_type.const_int(8, false),
                false,
                "port.hi",
            )
            .map_err(err)?;
        let port_be = compiler
            .builder()
            .build_or(
                compiler
                    .builder()
                    .build_left_shift(port_lo, i32_type.const_int(8, false), "port.lo.shifted")
                    .map_err(err)?,
                port_hi,
                "port.be",
            )
            .map_err(err)?;
        let port_be_i16 = compiler.builder().build_int_truncate(port_be, i16_type, "port.be.i16").map_err(err)?;

        let port_field = compiler
            .builder()
            .build_struct_gep(sockaddr_type, sockaddr_ptr, 1, "sockaddr.port")
            .map_err(err)?;
        compiler.builder().build_store(port_field, port_be_i16).map_err(err)?;

        let addr_field = compiler
            .builder()
            .build_struct_gep(sockaddr_type, sockaddr_ptr, 2, "sockaddr.addr")
            .map_err(err)?;
        compiler.builder().build_store(addr_field, i32_type.const_int(0, false)).map_err(err)?; // INADDR_ANY

        let zero_field = compiler
            .builder()
            .build_struct_gep(sockaddr_type, sockaddr_ptr, 3, "sockaddr.zero")
            .map_err(err)?;
        compiler
            .builder()
            .build_store(zero_field, i8_type.array_type(8).const_zero())
            .map_err(err)?;

        let bind_fn = compiler.libc().bind_fn;
        compiler
            .builder()
            .build_call(
                bind_fn,
                &[fd.into(), sockaddr_ptr.into(), i32_type.const_int(16, false).into()],
                "bind.call",
            )
            .map_err(err)?;

        let listen_fn = compiler.libc().listen_fn;
        compiler
            .builder()
            .build_call(listen_fn, &[fd.into(), i32_type.const_int(128, false).into()], "listen.call")
            .map_err(err)?;

        let fd_i64 = compiler.builder().build_int_z_extend(fd, context.i64_type(), "fd.i64").map_err(err)?;
        compiler.set_last_value(LlvmValue::I64(fd_i64));
        Ok(())
    };

    StdFunction {
        params,
        passed_by: vec![PassedBy::Value],
        execute,
        return_type: Type::I64,
        type_check: None,
        compile,
    }
}
