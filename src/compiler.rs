use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::IntType;
use inkwell::values::FunctionValue;
use inkwell::OptimizationLevel;

use crate::{
    ast::{Program, Statement},
    errors::{CompilerError, ErrorSeverity, IError},
};

pub struct Compiler<'a, 'ctx> {
    program: &'a Program,
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    // funkcja main, do której będziemy wstrzykiwać kod top-levelowych statementów
    main_fn: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(program: &'a Program, context: &'ctx Context) -> Self {
        let module = context.create_module("main_module");
        let builder = context.create_builder();

        Compiler {
            program,
            context,
            module,
            builder,
            main_fn: None,
        }
    }

    pub fn compile(&mut self) -> Result<(), Box<dyn IError>> {
        self.declare_main_function();
        self.visit_program(self.program)?;
        self.finish_main_function();
        self.verify_module()?;

        Ok(())
    }

    /// Zwraca wygenerowany LLVM IR jako string — przydatne do debugowania i testów.
    pub fn print_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }

    /// Zapisuje wygenerowany moduł LLVM IR (.ll) do pliku.
    pub fn write_ir_to_file(&self, path: &str) -> Result<(), Box<dyn IError>> {
        self.module
            .print_to_file(path)
            .map_err(|err| Box::new(CompilerError::new(ErrorSeverity::HIGH, format!("Failed to write IR to file: {}", err))) as Box<dyn IError>)
    }

    fn verify_module(&self) -> Result<(), Box<dyn IError>> {
        self.module
            .verify()
            .map_err(|err| Box::new(CompilerError::new(ErrorSeverity::HIGH, format!("Module verification failed: {}", err))) as Box<dyn IError>)
    }

    fn i64_type(&self) -> IntType<'ctx> {
        self.context.i64_type()
    }

    fn i32_type(&self) -> IntType<'ctx> {
        self.context.i32_type()
    }

    /// Deklaruje `fn main() -> i32` i ustawia builder na jej bloku wejściowym.
    /// Wszystkie top-levelowe statementy programu wylądują wewnątrz tej funkcji.
    fn declare_main_function(&mut self) {
        let fn_type = self.i32_type().fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);

        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        self.main_fn = Some(function);
    }

    /// Dodaje `return 0;` na końcu `main`, o ile blok nie ma już terminatora
    /// (np. gdy jakiś wcześniejszy `return` z poziomu top-level już go ustawił).
    fn finish_main_function(&mut self) {
        let current_block = self.builder.get_insert_block().expect("builder should be positioned inside main");

        if current_block.get_terminator().is_none() {
            let zero = self.i32_type().const_int(0, false);
            self.builder.build_return(Some(&zero)).expect("failed to build return");
        }
    }

    fn visit_program(&mut self, program: &'a Program) -> Result<(), Box<dyn IError>> {
        // TODO: najpierw wygenerować deklaracje funkcji użytkownika (program.functions),
        // żeby wywołania mogły się do nich odwoływać niezależnie od kolejności w pliku.
        for _function in program.functions.values() {
            // self.declare_function(function)?;
        }

        // TODO: właściwe wypełnienie ciał funkcji użytkownika.
        for _function in program.functions.values() {
            // self.compile_function_body(function)?;
        }

        // Top-levelowe statementy trafiają do ciała `main`.
        for statement in &program.statements {
            self.visit_statement(statement)?;
        }

        Ok(())
    }

    fn visit_statement(&mut self, statement: &'a crate::ast::Node<Statement>) -> Result<(), Box<dyn IError>> {
        match &statement.value {
            Statement::Declaration { .. } => {
                // TODO
                Ok(())
            }
            Statement::Assignment { .. } => {
                // TODO
                Ok(())
            }
            Statement::FunctionCall { .. } => {
                // TODO
                Ok(())
            }
            Statement::Conditional { .. } => {
                // TODO
                Ok(())
            }
            Statement::WhileLoop { .. } => {
                // TODO
                Ok(())
            }
            Statement::ForLoop { .. } => {
                // TODO
                Ok(())
            }
            Statement::Switch { .. } => {
                // TODO
                Ok(())
            }
            Statement::Return(_) => {
                // TODO
                Ok(())
            }
            Statement::Break => {
                // TODO
                Ok(())
            }
            Statement::Continue => {
                // TODO
                Ok(())
            }
        }
    }
}
