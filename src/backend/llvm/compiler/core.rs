use inkwell::passes::PassBuilderOptions;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;

use super::Compiler;
use crate::common::errors::{CompilerError, ErrorSeverity, IError};
use crate::common::visitor::Visitor;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile(&mut self) -> Result<(), Box<dyn IError>> {
        self.declare_main_function();
        self.declare_functions()?;
        self.declare_extern_functions()?;

        self.compile_functions()?;

        let main_fn = self.main_fn.expect("main function should be declared");
        let main_entry = main_fn.get_first_basic_block().expect("main should have an entry block");

        self.builder.position_at_end(main_entry);

        self.visit_program(self.program)?;

        self.finish_main_function();
        self.verify_module()?;

        Ok(())
    }

    pub(in crate::backend::llvm::compiler) fn verify_module(&self) -> Result<(), Box<dyn IError>> {
        self.module
            .verify()
            .map_err(|err| Box::new(CompilerError::new(ErrorSeverity::HIGH, format!("Module verification failed: {}", err))) as Box<dyn IError>)
    }

    pub fn optimize(&self, level: OptimizationLevel) -> Result<(), Box<dyn IError>> {
        Target::initialize_native(&InitializationConfig::default()).map_err(|err| {
            Box::new(CompilerError::new(
                ErrorSeverity::HIGH,
                format!("Failed to initialize native target: {}", err),
            )) as Box<dyn IError>
        })?;

        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple)
            .map_err(|err| Box::new(CompilerError::new(ErrorSeverity::HIGH, format!("Failed to get target: {}", err))) as Box<dyn IError>)?;

        let target_machine = target
            .create_target_machine(
                &triple,
                &TargetMachine::get_host_cpu_name().to_string(),
                &TargetMachine::get_host_cpu_features().to_string(),
                level,
                RelocMode::Default,
                CodeModel::Default,
            )
            .ok_or_else(|| Box::new(CompilerError::new(ErrorSeverity::HIGH, String::from("Failed to create target machine"))) as Box<dyn IError>)?;

        let passes = match level {
            OptimizationLevel::None => "default<O0>",
            OptimizationLevel::Less => "default<O1>",
            OptimizationLevel::Default => "default<O2>",
            OptimizationLevel::Aggressive => "default<O3>",
        };

        self.module
            .run_passes(passes, &target_machine, PassBuilderOptions::create())
            .map_err(|err| Box::new(CompilerError::new(ErrorSeverity::HIGH, format!("Module optimization failed: {}", err))) as Box<dyn IError>)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn print_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub fn write_ir_to_file(&self, path: &str) -> Result<(), Box<dyn IError>> {
        self.module
            .print_to_file(path)
            .map_err(|err| Box::new(CompilerError::new(ErrorSeverity::HIGH, format!("Failed to write IR to file: {}", err))) as Box<dyn IError>)
    }

    pub(in crate::backend::llvm::compiler) fn declare_main_function(&mut self) {
        let fn_type = self.i32_type().fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);

        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        self.main_fn = Some(function);
    }

    pub(in crate::backend::llvm::compiler) fn finish_main_function(&mut self) {
        let current_block = self.builder.get_insert_block().expect("builder should be positioned inside main");

        if current_block.get_terminator().is_none() {
            let zero = self.i32_type().const_int(0, false);
            self.builder.build_return(Some(&zero)).expect("failed to build return");
        }
    }
}
