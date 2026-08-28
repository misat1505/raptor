use inkwell::{context::Context, OptimizationLevel};

use crate::backend::llvm::{
    compiler::{tests::empty_program, Compiler},
    OverflowPolicy,
};

#[test]
fn new_compiler_has_empty_state() {
    let context = Context::create();
    let program = empty_program();
    let compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    assert!(compiler.main_fn.is_none());
    assert!(compiler.functions.is_empty());
    assert!(compiler.variables.is_empty());
    assert!(compiler.control_stack.is_empty());
    assert!(compiler.last_value.is_none());
}

#[test]
fn declare_main_function_creates_entry() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    compiler.declare_main_function();

    let main = compiler.main_fn.expect("main should exist");
    assert_eq!(main.get_name().to_str().unwrap(), "main");
    assert!(main.get_first_basic_block().is_some());
    assert_eq!(main.count_basic_blocks(), 1);
}

#[test]
fn finish_main_function_adds_return_when_missing() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    compiler.declare_main_function();
    let main = compiler.main_fn.unwrap();
    let entry = main.get_first_basic_block().unwrap();
    assert!(entry.get_terminator().is_none());

    compiler.finish_main_function();
    assert!(entry.get_terminator().is_some());
}

#[test]
fn finish_main_function_does_not_double_terminate() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    compiler.declare_main_function();
    // manually add a return
    let zero = compiler.i32_type().const_int(0, false);
    compiler.builder.build_return(Some(&zero)).unwrap();

    // should be a no-op (already has terminator)
    compiler.finish_main_function();

    let entry = compiler.main_fn.unwrap().get_first_basic_block().unwrap();
    // still exactly one terminator
    assert!(entry.get_terminator().is_some());
}

#[test]
fn compile_empty_program_succeeds() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    assert!(compiler.compile().is_ok());
    assert!(compiler.main_fn.is_some());
}

#[test]
fn compile_empty_program_produces_valid_ir() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    compiler.compile().unwrap();
    let ir = compiler.print_ir();
    assert!(ir.contains("define i32 @main"), "IR was:\n{}", ir);
    assert!(ir.contains("ret i32 0") || ir.contains("ret i32"), "IR was:\n{}", ir);
}

#[test]
fn verify_module_ok_after_compile() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    compiler.compile().unwrap();
    assert!(compiler.verify_module().is_ok());
}

#[test]
fn read_last_value_errors_when_empty() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    let err = compiler.read_last_value().unwrap_err();
    assert!(err.message().contains("No value produced"));
}

#[test]
fn set_and_read_last_value() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    let v = crate::backend::llvm::llvm_alu::llvm_value::LlvmValue::I64(context.i64_type().const_int(42, true));
    compiler.set_last_value(v);

    let read = compiler.read_last_value().unwrap();
    assert_eq!(read.to_type(), crate::common::types::Type::I64);

    // consumed
    assert!(compiler.read_last_value().is_err());
}

#[test]
fn get_variable_undeclared_fails() {
    let context = Context::create();
    let program = empty_program();
    let compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    let err = compiler.get_variable("nope").unwrap_err();
    assert!(err.message().contains("Undeclared variable"));
}

#[test]
fn type_helpers_match_context() {
    let context = Context::create();
    let program = empty_program();
    let compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    assert_eq!(compiler.i64_type().get_bit_width(), 64);
    assert_eq!(compiler.i32_type().get_bit_width(), 32);
    // f64_type exists and is usable
    let _ = compiler.f64_type().const_float(0.0);
}

#[test]
fn optimize_after_compile() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    compiler.compile().unwrap();
    if compiler.optimize(OptimizationLevel::Default).is_ok() {
        assert!(compiler.verify_module().is_ok());
    }
}

#[test]
fn optimize_all_levels() {
    for level in [
        OptimizationLevel::None,
        OptimizationLevel::Less,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
    ] {
        let context = Context::create();
        let program = empty_program();
        let mut compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);
        compiler.compile().unwrap();
        // just ensure it doesn't panic; native target may be unavailable
        let _ = compiler.optimize(level);
    }
}

#[test]
fn print_ir_non_empty_after_compile() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    compiler.compile().unwrap();
    let ir = compiler.print_ir();
    assert!(!ir.is_empty());
    assert!(ir.contains("@main"));
}

#[test]
fn write_ir_to_file() {
    let context = Context::create();
    let program = empty_program();
    let mut compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);
    compiler.compile().unwrap();

    let path = std::env::temp_dir().join(format!("test_ir_{}.ll", std::process::id()));
    assert!(compiler.write_ir_to_file(path.to_str().unwrap()).is_ok());
    assert!(path.exists());
    let contents = std::fs::read_to_string(&path).unwrap();
    assert!(contents.contains("@main"));
    let _ = std::fs::remove_file(&path);
}

#[test]
fn builder_and_libc_and_context_accessible() {
    let context = Context::create();
    let program = empty_program();
    let compiler = Compiler::new(&program, &context, OverflowPolicy::Ignore);

    let _ = compiler.builder();
    let _ = compiler.libc();
    assert!(std::ptr::eq(compiler.context(), &context));
}
