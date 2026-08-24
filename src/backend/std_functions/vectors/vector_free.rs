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

pub fn vector_free() -> StdFunction {
    let params = vec![Type::Vector(Box::new(Type::Void))];

    let execute = |params: &Vec<Rc<RefCell<Value>>>, span: Span| -> Result<Option<Value>, StdFunctionError> {
        let fn_name = "vector_free";
        let expected_types = vec![Type::Vector(Box::new(Type::Void))];
        let mut actual_types = Vec::new();

        if let Some(vector) = params.get(0) {
            actual_types.push(vector.borrow().to_type());

            let vector = vector.borrow();

            match &*vector {
                Value::Vector { .. } => Ok(None),
                _ => Err(build_usage_error(fn_name, expected_types, actual_types, span)),
            }
        } else {
            Err(build_usage_error(fn_name, expected_types, actual_types, span))
        }
    };

    let type_check: fn(&[Type]) -> Result<Type, String> = |arg_types: &[Type]| match arg_types {
        [Type::Vector(_)] => Ok(Type::Void),

        [other] => Err(format!("vector_free expected a vector, but got '{:?}'.", other)),

        _ => Err(String::from("vector_free expects exactly 1 argument.")),
    };

    let compile: LlvmCompileFn = |compiler, arguments, span| {
        let err = |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), span)) as Box<dyn IError>;

        let vector_arg = arguments.get(0).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'vector_free' expects exactly one argument."),
                span,
            )) as Box<dyn IError>
        })?;

        compiler.visit_expression(&vector_arg.value.value)?;

        let vector_value = compiler.read_last_value()?;

        let vector_ptr = match vector_value {
            LlvmValue::Vector(ptr, _) => ptr,

            other => {
                return Err(Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    format!("'vector_free' expects a vector, got '{:?}'.", other.to_type()),
                    span,
                )));
            }
        };

        let context = compiler.context();

        let ptr_type = context.ptr_type(inkwell::AddressSpace::default());
        let struct_type = LlvmValue::vector_struct_type(context);

        // vector.data
        let data_field = compiler
            .builder()
            .build_struct_gep(struct_type, vector_ptr, 0, "vector.data")
            .map_err(err)?;

        let data_ptr = compiler
            .builder()
            .build_load(ptr_type, data_field, "vector.data.ptr")
            .map_err(err)?
            .into_pointer_value();

        // free(vector.data)
        compiler
            .builder()
            .build_call(compiler.libc().free_fn, &[data_ptr.into()], "vector.data.free")
            .map_err(err)?;

        // free(vector.header)
        compiler
            .builder()
            .build_call(compiler.libc().free_fn, &[vector_ptr.into()], "vector.header.free")
            .map_err(err)?;

        Ok(())
    };

    StdFunction {
        params,
        passed_by: vec![PassedBy::Reference],
        execute,
        return_type: Type::Void,
        type_check: Some(type_check),
        compile,
    }
}
