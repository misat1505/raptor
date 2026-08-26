use std::{cell::RefCell, path::Path, rc::Rc, vec};

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

pub fn exists_file() -> StdFunction {
    let params = vec![Type::Str];
    let execute = |params: &Vec<Rc<RefCell<Value>>>, span: Span| -> Result<Option<Value>, StdFunctionError> {
        let fn_name = "exists_file";
        let expected_types = vec![Type::Str];
        let mut actual_types: Vec<Type> = vec![];

        if let Some(filepath) = params.get(0) {
            actual_types.push(filepath.borrow().to_type());
            let filepath = filepath.borrow();

            match &*filepath {
                Value::String(path) => {
                    let exists = Path::new(path).exists();
                    Ok(Some(Value::Bool(exists)))
                }
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
                String::from("'exists_file' expects exactly one argument."),
                span,
            )) as Box<dyn IError>
        })?;

        compiler.visit_expression(&arg.value.value)?;
        let path_ptr = compiler.read_last_value()?.into_str_value(span)?;
        let owned_ptr = compiler.build_string_copy(path_ptr, span)?;

        let context = compiler.context();
        let i32_type = context.i32_type();

        let access_fn = compiler.libc().access_fn;
        let result = compiler
            .builder()
            .build_call(access_fn, &[owned_ptr.into(), i32_type.const_int(0, false).into()], "access.call")
            .map_err(err)?
            .try_as_basic_value()
            .basic()
            .expect("access should return a value")
            .into_int_value();

        let exists = compiler
            .builder()
            .build_int_compare(inkwell::IntPredicate::EQ, result, i32_type.const_int(0, false), "access.exists")
            .map_err(err)?;

        let free_fn = compiler.libc().free_fn;

        compiler.builder().build_call(free_fn, &[owned_ptr.into()], "free.path").map_err(err)?;

        compiler.set_last_value(LlvmValue::Bool(exists));
        Ok(())
    };

    StdFunction {
        params,
        execute,
        passed_by: vec![PassedBy::Value],
        return_type: Type::Bool,
        type_check: None,
        compile,
    }
}
