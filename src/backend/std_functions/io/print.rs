use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    backend::{
        interpreter::Value,
        std_functions::std_functions::{build_usage_error, LlvmCompileFn, StdFunction},
    },
    common::{
        errors::{CompilerError, ErrorSeverity, IError, StdFunctionError},
        span::Span,
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::PassedBy,
};

pub fn print() -> StdFunction {
    let params = vec![Type::Str];
    let execute = |params: &Vec<Rc<RefCell<Value>>>, span: Span| -> Result<Option<Value>, StdFunctionError> {
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
                _ => Err(build_usage_error(fn_name, expected_types, actual_types, span)),
            }
        } else {
            Err(build_usage_error(fn_name, expected_types, actual_types, span))
        }
    };

    let compile: LlvmCompileFn = |compiler, arguments, span| {
        let err = |err: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), span)) as Box<dyn IError>;

        let arg = arguments.get(0).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'print' expects exactly one argument."),
                span,
            )) as Box<dyn IError>
        })?;

        compiler.visit_expression(&arg.value.value)?;
        let text_value = compiler.read_last_value()?;

        let printf_fn = compiler.libc().printf_fn;

        let format_str = compiler.builder().build_global_string_ptr("%s", "fmt").map_err(err)?;

        compiler
            .builder()
            .build_call(
                printf_fn,
                &[format_str.as_pointer_value().into(), text_value.as_basic_value_enum().into()],
                "printf_call",
            )
            .map_err(err)?;

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
