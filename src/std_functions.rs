use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
    rc::Rc,
    writeln,
};

use crate::{
    ast::Type,
    errors::{ErrorSeverity, StdFunctionError},
    value::Value,
};

#[derive(Debug, Clone, PartialEq)]
pub struct StdFunction {
    pub params: Vec<Type>,
    pub execute: fn(&Vec<Rc<RefCell<Value>>>) -> Result<Option<Value>, StdFunctionError>,
}

impl StdFunction {
    fn print() -> Self {
        let params = vec![Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            if let Some(value) = params.get(0) {
                let value = value.borrow();
                match &*value {
                    Value::String(text) => {
                        print!("{}", text);
                        Ok(None)
                    }
                    _ => Err(StdFunctionError::new(
                        ErrorSeverity::HIGH,
                        format!(
                            "Std function 'print' expected '{:?}' as the only argument, but was given '{:?}'.",
                            Type::Str,
                            value.to_type()
                        ),
                    )),
                }
            } else {
                Err(StdFunctionError::new(
                    ErrorSeverity::HIGH,
                    String::from("Missing argument for 'print' function."),
                ))
            }
        };
        StdFunction { params, execute }
    }

    fn println() -> Self {
        let params = vec![Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            if let Some(value) = params.get(0) {
                let value = value.borrow();
                match &*value {
                    Value::String(text) => {
                        println!("{}", text);
                        Ok(None)
                    }
                    _ => Err(StdFunctionError::new(
                        ErrorSeverity::HIGH,
                        format!(
                            "Std function 'println' expected '{:?}' as the only argument, but was given '{:?}'.",
                            Type::Str,
                            value.to_type()
                        ),
                    )),
                }
            } else {
                Err(StdFunctionError::new(
                    ErrorSeverity::HIGH,
                    String::from("Missing argument for 'println' function."),
                ))
            }
        };
        StdFunction { params, execute }
    }

    fn input() -> Self {
        let params = vec![Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            if let Some(value) = params.get(0) {
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
                    _ => Err(StdFunctionError::new(
                        ErrorSeverity::HIGH,
                        format!(
                            "Std function 'input' expected '{:?}' as the only argument, but was given '{:?}'.",
                            Type::Str,
                            value.to_type()
                        ),
                    )),
                }
            } else {
                Err(StdFunctionError::new(
                    ErrorSeverity::HIGH,
                    String::from("Missing argument for 'input' function."),
                ))
            }
        };
        StdFunction { params, execute }
    }

    fn modulo() -> Self {
        let params = vec![Type::I64, Type::I64];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            if let (Some(val1), Some(val2)) = (params.get(0), params.get(1)) {
                let val1 = val1.borrow();
                let val2 = val2.borrow();
                match (&*val1, &*val2) {
                    (Value::I64(val1), Value::I64(val2)) => Ok(Some(Value::I64(*val1 % *val2))),
                    _ => Err(StdFunctionError::new(
                        ErrorSeverity::HIGH,
                        format!(
                            "Cannot perform modulo operation between values of types '{:?}' and '{:?}'.",
                            val1.to_type(),
                            val2.to_type()
                        ),
                    )),
                }
            } else {
                Err(StdFunctionError::new(
                    ErrorSeverity::HIGH,
                    String::from("Missing arguments for 'mod' function."),
                ))
            }
        };
        StdFunction { params, execute }
    }

    fn read_file() -> Self {
        let params = vec![Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            if let Some(filepath) = params.get(0) {
                let filepath = filepath.borrow();
                match &*filepath {
                    Value::String(path) => match fs::read_to_string(path) {
                        Ok(content) => Ok(Some(Value::String(content))),
                        Err(_) => Err(StdFunctionError::new(ErrorSeverity::HIGH, String::from("Failed to read file."))),
                    },
                    _ => Err(StdFunctionError::new(
                        ErrorSeverity::HIGH,
                        format!(
                            "Std function 'read_file' expected '{:?}' as the only argument, but was given '{:?}'.",
                            Type::Str,
                            filepath.to_type()
                        ),
                    )),
                }
            } else {
                Err(StdFunctionError::new(
                    ErrorSeverity::HIGH,
                    String::from("Missing argument for 'read_file' function."),
                ))
            }
        };
        StdFunction { params, execute }
    }

    fn write_file() -> Self {
        let params = vec![Type::Str, Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            if let Some(filepath) = params.get(0) {
                if let Some(content) = params.get(1) {
                    let filepath = filepath.borrow();
                    let content = content.borrow();
                    match &*filepath {
                        Value::String(path) => match &*content {
                            Value::String(con) => match fs::write(path, con) {
                                Ok(content) => Ok(None),
                                Err(_) => Err(StdFunctionError::new(ErrorSeverity::HIGH, String::from("Failed to write file."))),
                            },
                            _ => Err(StdFunctionError::new(
                                ErrorSeverity::HIGH,
                                format!(
                                    "Std function 'write_file' expected '{:?}' as the only argument, but was given '{:?}'.",
                                    Type::Str,
                                    filepath.to_type()
                                ),
                            )),
                        },
                        _ => Err(StdFunctionError::new(
                            ErrorSeverity::HIGH,
                            format!(
                                "Std function 'write_file' expected '{:?}' as the only argument, but was given '{:?}'.",
                                Type::Str,
                                filepath.to_type()
                            ),
                        )),
                    }
                } else {
                    Err(StdFunctionError::new(
                        ErrorSeverity::HIGH,
                        String::from("Missing argument for 'write_file' function."),
                    ))
                }
            } else {
                Err(StdFunctionError::new(
                    ErrorSeverity::HIGH,
                    String::from("Missing argument for 'write_file' function."),
                ))
            }
        };
        StdFunction { params, execute }
    }

    fn append_file() -> Self {
        let params = vec![Type::Str, Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            if let Some(filepath) = params.get(0) {
                if let Some(content) = params.get(1) {
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
                            _ => Err(StdFunctionError::new(
                                ErrorSeverity::HIGH,
                                format!(
                                    "Std function 'write_file' expected '{:?}' as the only argument, but was given '{:?}'.",
                                    Type::Str,
                                    filepath.to_type()
                                ),
                            )),
                        },
                        _ => Err(StdFunctionError::new(
                            ErrorSeverity::HIGH,
                            format!(
                                "Std function 'write_file' expected '{:?}' as the only argument, but was given '{:?}'.",
                                Type::Str,
                                filepath.to_type()
                            ),
                        )),
                    }
                } else {
                    Err(StdFunctionError::new(
                        ErrorSeverity::HIGH,
                        String::from("Missing argument for 'write_file' function."),
                    ))
                }
            } else {
                Err(StdFunctionError::new(
                    ErrorSeverity::HIGH,
                    String::from("Missing argument for 'write_file' function."),
                ))
            }
        };
        StdFunction { params, execute }
    }

    fn delete_file() -> Self {
        let params = vec![Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            if let Some(filepath) = params.get(0) {
                let filepath = filepath.borrow();
                match &*filepath {
                    Value::String(path) => match fs::remove_file(path) {
                        Ok(_) => Ok(None),
                        Err(_) => Err(StdFunctionError::new(ErrorSeverity::HIGH, String::from("Failed to delete file."))),
                    },
                    _ => Err(StdFunctionError::new(
                        ErrorSeverity::HIGH,
                        format!(
                            "Std function 'delete_file' expected '{:?}' as the only argument, but was given '{:?}'.",
                            Type::Str,
                            filepath.to_type()
                        ),
                    )),
                }
            } else {
                Err(StdFunctionError::new(
                    ErrorSeverity::HIGH,
                    String::from("Missing argument for 'delete_file' function."),
                ))
            }
        };
        StdFunction { params, execute }
    }

    fn exists_file() -> Self {
        let params = vec![Type::Str];
        let execute = |params: &Vec<Rc<RefCell<Value>>>| -> Result<Option<Value>, StdFunctionError> {
            if let Some(filepath) = params.get(0) {
                let filepath = filepath.borrow();
                match &*filepath {
                    Value::String(path) => {
                        let exists = Path::new(path).exists();
                        return Ok(Some(Value::Bool(exists)));
                    }
                    _ => Err(StdFunctionError::new(
                        ErrorSeverity::HIGH,
                        format!(
                            "Std function 'exists_file' expected '{:?}' as the only argument, but was given '{:?}'.",
                            Type::Str,
                            filepath.to_type()
                        ),
                    )),
                }
            } else {
                Err(StdFunctionError::new(
                    ErrorSeverity::HIGH,
                    String::from("Missing argument for 'exists_file' function."),
                ))
            }
        };
        StdFunction { params, execute }
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
    std_functions
}
