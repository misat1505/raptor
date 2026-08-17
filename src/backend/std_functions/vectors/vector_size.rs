use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    backend::{
        interpreter::Value,
        llvm::LlvmValue,
        std_functions::std_functions::{build_usage_error, LlvmCompileFn, StdFunction},
    },
    common::{
        errors::{CompilerError, ErrorSeverity, IError, StdFunctionError},
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::PassedBy,
};

pub fn vector_size() -> StdFunction {
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

    let compile: LlvmCompileFn = |compiler, arguments, position| {
        let err = |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

        let vector_arg = arguments.get(0).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("'vector_size' expects exactly one argument."),
                position,
            )) as Box<dyn IError>
        })?;

        compiler.visit_expression(&vector_arg.value.value)?;

        let vector_value = compiler.read_last_value()?;

        let vector_ptr = match vector_value {
            LlvmValue::Vector(ptr, _) => ptr,

            other => {
                return Err(Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    format!("'vector_size' expects a vector, got '{:?}'.", other.to_type()),
                    position,
                )))
            }
        };

        let context = compiler.context();
        let i64_type = context.i64_type();
        let struct_type = LlvmValue::vector_struct_type(context);

        let length_field = compiler
            .builder()
            .build_struct_gep(struct_type, vector_ptr, 1, "vector.length")
            .map_err(err)?;

        let length = compiler
            .builder()
            .build_load(i64_type, length_field, "vector.size")
            .map_err(err)?
            .into_int_value();

        compiler.set_last_value(LlvmValue::I64(length));

        Ok(())
    };

    StdFunction {
        params,
        passed_by: vec![PassedBy::Reference],
        execute,
        return_type: Type::I64,
        type_check: Some(type_check),
        compile,
    }
}
