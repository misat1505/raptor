use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    backend::{
        interpreter::Value,
        llvm::LlvmValue,
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

pub fn str_free() -> StdFunction {
    let params = vec![Type::Str];

    let execute = |params: &Vec<Rc<RefCell<Value>>>, span: Span| -> Result<Option<Value>, StdFunctionError> {
        let fn_name = "str_free";
        let expected_types = vec![Type::Str];
        let mut actual_types: Vec<Type> = vec![];

        if let Some(text) = params.get(0) {
            actual_types.push(text.borrow().to_type());

            let text = text.borrow();

            match &*text {
                Value::String(_) => Ok(None),
                _ => Err(build_usage_error(fn_name, expected_types, actual_types, span)),
            }
        } else {
            Err(build_usage_error(fn_name, expected_types, actual_types, span))
        }
    };

    let compile: LlvmCompileFn = |compiler, arguments, span| {
        let err = |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), span)) as Box<dyn IError>;

        let arg = arguments.get(0).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'str_free' expects exactly one argument."),
                span,
            )) as Box<dyn IError>
        })?;

        compiler.visit_expression(&arg.value.value)?;

        let str_value = compiler.read_last_value()?;

        let str_ptr = match str_value {
            LlvmValue::Str(ptr) => ptr,

            other => {
                return Err(Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    format!("'str_free' expects a string, got '{:?}'.", other.to_type()),
                    span,
                )));
            }
        };

        compiler
            .builder()
            .build_call(compiler.libc().free_fn, &[str_ptr.into()], "str.free")
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
