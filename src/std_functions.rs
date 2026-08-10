use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    println,
    rc::Rc,
    sync::Mutex,
    thread, time, vec,
};

use crate::{
    ast::{PassedBy, Type},
    errors::{ErrorSeverity, StdFunctionError},
    value::Value,
};

#[derive(Debug, Clone)]
pub struct StdFunction {
    pub params: Vec<Type>,
    pub passed_by: Vec<PassedBy>,
    pub execute: fn(&Vec<Rc<RefCell<Value>>>) -> Result<Option<Value>, StdFunctionError>,
    pub return_type: Type,
    pub type_check: Option<fn(&[Type]) -> Result<Type, String>>,
}

impl PartialEq for StdFunction {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params
    }
}

fn format_types(types: &[Type]) -> String {
    types.iter().map(|t| format!("{:?}", t)).collect::<Vec<String>>().join(", ")
}

fn build_usage_error(fn_name: &str, expected_types: Vec<Type>, actual_types: Vec<Type>) -> StdFunctionError {
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
    fn print() -> Self {
        let params = vec![Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "write_file";
            let expected_types = vec![Type::Str];
            let mut actual_types: Vec<Type> = vec![];
            if let Some(value) = params.get(0) {
                actual_types.push(value.borrow().to_type());
                let value = value.borrow();
                match &*value {
                    Value::String(text) => {
                        print!("{}", text);
                        Ok(None)
                    }
                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        StdFunction {
            params,
            execute,
            passed_by: vec![PassedBy::Value],
            return_type: Type::Void,
            type_check: None,
        }
    }

    fn println() -> Self {
        let params = vec![Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "println";
            let expected_types = vec![Type::Str];
            let mut actual_types: Vec<Type> = vec![];
            if let Some(value) = params.get(0) {
                actual_types.push(value.borrow().to_type());
                let value = value.borrow();
                match &*value {
                    Value::String(text) => {
                        println!("{}", text);
                        Ok(None)
                    }
                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        StdFunction {
            params,
            execute,
            passed_by: vec![PassedBy::Value],
            return_type: Type::Void,
            type_check: None,
        }
    }

    fn input() -> Self {
        let params = vec![Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
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
                            Err(_) => Err(StdFunctionError::new(ErrorSeverity::HIGH, String::from("Failed to read input."))),
                        }
                    }
                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        StdFunction {
            params,
            execute,
            passed_by: vec![PassedBy::Value],
            return_type: Type::Str,
            type_check: None,
        }
    }

    fn read_file() -> Self {
        let params = vec![Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "read_file";
            let expected_types = vec![Type::Str];
            let mut actual_types: Vec<Type> = vec![];
            if let Some(filepath) = params.get(0) {
                actual_types.push(filepath.borrow().to_type());
                let filepath = filepath.borrow();
                match &*filepath {
                    Value::String(path) => match fs::read_to_string(path) {
                        Ok(content) => Ok(Some(Value::String(content))),
                        Err(_) => Err(StdFunctionError::new(ErrorSeverity::HIGH, String::from("Failed to read file."))),
                    },
                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        StdFunction {
            params,
            execute,
            passed_by: vec![PassedBy::Value],
            return_type: Type::Str,
            type_check: None,
        }
    }

    fn write_file() -> Self {
        let params = vec![Type::Str, Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "write_file";
            let expected_types = vec![Type::Str, Type::Str];
            let mut actual_types: Vec<Type> = vec![];
            if let Some(filepath) = params.get(0) {
                actual_types.push(filepath.borrow().to_type());
                if let Some(content) = params.get(1) {
                    actual_types.push(content.borrow().to_type());
                    let filepath = filepath.borrow();
                    let content = content.borrow();
                    match &*filepath {
                        Value::String(path) => match &*content {
                            Value::String(con) => match fs::write(path, con) {
                                Ok(_) => Ok(None),
                                Err(_) => Err(StdFunctionError::new(ErrorSeverity::HIGH, String::from("Failed to write file."))),
                            },
                            _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                        },
                        _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                    }
                } else {
                    Err(build_usage_error(fn_name, expected_types, actual_types))
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        StdFunction {
            params,
            execute,
            passed_by: vec![PassedBy::Value, PassedBy::Value],
            return_type: Type::Void,
            type_check: None,
        }
    }

    fn append_file() -> Self {
        let params = vec![Type::Str, Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "append_file";
            let expected_types = vec![Type::Str, Type::Str];
            let mut actual_types: Vec<Type> = vec![];
            if let Some(filepath) = params.get(0) {
                actual_types.push(filepath.borrow().to_type());
                if let Some(content) = params.get(1) {
                    actual_types.push(content.borrow().to_type());
                    let filepath = filepath.borrow();
                    let content = content.borrow();
                    match &*filepath {
                        Value::String(path) => match &*content {
                            Value::String(con) => match OpenOptions::new().append(true).create(true).open(path) {
                                Ok(mut file) => match write!(file, "{}", con) {
                                    Ok(_) => Ok(None),
                                    Err(_) => Err(StdFunctionError::new(ErrorSeverity::HIGH, String::from("Failed to append to file."))),
                                },
                                Err(_) => Err(StdFunctionError::new(ErrorSeverity::HIGH, String::from("Failed to append to file."))),
                            },
                            _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                        },
                        _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                    }
                } else {
                    Err(build_usage_error(fn_name, expected_types, actual_types))
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        StdFunction {
            params,
            execute,
            passed_by: vec![PassedBy::Value, PassedBy::Value],
            return_type: Type::Void,
            type_check: None,
        }
    }

    fn delete_file() -> Self {
        let params = vec![Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "delete_file";
            let expected_types = vec![Type::Str];
            let mut actual_types: Vec<Type> = vec![];
            if let Some(filepath) = params.get(0) {
                actual_types.push(filepath.borrow().to_type());
                let filepath = filepath.borrow();
                match &*filepath {
                    Value::String(path) => match fs::remove_file(path) {
                        Ok(_) => Ok(None),
                        Err(_) => Err(StdFunctionError::new(ErrorSeverity::HIGH, String::from("Failed to delete file."))),
                    },
                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        StdFunction {
            params,
            execute,
            passed_by: vec![PassedBy::Value],
            return_type: Type::Void,
            type_check: None,
        }
    }

    fn exists_file() -> Self {
        let params = vec![Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "exists_file";
            let expected_types = vec![Type::Str];
            let mut actual_types: Vec<Type> = vec![];
            if let Some(filepath) = params.get(0) {
                actual_types.push(filepath.borrow().to_type());
                let filepath = filepath.borrow();
                match &*filepath {
                    Value::String(path) => {
                        let exists = Path::new(path).exists();
                        return Ok(Some(Value::Bool(exists)));
                    }
                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };

        StdFunction {
            params,
            execute,
            passed_by: vec![PassedBy::Value],
            return_type: Type::Bool,
            type_check: None,
        }
    }

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
            [other] => Err(format!("vector_size expected a vector, but got '{:?}'.", other)),
            _ => Err(String::from("vector_size expects exactly 1 argument.")),
        };

        StdFunction {
            params,
            execute,
            passed_by: vec![PassedBy::Value],
            return_type: Type::Str,
            type_check: Some(type_check),
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

                                if !inner.accepts(&value_ref) {
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

        StdFunction {
            params,
            passed_by: vec![PassedBy::Reference, PassedBy::Value],
            execute,
            return_type: Type::Void,
            type_check: Some(type_check),
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

        StdFunction {
            params,
            passed_by: vec![PassedBy::Reference],
            execute,
            return_type: Type::I64,
            type_check: Some(type_check),
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

        StdFunction {
            params,
            passed_by: vec![PassedBy::Value],
            execute,
            return_type: Type::Void,
            type_check: None,
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

        StdFunction {
            params,
            passed_by: vec![PassedBy::Value],
            execute,
            return_type: Type::I64,
            type_check: None,
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

        StdFunction {
            params,
            passed_by: vec![PassedBy::Value],
            execute,
            return_type: Type::I64,
            type_check: None,
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

        StdFunction {
            params,
            passed_by: vec![PassedBy::Value],
            execute,
            return_type: Type::Str,
            type_check: None,
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

        StdFunction {
            params,
            passed_by: vec![PassedBy::Value, PassedBy::Value],
            execute,
            return_type: Type::Void,
            type_check: None,
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

        StdFunction {
            params,
            passed_by: vec![PassedBy::Value],
            execute,
            return_type: Type::Void,
            type_check: None,
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

        StdFunction {
            params,
            passed_by: vec![PassedBy::Value],
            execute,
            return_type: Type::I64,
            type_check: None,
        }
    }
}

pub fn get_std_functions() -> HashMap<String, StdFunction> {
    let mut std_functions: HashMap<String, StdFunction> = HashMap::new();
    std_functions.insert("print".to_owned(), StdFunction::print());
    std_functions.insert("println".to_owned(), StdFunction::println());
    std_functions.insert("input".to_owned(), StdFunction::input());
    std_functions.insert("read_file".to_owned(), StdFunction::read_file());
    std_functions.insert("write_file".to_owned(), StdFunction::write_file());
    std_functions.insert("append_file".to_owned(), StdFunction::append_file());
    std_functions.insert("delete_file".to_owned(), StdFunction::delete_file());
    std_functions.insert("exists_file".to_owned(), StdFunction::exists_file());
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
