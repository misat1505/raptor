use std::{cell::RefCell, rc::Rc, vec};

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

pub fn println() -> StdFunction {
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

    let compile: LlvmCompileFn = |compiler, arguments, position| {
        let arg = arguments.get(0).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'println' expects exactly one argument."),
                position,
            )) as Box<dyn IError>
        })?;

        compiler.visit_expression(&arg.value.value)?;
        let text_value = compiler.read_last_value()?;

        let printf_fn = compiler.libc().printf_fn;

        let format_str = compiler
            .builder()
            .build_global_string_ptr("%s\n", "fmt")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), position)) as Box<dyn IError>)?;

        compiler
            .builder()
            .build_call(
                printf_fn,
                &[format_str.as_pointer_value().into(), text_value.as_basic_value_enum().into()],
                "printf_call",
            )
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), position)) as Box<dyn IError>)?;

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
