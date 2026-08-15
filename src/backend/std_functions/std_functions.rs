use std::{
    cell::RefCell,
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    rc::Rc,
    sync::Mutex,
    thread, time, vec,
};

use inkwell::AddressSpace;

use crate::{
    backend::{
        interpreter::Value,
        llvm::{compiler::Compiler, LlvmValue},
        std_functions::{
            files::{append_file::append_file, delete_file::delete_file, exists_file::exists_file, read_file::read_file, write_file::write_file},
            io::{input::input, print::print, println::println},
        },
        type_utils::type_accepts_value,
    },
    common::{
        errors::{CompilerError, ErrorSeverity, IError, StdFunctionError},
        position::Position,
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::{Argument, Expression, Node, PassedBy},
};

pub type LlvmCompileFn = for<'a, 'ctx> fn(&mut Compiler<'a, 'ctx>, &'a Vec<Box<Node<Argument>>>, Position) -> Result<(), Box<dyn IError>>;

#[derive(Debug, Clone)]
pub struct StdFunction {
    pub params: Vec<Type>,
    pub passed_by: Vec<PassedBy>,
    pub execute: fn(&Vec<Rc<RefCell<Value>>>) -> Result<Option<Value>, StdFunctionError>,
    pub return_type: Type,
    pub type_check: Option<fn(&[Type]) -> Result<Type, String>>,
    pub compile: LlvmCompileFn,
}

impl PartialEq for StdFunction {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params
    }
}

pub fn format_types(types: &[Type]) -> String {
    types.iter().map(|t| format!("{:?}", t)).collect::<Vec<String>>().join(", ")
}

pub fn build_usage_error(fn_name: &str, expected_types: Vec<Type>, actual_types: Vec<Type>) -> StdFunctionError {
    let message = format!(
        "\nInvalid usage of built-in function '{}'.\nExpected signature: {}({})\nProvided types: {}({})",
        fn_name,
        fn_name,
        format_types(&expected_types),
        fn_name,
        format_types(&actual_types)
    );
    StdFunctionError::new(ErrorSeverity::HIGH, message)
}

fn stringify_value(value: &Value) -> String {
    match value {
        Value::I64(v) => v.to_string(),
        Value::F64(v) => v.to_string(),
        Value::String(v) => format!("\"{}\"", v),
        Value::Bool(v) => v.to_string(),
        Value::Vector { values, .. } => {
            let values = values.borrow().iter().map(|v| stringify_value(&v.borrow())).collect::<Vec<String>>();

            return format!("[{}]", values.join(", "));
        }
    }
}

static LISTENERS: Mutex<Option<HashMap<i64, TcpListener>>> = Mutex::new(None);
static STREAMS: Mutex<Option<HashMap<i64, TcpStream>>> = Mutex::new(None);
static NEXT_HANDLE: Mutex<i64> = Mutex::new(0);

fn next_handle() -> i64 {
    let mut h = NEXT_HANDLE.lock().unwrap();
    *h += 1;
    *h
}

impl StdFunction {
    fn vector_stringify() -> Self {
        let params = vec![Type::Vector(Box::new(Type::Void))];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "vector_stringify";
            let expected_types = vec![Type::Vector(Box::new(Type::Void))];
            let mut actual_types: Vec<Type> = vec![];

            if let Some(vector) = params.get(0) {
                actual_types.push(vector.borrow().to_type());

                let vector = vector.borrow();

                match &*vector {
                    Value::Vector { .. } => {
                        return Ok(Some(Value::String(stringify_value(&vector))));
                    }
                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        let type_check: fn(&[Type]) -> Result<Type, String> = |arg_types: &[Type]| match arg_types {
            [Type::Vector(_)] => Ok(Type::Str),
            [other] => Err(format!("vector_stringify expected a vector, but got '{:?}'.", other)),
            _ => Err(String::from("vector_stringify expects exactly 1 argument.")),
        };

        let compile: LlvmCompileFn = |compiler, arguments, position| {
            let vector_arg = arguments.get(0).ok_or_else(|| {
                Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    String::from("'vector_stringify' expects exactly one argument."),
                    position,
                )) as Box<dyn IError>
            })?;

            compiler.visit_expression(&vector_arg.value.value)?;
            let vector_value = compiler.read_last_value()?;

            let (vector_ptr, inner_type) = match vector_value {
                LlvmValue::Vector(ptr, inner) => (ptr, *inner),
                other => {
                    return Err(Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("'vector_stringify' expects a vector, got '{:?}'.", other.to_type()),
                        position,
                    )))
                }
            };

            let result = compiler.build_vector_to_string(vector_ptr, &inner_type, position)?;
            compiler.set_last_value(LlvmValue::Str(result));

            Ok(())
        };

        StdFunction {
            params,
            execute,
            passed_by: vec![PassedBy::Value],
            return_type: Type::Str,
            type_check: Some(type_check),
            compile,
        }
    }

    fn vector_push() -> Self {
        let params = vec![Type::Vector(Box::new(Type::Void)), Type::Void];

        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "vector_push";
            let expected_types = vec![Type::Vector(Box::new(Type::Void)), Type::Void];

            let mut actual_types: Vec<Type> = vec![];

            if let (Some(vector), Some(value)) = (params.get(0), params.get(1)) {
                actual_types.push(vector.borrow().to_type());
                actual_types.push(value.borrow().to_type());

                let mut vector = vector.borrow_mut();

                match &mut *vector {
                    Value::Vector { kind, values } => {
                        if let Type::Vector(inner) = kind.as_ref() {
                            {
                                let value_ref = value.borrow();

                                if !type_accepts_value(inner, &value_ref) {
                                    return Err(build_usage_error(fn_name, expected_types, actual_types));
                                }
                            }

                            values.borrow_mut().push(Rc::clone(value));

                            return Ok(None);
                        }
                        Err(build_usage_error(fn_name, expected_types, actual_types))
                    }

                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        let type_check: fn(&[Type]) -> Result<Type, String> = |arg_types: &[Type]| match arg_types {
            [Type::Vector(inner), value_type] => {
                if inner.is_compatible(value_type) {
                    Ok(Type::Void)
                } else {
                    Err(format!("vector_push expected element of type '{:?}', but got '{:?}'.", inner, value_type))
                }
            }
            [other, _] => Err(format!("vector_push expected a vector as first argument, but got '{:?}'.", other)),
            _ => Err(String::from("vector_push expects exactly 2 arguments.")),
        };

        let compile: LlvmCompileFn = |compiler, arguments, position| {
            let err =
                |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

            let err_arity = || {
                Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    String::from("'vector_push' expects exactly 2 arguments."),
                    position,
                )) as Box<dyn IError>
            };

            let vector_arg = arguments.get(0).ok_or_else(err_arity)?;
            let value_arg = arguments.get(1).ok_or_else(err_arity)?;

            let variable_name = match &vector_arg.value.value.value {
                Expression::Variable(name) => name.clone(),
                other => {
                    return Err(Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("'vector_push' expects a variable as its first argument, got '{:?}'.", other),
                        position,
                    )))
                }
            };

            let (var_slot_ptr, var_type) = compiler.get_variable(&variable_name)?;
            let inner_type = match var_type {
                Type::Vector(inner) => *inner,
                other => {
                    return Err(Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("'vector_push' expects a vector, got '{:?}'.", other),
                        position,
                    )))
                }
            };

            compiler.visit_expression(&value_arg.value.value)?;
            let pushed_value = compiler.read_last_value()?;

            if pushed_value.to_type() != inner_type {
                return Err(Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    format!(
                        "Vector element type mismatch: expected '{:?}', got '{:?}'.",
                        inner_type,
                        pushed_value.to_type()
                    ),
                    position,
                )));
            }

            let context = compiler.context();
            let ptr_type = context.ptr_type(AddressSpace::default());
            let i64_type = context.i64_type();
            let struct_type = LlvmValue::vector_struct_type(context);

            let function = compiler
                .builder()
                .get_insert_block()
                .expect("builder should be positioned inside a function")
                .get_parent()
                .expect("basic block should belong to a function");

            let struct_ptr = compiler
                .builder()
                .build_load(ptr_type, var_slot_ptr, "vector.ptr")
                .map_err(err)?
                .into_pointer_value();

            let data_field = compiler
                .builder()
                .build_struct_gep(struct_type, struct_ptr, 0, "vector.data")
                .map_err(err)?;
            let length_field = compiler
                .builder()
                .build_struct_gep(struct_type, struct_ptr, 1, "vector.length")
                .map_err(err)?;
            let capacity_field = compiler
                .builder()
                .build_struct_gep(struct_type, struct_ptr, 2, "vector.capacity")
                .map_err(err)?;

            let old_length = compiler
                .builder()
                .build_load(i64_type, length_field, "vector.length.old")
                .map_err(err)?
                .into_int_value();
            let old_capacity = compiler
                .builder()
                .build_load(i64_type, capacity_field, "vector.capacity.old")
                .map_err(err)?
                .into_int_value();

            let element_llvm_type = LlvmValue::type_to_basic_type_enum(&inner_type, context).ok_or_else(|| {
                Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    format!("Compiling vectors of type '{:?}' is not yet supported.", inner_type),
                    position,
                )) as Box<dyn IError>
            })?;
            let element_size = LlvmValue::element_byte_size(&inner_type, i64_type)?;

            let needs_grow = compiler
                .builder()
                .build_int_compare(inkwell::IntPredicate::SGE, old_length, old_capacity, "needs.grow")
                .map_err(err)?;

            let grow_block = context.append_basic_block(function, "push.grow");
            let merge_block = context.append_basic_block(function, "push.merge");

            compiler
                .builder()
                .build_conditional_branch(needs_grow, grow_block, merge_block)
                .map_err(err)?;

            compiler.builder().position_at_end(grow_block);
            let old_data = compiler
                .builder()
                .build_load(ptr_type, data_field, "vector.data.old")
                .map_err(err)?
                .into_pointer_value();

            let is_zero = compiler
                .builder()
                .build_int_compare(inkwell::IntPredicate::EQ, old_capacity, i64_type.const_int(0, false), "cap.is_zero")
                .map_err(err)?;
            let doubled = compiler
                .builder()
                .build_int_mul(old_capacity, i64_type.const_int(2, false), "cap.doubled")
                .map_err(err)?;
            let new_capacity = compiler
                .builder()
                .build_select(is_zero, i64_type.const_int(1, false), doubled, "cap.new")
                .map_err(err)?
                .into_int_value();

            let new_bytes = compiler
                .builder()
                .build_int_mul(new_capacity, element_size, "vector.bytes.new")
                .map_err(err)?;
            let new_data = compiler
                .builder()
                .build_call(compiler.libc().realloc_fn, &[old_data.into(), new_bytes.into()], "vector.realloc")
                .map_err(err)?
                .try_as_basic_value()
                .basic()
                .expect("realloc should return a value")
                .into_pointer_value();

            compiler.builder().build_store(data_field, new_data).map_err(err)?;
            compiler.builder().build_store(capacity_field, new_capacity).map_err(err)?;
            compiler.builder().build_unconditional_branch(merge_block).map_err(err)?;

            compiler.builder().position_at_end(merge_block);

            let current_data = compiler
                .builder()
                .build_load(ptr_type, data_field, "vector.data.current")
                .map_err(err)?
                .into_pointer_value();

            let elem_ptr = unsafe {
                compiler
                    .builder()
                    .build_gep(element_llvm_type, current_data, &[old_length], "vector.push.elem")
                    .map_err(err)?
            };
            compiler
                .builder()
                .build_store(elem_ptr, pushed_value.as_basic_value_enum())
                .map_err(err)?;

            let new_length = compiler
                .builder()
                .build_int_add(old_length, i64_type.const_int(1, false), "vector.length.new")
                .map_err(err)?;
            compiler.builder().build_store(length_field, new_length).map_err(err)?;

            Ok(())
        };

        StdFunction {
            params,
            passed_by: vec![PassedBy::Reference, PassedBy::Value],
            execute,
            return_type: Type::Void,
            type_check: Some(type_check),
            compile,
        }
    }

    fn vector_size() -> Self {
        let params = vec![Type::Vector(Box::new(Type::Void))];

        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "vector_size";
            let expected_types = vec![Type::Vector(Box::new(Type::Void))];

            let mut actual_types: Vec<Type> = vec![];

            if let Some(vector) = params.get(0) {
                actual_types.push(vector.borrow().to_type());

                let vector = vector.borrow();

                match &*vector {
                    Value::Vector { values, .. } => {
                        let borrowed = values.borrow().clone();
                        Ok(Some(Value::I64(borrowed.len() as i64)))
                    }

                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        let type_check: fn(&[Type]) -> Result<Type, String> = |arg_types: &[Type]| match arg_types {
            [Type::Vector(_)] => Ok(Type::I64),
            [other] => Err(format!("vector_size expected a vector, but got '{:?}'.", other)),
            _ => Err(String::from("vector_size expects exactly 1 argument.")),
        };

        let compile: LlvmCompileFn = |compiler, arguments, position| {
            let err =
                |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

            let vector_arg = arguments.get(0).ok_or_else(|| {
                Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    String::from("'vector_size' expects exactly one argument."),
                    position,
                )) as Box<dyn IError>
            })?;

            compiler.visit_expression(&vector_arg.value.value)?;

            let vector_value = compiler.read_last_value()?;

            let vector_ptr = match vector_value {
                LlvmValue::Vector(ptr, _) => ptr,

                other => {
                    return Err(Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("'vector_size' expects a vector, got '{:?}'.", other.to_type()),
                        position,
                    )))
                }
            };

            let context = compiler.context();
            let i64_type = context.i64_type();
            let struct_type = LlvmValue::vector_struct_type(context);

            let length_field = compiler
                .builder()
                .build_struct_gep(struct_type, vector_ptr, 1, "vector.length")
                .map_err(err)?;

            let length = compiler
                .builder()
                .build_load(i64_type, length_field, "vector.size")
                .map_err(err)?
                .into_int_value();

            compiler.set_last_value(LlvmValue::I64(length));

            Ok(())
        };

        StdFunction {
            params,
            passed_by: vec![PassedBy::Reference],
            execute,
            return_type: Type::I64,
            type_check: Some(type_check),
            compile,
        }
    }

    fn sleep_ms() -> Self {
        let params = vec![Type::I64];

        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "vector_size";
            let expected_types = vec![Type::I64];

            let mut actual_types: Vec<Type> = vec![];

            if let Some(millis) = params.get(0) {
                actual_types.push(millis.borrow().to_type());

                let millis = millis.borrow();

                match &*millis {
                    Value::I64(ms) => {
                        let duration = time::Duration::from_millis(*ms as u64);
                        thread::sleep(duration);
                        Ok(None)
                    }

                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        let compile: LlvmCompileFn = |compiler, arguments, position| {
            let err =
                |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

            let arg = arguments.get(0).ok_or_else(|| {
                Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    String::from("'sleep_ms' expects exactly one argument."),
                    position,
                )) as Box<dyn IError>
            })?;

            compiler.visit_expression(&arg.value.value)?;
            let ms_value = compiler.read_last_value()?.into_i64_value(position)?;

            let context = compiler.context();
            let i32_type = context.i32_type();

            let i64_type = context.i64_type();
            let micros_i64 = compiler
                .builder()
                .build_int_mul(ms_value, i64_type.const_int(1000, false), "sleep.micros")
                .map_err(err)?;
            let micros_i32 = compiler
                .builder()
                .build_int_truncate(micros_i64, i32_type, "sleep.micros.i32")
                .map_err(err)?;

            let usleep_fn = compiler.libc().usleep_fn;
            compiler
                .builder()
                .build_call(usleep_fn, &[micros_i32.into()], "usleep.call")
                .map_err(err)?;

            Ok(())
        };

        StdFunction {
            params,
            passed_by: vec![PassedBy::Value],
            execute,
            return_type: Type::Void,
            type_check: None,
            compile,
        }
    }

    fn tcp_listen() -> Self {
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
            let err =
                |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

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

    fn tcp_accept() -> Self {
        let params = vec![Type::I64];

        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "tcp_accept";
            let expected_types = vec![Type::I64];

            let mut actual_types: Vec<Type> = vec![];

            if let Some(handle) = params.get(0) {
                actual_types.push(handle.borrow().to_type());

                let handle = handle.borrow();

                match &*handle {
                    Value::I64(listener_handle) => {
                        let listeners = LISTENERS.lock().unwrap();
                        let listener = listeners
                            .as_ref()
                            .and_then(|m| m.get(listener_handle))
                            .ok_or_else(|| StdFunctionError::new(ErrorSeverity::HIGH, format!("Invalid listener handle {}", listener_handle)))?;

                        let (stream, _addr) = listener
                            .accept()
                            .map_err(|e| StdFunctionError::new(ErrorSeverity::HIGH, format!("Accept failed: {}", e)))?;

                        drop(listeners);

                        let new_handle = next_handle();
                        STREAMS.lock().unwrap().get_or_insert_with(HashMap::new).insert(new_handle, stream);

                        Ok(Some(Value::I64(new_handle)))
                    }
                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        let compile: LlvmCompileFn = |compiler, arguments, position| {
            let err =
                |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

            let arg = arguments.get(0).ok_or_else(|| {
                Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    String::from("'tcp_accept' expects exactly one argument."),
                    position,
                )) as Box<dyn IError>
            })?;

            compiler.visit_expression(&arg.value.value)?;
            let listener_fd = compiler.read_last_value()?.into_i64_value(position)?;

            let context = compiler.context();
            let i32_type = context.i32_type();
            let ptr_type = context.ptr_type(AddressSpace::default());

            let listener_fd_i32 = compiler.builder().build_int_truncate(listener_fd, i32_type, "fd.i32").map_err(err)?;

            let accept_fn = compiler.libc().accept_fn;
            let client_fd = compiler
                .builder()
                .build_call(
                    accept_fn,
                    &[listener_fd_i32.into(), ptr_type.const_null().into(), ptr_type.const_null().into()],
                    "accept.call",
                )
                .map_err(err)?
                .try_as_basic_value()
                .basic()
                .expect("accept should return a value")
                .into_int_value();

            let client_fd_i64 = compiler
                .builder()
                .build_int_z_extend(client_fd, context.i64_type(), "fd.i64")
                .map_err(err)?;
            compiler.set_last_value(LlvmValue::I64(client_fd_i64));
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

    fn tcp_read() -> Self {
        let params = vec![Type::I64];

        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "tcp_read";
            let expected_types = vec![Type::I64];

            let mut actual_types: Vec<Type> = vec![];

            if let Some(handle) = params.get(0) {
                actual_types.push(handle.borrow().to_type());

                let handle = handle.borrow();

                match &*handle {
                    Value::I64(stream_handle) => {
                        let mut streams = STREAMS.lock().unwrap();
                        let stream = streams
                            .as_mut()
                            .and_then(|m| m.get_mut(stream_handle))
                            .ok_or_else(|| StdFunctionError::new(ErrorSeverity::HIGH, format!("Invalid stream handle {}", stream_handle)))?;

                        let mut buffer = [0u8; 4096];
                        let n = stream
                            .read(&mut buffer)
                            .map_err(|e| StdFunctionError::new(ErrorSeverity::HIGH, format!("Read failed: {}", e)))?;

                        Ok(Some(Value::String(String::from_utf8_lossy(&buffer[..n]).to_string())))
                    }
                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        let compile: LlvmCompileFn = |compiler, arguments, position| {
            let err =
                |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

            let arg = arguments.get(0).ok_or_else(|| {
                Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    String::from("'tcp_read' expects exactly one argument."),
                    position,
                )) as Box<dyn IError>
            })?;

            compiler.visit_expression(&arg.value.value)?;
            let fd = compiler.read_last_value()?.into_i64_value(position)?;

            let context = compiler.context();
            let i32_type = context.i32_type();
            let i64_type = context.i64_type();
            let buf_size = 4096u64;

            let fd_i32 = compiler.builder().build_int_truncate(fd, i32_type, "fd.i32").map_err(err)?;

            let malloc_fn = compiler.libc().malloc_fn;
            let buf = compiler
                .builder()
                .build_call(malloc_fn, &[i64_type.const_int(buf_size, false).into()], "recv.buf")
                .map_err(err)?
                .try_as_basic_value()
                .basic()
                .expect("malloc should return a value")
                .into_pointer_value();

            let recv_fn = compiler.libc().recv_fn;
            let n = compiler
                .builder()
                .build_call(
                    recv_fn,
                    &[
                        fd_i32.into(),
                        buf.into(),
                        i64_type.const_int(buf_size - 1, false).into(),
                        i32_type.const_int(0, false).into(),
                    ],
                    "recv.call",
                )
                .map_err(err)?
                .try_as_basic_value()
                .basic()
                .expect("recv should return a value")
                .into_int_value();

            let end_ptr = unsafe { compiler.builder().build_gep(context.i8_type(), buf, &[n], "recv.end").map_err(err)? };
            compiler
                .builder()
                .build_store(end_ptr, context.i8_type().const_int(0, false))
                .map_err(err)?;

            compiler.set_last_value(LlvmValue::Str(buf));
            Ok(())
        };

        StdFunction {
            params,
            passed_by: vec![PassedBy::Value],
            execute,
            return_type: Type::Str,
            type_check: None,
            compile,
        }
    }

    fn tcp_write() -> Self {
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
            let err =
                |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

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

    fn tcp_close() -> Self {
        let params = vec![Type::I64];

        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "tcp_close";
            let expected_types = vec![Type::I64];

            let mut actual_types: Vec<Type> = vec![];

            if let Some(handle) = params.get(0) {
                actual_types.push(handle.borrow().to_type());

                let handle = handle.borrow();

                match &*handle {
                    Value::I64(h) => {
                        STREAMS.lock().unwrap().as_mut().map(|m| m.remove(h));
                        LISTENERS.lock().unwrap().as_mut().map(|m| m.remove(h));
                        Ok(None)
                    }
                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        let compile: LlvmCompileFn = |compiler, arguments, position| {
            let err =
                |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

            let arg = arguments.get(0).ok_or_else(|| {
                Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    String::from("'tcp_close' expects exactly one argument."),
                    position,
                )) as Box<dyn IError>
            })?;

            compiler.visit_expression(&arg.value.value)?;
            let fd = compiler.read_last_value()?.into_i64_value(position)?;

            let context = compiler.context();
            let i32_type = context.i32_type();
            let fd_i32 = compiler.builder().build_int_truncate(fd, i32_type, "fd.i32").map_err(err)?;

            let close_fn = compiler.libc().close_fn;
            compiler.builder().build_call(close_fn, &[fd_i32.into()], "close.call").map_err(err)?;

            Ok(())
        };

        StdFunction {
            params,
            passed_by: vec![PassedBy::Value],
            execute,
            return_type: Type::Void,
            type_check: None,
            compile,
        }
    }

    fn str_len() -> Self {
        let params = vec![Type::Str];

        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "str_len";
            let expected_types = vec![Type::Str];

            let mut actual_types: Vec<Type> = vec![];

            if let Some(text) = params.get(0) {
                actual_types.push(text.borrow().to_type());

                let text = text.borrow();

                match &*text {
                    Value::String(s) => Ok(Some(Value::I64(s.chars().count() as i64))),
                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        let compile: LlvmCompileFn = |compiler, arguments, position| {
            let err =
                |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

            let arg = arguments.get(0).ok_or_else(|| {
                Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    String::from("'str_len' expects exactly one argument."),
                    position,
                )) as Box<dyn IError>
            })?;

            compiler.visit_expression(&arg.value.value)?;
            let str_value = compiler.read_last_value()?;

            let str_ptr = match str_value {
                LlvmValue::Str(ptr) => ptr,
                other => {
                    return Err(Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("'str_len' expects a string, got '{:?}'.", other.to_type()),
                        position,
                    )))
                }
            };

            let strlen_fn = compiler.libc().strlen_fn;
            let length = compiler
                .builder()
                .build_call(strlen_fn, &[str_ptr.into()], "str.len")
                .map_err(err)?
                .try_as_basic_value()
                .basic()
                .expect("strlen should return a value")
                .into_int_value();

            compiler.set_last_value(LlvmValue::I64(length));
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
}

pub fn get_std_functions() -> HashMap<String, StdFunction> {
    let mut std_functions: HashMap<String, StdFunction> = HashMap::new();
    std_functions.insert("print".to_owned(), print());
    std_functions.insert("println".to_owned(), println());
    std_functions.insert("input".to_owned(), input());
    std_functions.insert("read_file".to_owned(), read_file());
    std_functions.insert("write_file".to_owned(), write_file());
    std_functions.insert("append_file".to_owned(), append_file());
    std_functions.insert("delete_file".to_owned(), delete_file());
    std_functions.insert("exists_file".to_owned(), exists_file());
    std_functions.insert("vector_stringify".to_owned(), StdFunction::vector_stringify());
    std_functions.insert("vector_push".to_owned(), StdFunction::vector_push());
    std_functions.insert("vector_size".to_owned(), StdFunction::vector_size());
    std_functions.insert("sleep_ms".to_owned(), StdFunction::sleep_ms());
    std_functions.insert("tcp_accept".to_owned(), StdFunction::tcp_accept());
    std_functions.insert("tcp_close".to_owned(), StdFunction::tcp_close());
    std_functions.insert("tcp_listen".to_owned(), StdFunction::tcp_listen());
    std_functions.insert("tcp_read".to_owned(), StdFunction::tcp_read());
    std_functions.insert("tcp_write".to_owned(), StdFunction::tcp_write());
    std_functions.insert("str_len".to_owned(), StdFunction::str_len());
    std_functions
}
