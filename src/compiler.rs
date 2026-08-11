use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::IntType;
use inkwell::values::FunctionValue;

use crate::{
    ast::{Argument, Block, Expression, Literal, Node, Parameter, Program, Statement, SwitchCase, SwitchExpression, Type},
    errors::{CompilerError, ErrorSeverity, IError},
    lazy_stream_reader::Position,
    visitor::Visitor,
};

pub struct Compiler<'a, 'ctx> {
    program: &'a Program,
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

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

    pub fn print_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }

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

    #[allow(dead_code)]
    fn i64_type(&self) -> IntType<'ctx> {
        self.context.i64_type()
    }

    fn i32_type(&self) -> IntType<'ctx> {
        self.context.i32_type()
    }

    fn declare_main_function(&mut self) {
        let fn_type = self.i32_type().fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);

        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        self.main_fn = Some(function);
    }

    fn finish_main_function(&mut self) {
        let current_block = self.builder.get_insert_block().expect("builder should be positioned inside main");

        if current_block.get_terminator().is_none() {
            let zero = self.i32_type().const_int(0, false);
            self.builder.build_return(Some(&zero)).expect("failed to build return");
        }
    }
}

impl<'a, 'ctx> Visitor<'a> for Compiler<'a, 'ctx> {
    fn visit_program(&mut self, program: &'a Program) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_statement(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_parameter(&mut self, parameter: &'a Node<Parameter>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_argument(&mut self, argument: &'a Node<Argument>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_type(&mut self, node_type: &'a Node<Type>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_block(&mut self, block: &'a Node<Block>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_switch_expression(&mut self, switch_expression: &'a Node<SwitchExpression>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_switch_case(&mut self, switch_case: &'a Node<SwitchCase>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_literal(&mut self, literal: &'a Literal) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_vector_literal(&mut self, vector: &'a Vec<Box<Node<Expression>>>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_variable(&mut self, variable: &'a String, position: Position) -> Result<(), Box<dyn IError>> {
        Ok(())
    }
}
