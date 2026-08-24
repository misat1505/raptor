use std::{cell::RefCell, collections::HashMap, net::TcpStream, rc::Rc, vec};

use crate::{
    backend::{
        interpreter::Value,
        llvm::LlvmValue,
        std_functions::{
            network::state::{next_handle, STREAMS},
            std_functions::{build_usage_error, LlvmCompileFn, StdFunction},
        },
    },
    common::{
        errors::{CompilerError, ErrorSeverity, IError, StdFunctionError},
        span::Span,
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::PassedBy,
};

pub fn tcp_connect() -> StdFunction {
    let params = vec![Type::Str, Type::I64];

    let execute = |params: &Vec<Rc<RefCell<Value>>>, span: Span| -> Result<Option<Value>, StdFunctionError> {
        let fn_name = "tcp_connect";
        let expected_types = vec![Type::Str, Type::I64];
        let mut actual_types: Vec<Type> = vec![];

        let host_val = params.get(0);
        let port_val = params.get(1);

        if let (Some(host), Some(port)) = (host_val, port_val) {
            actual_types.push(host.borrow().to_type());
            actual_types.push(port.borrow().to_type());

            let host = host.borrow();
            let port = port.borrow();

            match (&*host, &*port) {
                (Value::String(h), Value::I64(p)) => match TcpStream::connect(format!("{}:{}", h, p)) {
                    Ok(stream) => {
                        let handle = next_handle();
                        STREAMS.lock().unwrap().get_or_insert_with(HashMap::new).insert(handle, stream);
                        Ok(Some(Value::I64(handle)))
                    }
                    Err(_) => Ok(Some(Value::I64(-1))),
                },
                _ => Err(build_usage_error(fn_name, expected_types, actual_types, span)),
            }
        } else {
            Err(build_usage_error(fn_name, expected_types, actual_types, span))
        }
    };

    let compile: LlvmCompileFn = |compiler, arguments, span| {
        let err = |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), span)) as Box<dyn IError>;

        let host_arg = arguments.get(0).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'tcp_connect' expects exactly two arguments (host, port)."),
                span,
            )) as Box<dyn IError>
        })?;
        let port_arg = arguments.get(1).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'tcp_connect' expects exactly two arguments (host, port)."),
                span,
            )) as Box<dyn IError>
        })?;

        compiler.visit_expression(&host_arg.value.value)?;
        let host_ptr = compiler.read_last_value()?.into_str_value(span)?;

        compiler.visit_expression(&port_arg.value.value)?;
        let port = compiler.read_last_value()?.into_i64_value(span)?;

        let context = compiler.context();
        let i16_type = context.i16_type();
        let i32_type = context.i32_type();
        let i8_type = context.i8_type();
        let i64_type = context.i64_type();

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

        let sockaddr_type = context.struct_type(&[i16_type.into(), i16_type.into(), i32_type.into(), i8_type.array_type(8).into()], false);
        let sockaddr_ptr = compiler.builder().build_alloca(sockaddr_type, "sockaddr").map_err(err)?;

        let family_field = compiler
            .builder()
            .build_struct_gep(sockaddr_type, sockaddr_ptr, 0, "sockaddr.family")
            .map_err(err)?;
        compiler.builder().build_store(family_field, i16_type.const_int(2, false)).map_err(err)?; // AF_INET

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

        // host resolve: "localhost" -> "127.0.0.1", otherwise A.B.C.D
        let localhost_lit = compiler
            .builder()
            .build_global_string_ptr("localhost", "tcp_connect.localhost.lit")
            .map_err(err)?;
        let loopback_lit = compiler
            .builder()
            .build_global_string_ptr("127.0.0.1", "tcp_connect.loopback.lit")
            .map_err(err)?;

        let strcmp_fn = compiler.libc().strcmp_fn;
        let cmp = compiler
            .builder()
            .build_call(strcmp_fn, &[host_ptr.into(), localhost_lit.as_pointer_value().into()], "host.cmp")
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("strcmp should return a value")
            .into_int_value();

        let is_localhost = compiler
            .builder()
            .build_int_compare(inkwell::IntPredicate::EQ, cmp, i32_type.const_zero(), "is.localhost")
            .map_err(err)?;

        let resolved_host_ptr = compiler
            .builder()
            .build_select(is_localhost, loopback_lit.as_pointer_value(), host_ptr, "resolved.host")
            .map_err(err)?
            .into_pointer_value();

        let inet_addr_fn = compiler.libc().inet_addr_fn;
        let addr_be = compiler
            .builder()
            .build_call(inet_addr_fn, &[resolved_host_ptr.into()], "inet_addr.call")
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("inet_addr should return a value")
            .into_int_value();

        let addr_field = compiler
            .builder()
            .build_struct_gep(sockaddr_type, sockaddr_ptr, 2, "sockaddr.addr")
            .map_err(err)?;
        compiler.builder().build_store(addr_field, addr_be).map_err(err)?;

        let zero_field = compiler
            .builder()
            .build_struct_gep(sockaddr_type, sockaddr_ptr, 3, "sockaddr.zero")
            .map_err(err)?;
        compiler
            .builder()
            .build_store(zero_field, i8_type.array_type(8).const_zero())
            .map_err(err)?;

        // connect(fd, &sockaddr, 16)
        let connect_fn = compiler.libc().connect_fn;
        let connect_result = compiler
            .builder()
            .build_call(
                connect_fn,
                &[fd.into(), sockaddr_ptr.into(), i32_type.const_int(16, false).into()],
                "connect.call",
            )
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("connect should return a value")
            .into_int_value();

        let is_error = compiler
            .builder()
            .build_int_compare(inkwell::IntPredicate::SLT, connect_result, i32_type.const_zero(), "connect.failed")
            .map_err(err)?;

        let current_block = compiler.builder().get_insert_block().ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("tcp_connect: no current basic block"),
                span,
            )) as Box<dyn IError>
        })?;

        let current_fn = current_block.get_parent().ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("tcp_connect: no parent function for current block"),
                span,
            )) as Box<dyn IError>
        })?;

        let close_block = compiler.context().append_basic_block(current_fn, "tcp_connect.close_on_error");

        let free_block = compiler.context().append_basic_block(current_fn, "tcp_connect.free_host");

        let continue_block = compiler.context().append_basic_block(current_fn, "tcp_connect.continue");

        compiler
            .builder()
            .build_conditional_branch(is_error, close_block, free_block)
            .map_err(err)?;

        // connect() failed -> close socket
        compiler.builder().position_at_end(close_block);

        let close_fn = compiler.libc().close_fn;

        compiler
            .builder()
            .build_call(close_fn, &[fd.into()], "tcp_connect.close_on_error.call")
            .map_err(err)?;

        compiler.builder().build_unconditional_branch(free_block).map_err(err)?;

        // Both success and error paths arrive here.
        // host_ptr is a temporary string copy created because
        // tcp_connect takes its string argument by value.
        compiler.builder().position_at_end(free_block);

        let free_fn = compiler.libc().free_fn;

        compiler
            .builder()
            .build_call(free_fn, &[host_ptr.into()], "tcp_connect.free_host")
            .map_err(err)?;

        compiler.builder().build_unconditional_branch(continue_block).map_err(err)?;

        compiler.builder().position_at_end(continue_block);

        let fd_i64 = compiler.builder().build_int_z_extend(fd, i64_type, "fd.i64").map_err(err)?;

        let neg_one = i64_type.const_int(u64::MAX, true);

        let result = compiler
            .builder()
            .build_select(is_error, neg_one, fd_i64, "connect.result")
            .map_err(err)?
            .into_int_value();

        compiler.set_last_value(LlvmValue::I64(result));

        Ok(())
    };

    StdFunction {
        params,
        passed_by: vec![PassedBy::Value, PassedBy::Value],
        execute,
        return_type: Type::I64,
        type_check: None,
        compile,
    }
}
