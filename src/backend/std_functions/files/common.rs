use crate::{
    backend::llvm::compiler::Compiler,
    common::{
        errors::{CompilerError, ErrorSeverity, IError},
        position::Position,
        visitor::Visitor,
    },
    frontend::ast::{Argument, Node},
};

pub fn compile_write_or_append<'a, 'ctx>(
    compiler: &mut Compiler<'a, 'ctx>,
    arguments: &'a Vec<Box<Node<Argument>>>,
    mode_str: &str,
    position: Position,
) -> Result<(), Box<dyn IError>> {
    let err = |e: inkwell::builder::BuilderError| Box::new(CompilerError::at(ErrorSeverity::HIGH, e.to_string(), position)) as Box<dyn IError>;

    let path_arg = arguments.get(0).ok_or_else(|| {
        Box::new(CompilerError::at(
            ErrorSeverity::HIGH,
            String::from("Expected a file path argument."),
            position,
        )) as Box<dyn IError>
    })?;
    let content_arg = arguments.get(1).ok_or_else(|| {
        Box::new(CompilerError::at(
            ErrorSeverity::HIGH,
            String::from("Expected a content argument."),
            position,
        )) as Box<dyn IError>
    })?;

    compiler.visit_expression(&path_arg.value.value)?;
    let path_ptr = compiler.read_last_value()?.into_str_value(position)?;

    compiler.visit_expression(&content_arg.value.value)?;
    let content_ptr = compiler.read_last_value()?.into_str_value(position)?;

    let mode = compiler
        .builder()
        .build_global_string_ptr(mode_str, "mode")
        .map_err(err)?
        .as_pointer_value();
    let fopen_fn = compiler.libc().fopen_fn;
    let file = compiler
        .builder()
        .build_call(fopen_fn, &[path_ptr.into(), mode.into()], "fopen.call")
        .map_err(err)?
        .try_as_basic_value()
        .basic()
        .expect("fopen should return a value")
        .into_pointer_value();

    let strlen_fn = compiler.libc().strlen_fn;
    let len = compiler
        .builder()
        .build_call(strlen_fn, &[content_ptr.into()], "content.len")
        .map_err(err)?
        .try_as_basic_value()
        .basic()
        .expect("strlen should return a value")
        .into_int_value();

    let i64_type = compiler.context().i64_type();
    let fwrite_fn = compiler.libc().fwrite_fn;
    compiler
        .builder()
        .build_call(
            fwrite_fn,
            &[content_ptr.into(), i64_type.const_int(1, false).into(), len.into(), file.into()],
            "fwrite.call",
        )
        .map_err(err)?;

    let fclose_fn = compiler.libc().fclose_fn;
    compiler.builder().build_call(fclose_fn, &[file.into()], "fclose.call").map_err(err)?;

    Ok(())
}
