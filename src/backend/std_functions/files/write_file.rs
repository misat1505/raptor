use std::{cell::RefCell, fs, rc::Rc, vec};

use crate::{
    backend::{
        interpreter::Value,
        std_functions::{
            files::common::compile_write_or_append,
            std_functions::{build_usage_error, LlvmCompileFn, StdFunction},
        },
    },
    common::{
        errors::{ErrorSeverity, StdFunctionError},
        types::Type,
    },
    frontend::ast::PassedBy,
};

pub fn write_file() -> StdFunction {
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

    let compile: LlvmCompileFn = |compiler, arguments, position| compile_write_or_append(compiler, arguments, "wb", position);

    StdFunction {
        params,
        execute,
        passed_by: vec![PassedBy::Value, PassedBy::Value],
        return_type: Type::Void,
        type_check: None,
        compile,
    }
}
