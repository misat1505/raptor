use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
    println,
    rc::Rc,
    vec,
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
        }
    }

    fn modulo() -> Self {
        let params = vec![Type::I64, Type::I64];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            let fn_name = "modulo";
            let expected_types = vec![Type::I64, Type::I64];
            let mut actual_types: Vec<Type> = vec![];
            if let (Some(val1), Some(val2)) = (params.get(0), params.get(1)) {
                actual_types.push(val1.borrow().to_type());
                actual_types.push(val2.borrow().to_type());
                let val1 = val1.borrow();
                let val2 = val2.borrow();
                match (&*val1, &*val2) {
                    (Value::I64(val1), Value::I64(val2)) => Ok(Some(Value::I64(*val1 % *val2))),
                    _ => Err(build_usage_error(fn_name, expected_types, actual_types)),
                }
            } else {
                Err(build_usage_error(fn_name, expected_types, actual_types))
            }
        };
        StdFunction {
            params,
            execute,
            passed_by: vec![PassedBy::Value, PassedBy::Value],
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

        StdFunction {
            params,
            execute,
            passed_by: vec![PassedBy::Value],
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

        StdFunction {
            params,
            passed_by: vec![PassedBy::Reference, PassedBy::Value],
            execute,
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

        StdFunction {
            params,
            passed_by: vec![PassedBy::Reference],
            execute,
        }
    }
}

pub fn get_std_functions() -> HashMap<String, StdFunction> {
    let mut std_functions: HashMap<String, StdFunction> = HashMap::new();
    std_functions.insert("print".to_owned(), StdFunction::print());
    std_functions.insert("println".to_owned(), StdFunction::println());
    std_functions.insert("input".to_owned(), StdFunction::input());
    std_functions.insert("mod".to_owned(), StdFunction::modulo());
    std_functions.insert("read_file".to_owned(), StdFunction::read_file());
    std_functions.insert("write_file".to_owned(), StdFunction::write_file());
    std_functions.insert("append_file".to_owned(), StdFunction::append_file());
    std_functions.insert("delete_file".to_owned(), StdFunction::delete_file());
    std_functions.insert("exists_file".to_owned(), StdFunction::exists_file());
    std_functions.insert("vector_stringify".to_owned(), StdFunction::vector_stringify());
    std_functions.insert("vector_push".to_owned(), StdFunction::vector_push());
    std_functions.insert("vector_size".to_owned(), StdFunction::vector_size());
    std_functions
}
