use std::{cell::RefCell, fs, rc::Rc, vec};

use crate::{
    backend::{
        interpreter::Value,
        std_functions::std_functions::{build_usage_error, LlvmCompileFn, StdFunction},
    },
    common::{
        errors::{CompilerError, ErrorSeverity, IError, StdFunctionError},
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::PassedBy,
};

pub fn delete_file() -> StdFunction {
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

    let compile: LlvmCompileFn = |compiler, arguments, position| {
        let err = |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

        let arg = arguments.get(0).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'delete_file' expects exactly one argument."),
                position,
            )) as Box<dyn IError>
        })?;

        compiler.visit_expression(&arg.value.value)?;
        let path_ptr = compiler.read_last_value()?.into_str_value(position)?;

        let remove_fn = compiler.libc().remove_fn;
        compiler.builder().build_call(remove_fn, &[path_ptr.into()], "remove.call").map_err(err)?;

        Ok(())
    };

    StdFunction {
        params,
        execute,
        passed_by: vec![PassedBy::Value],
        return_type: Type::Void,
        type_check: None,
        compile,
    }
}
