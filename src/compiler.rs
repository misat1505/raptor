use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassBuilderOptions;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::types::{BasicTypeEnum, IntType, PointerType};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::{IntPredicate, OptimizationLevel};

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
    printf_fn: Option<FunctionValue<'ctx>>,
    snprintf_fn: Option<FunctionValue<'ctx>>,

    // płaska tabela zmiennych: nazwa -> wskaźnik z `alloca`.
    // TODO: docelowo zastąpić stosem zakresów, analogicznie do ScopeManager w interpreterze.
    variables: HashMap<String, PointerValue<'ctx>>,

    // odpowiednik `last_result` z interpretera — wynik ostatnio odwiedzonego wyrażenia.
    last_value: Option<BasicValueEnum<'ctx>>,

    position: Position,
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
            printf_fn: None,
            snprintf_fn: None,
            variables: HashMap::new(),
            last_value: None,
            position: Position {
                filename: None,
                line: 0,
                column: 0,
                offset: 0,
            },
        }
    }

    pub fn compile(&mut self) -> Result<(), Box<dyn IError>> {
        self.declare_main_function();
        self.declare_printf();
        self.declare_snprintf();
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

    fn i64_type(&self) -> IntType<'ctx> {
        self.context.i64_type()
    }

    fn i32_type(&self) -> IntType<'ctx> {
        self.context.i32_type()
    }

    fn string_type(&self) -> PointerType<'ctx> {
        self.context.ptr_type(inkwell::AddressSpace::default())
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

    fn declare_printf(&mut self) {
        let i32_type = self.i32_type();
        let str_type = self.string_type();
        let printf_type = i32_type.fn_type(&[str_type.into()], true);
        let function = self.module.add_function("printf", printf_type, None);
        self.printf_fn = Some(function);
    }

    fn declare_snprintf(&mut self) {
        let i32_type = self.i32_type();
        let i64_type = self.i64_type();
        let str_type = self.string_type();

        let snprintf_type = i32_type.fn_type(&[str_type.into(), i64_type.into(), str_type.into()], true);
        let function = self.module.add_function("snprintf", snprintf_type, None);
        self.snprintf_fn = Some(function);
    }

    fn read_last_value(&mut self) -> Result<BasicValueEnum<'ctx>, Box<dyn IError>> {
        self.last_value.take().ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("No value produced where it is needed."),
                self.position,
            )) as Box<dyn IError>
        })
    }

    fn get_variable(&self, name: &str) -> Result<PointerValue<'ctx>, Box<dyn IError>> {
        self.variables.get(name).copied().ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Undeclared variable '{}'.", name),
                self.position,
            )) as Box<dyn IError>
        })
    }

    fn current_function(&self) -> FunctionValue<'ctx> {
        self.builder
            .get_insert_block()
            .expect("builder should be positioned inside a function")
            .get_parent()
            .expect("basic block should belong to a function")
    }
}

impl<'a, 'ctx> Visitor<'a> for Compiler<'a, 'ctx> {
    fn visit_program(&mut self, program: &'a Program) -> Result<(), Box<dyn IError>> {
        for statement in &program.statements {
            self.visit_statement(statement)?;
        }

        Ok(())
    }

    fn visit_statement(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        self.position = statement.position;

        match &statement.value {
            Statement::FunctionCall { identifier, arguments } => match identifier.value.as_str() {
                "println" => {
                    let arg = arguments.get(0).ok_or_else(|| {
                        Box::new(CompilerError::at(
                            ErrorSeverity::HIGH,
                            String::from("'println' expects exactly one argument."),
                            statement.position,
                        )) as Box<dyn IError>
                    })?;

                    self.visit_expression(&arg.value.value)?;
                    let text_value = self.read_last_value()?;

                    let printf_fn = self.printf_fn.expect("printf should be declared before visiting the program");

                    let format_str = self
                        .builder
                        .build_global_string_ptr("%s\n", "fmt")
                        .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                    self.builder
                        .build_call(printf_fn, &[format_str.as_pointer_value().into(), text_value.into()], "printf_call")
                        .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                    Ok(())
                }

                other => Err(Box::new(CompilerError::at(
                    ErrorSeverity::HIGH,
                    format!("Compiling calls to '{}' is not yet supported.", other),
                    statement.position,
                ))),
            },
            Statement::Declaration { var_type, identifier, value } => {
                let llvm_type: BasicTypeEnum<'ctx> = match &var_type.value {
                    Type::I64 => self.i64_type().into(),
                    Type::Str => self.string_type().into(),
                    other => {
                        return Err(Box::new(CompilerError::at(
                            ErrorSeverity::HIGH,
                            format!("Compiling declarations of type '{:?}' is not yet supported.", other),
                            statement.position,
                        )))
                    }
                };

                let ptr = self
                    .builder
                    .build_alloca(llvm_type, identifier.value.as_str())
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                if let Some(val_expr) = value {
                    self.visit_expression(val_expr)?;
                    let init_value = self.read_last_value()?;
                    self.builder
                        .build_store(ptr, init_value)
                        .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;
                }

                self.variables.insert(identifier.value.clone(), ptr);

                Ok(())
            }

            Statement::Assignment { identifier, value, indices } => {
                if !indices.is_empty() {
                    return Err(Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        String::from("Compiling indexed assignment is not yet supported."),
                        statement.position,
                    )));
                }

                self.visit_expression(value)?;
                let new_value = self.read_last_value()?;

                let ptr = self.get_variable(identifier.value.as_str())?;
                self.builder
                    .build_store(ptr, new_value)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                Ok(())
            }

            Statement::ForLoop {
                declaration,
                condition,
                assignment,
                block,
            } => {
                let function = self.current_function();

                if let Some(decl) = declaration {
                    self.visit_statement(decl)?;
                }

                let cond_block = self.context.append_basic_block(function, "for.cond");
                let body_block = self.context.append_basic_block(function, "for.body");
                let after_block = self.context.append_basic_block(function, "for.after");

                self.builder
                    .build_unconditional_branch(cond_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                self.builder.position_at_end(cond_block);
                self.visit_expression(condition)?;
                let cond_value = self.read_last_value()?.into_int_value();
                self.builder
                    .build_conditional_branch(cond_value, body_block, after_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                self.builder.position_at_end(body_block);
                self.visit_block(block)?;

                if let Some(assign) = assignment {
                    self.visit_statement(assign)?;
                }

                // TODO: `break`/`continue` będą wymagały osobnego stosu bloków docelowych,
                // na razie zakładamy, że blok nigdy nie kończy się już terminatorem.
                self.builder
                    .build_unconditional_branch(cond_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                self.builder.position_at_end(after_block);

                Ok(())
            }

            Statement::Conditional {
                condition,
                if_block,
                else_block,
            } => {
                let function = self.current_function();

                let cond_block = self.context.append_basic_block(function, "if.cond");
                let true_block = self.context.append_basic_block(function, "if.true");
                let false_block = match else_block {
                    Some(_) => Some(self.context.append_basic_block(function, "if.false")),
                    None => None,
                };
                let after_block = self.context.append_basic_block(function, "if.after");

                self.builder
                    .build_unconditional_branch(cond_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                self.builder.position_at_end(cond_block);
                self.visit_expression(condition)?;
                let cond_value = self.read_last_value()?.into_int_value();
                self.builder
                    .build_conditional_branch(cond_value, true_block, false_block.unwrap_or(after_block))
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                self.builder.position_at_end(true_block);
                self.visit_block(if_block)?;
                self.builder
                    .build_unconditional_branch(after_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                if let Some(b) = false_block {
                    self.builder.position_at_end(b);
                    self.visit_block(&else_block.as_ref().unwrap())?;
                    self.builder
                        .build_unconditional_branch(after_block)
                        .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;
                }

                self.builder.position_at_end(after_block);

                Ok(())
            }

            Statement::WhileLoop { condition, block } => {
                let function = self.current_function();

                let cond_block = self.context.append_basic_block(function, "while.cond");
                let body_block = self.context.append_basic_block(function, "while.block");
                let after_block = self.context.append_basic_block(function, "while.after");

                self.builder
                    .build_unconditional_branch(cond_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                self.builder.position_at_end(cond_block);
                self.visit_expression(condition)?;
                let cond_value = self.read_last_value()?.into_int_value();
                self.builder
                    .build_conditional_branch(cond_value, body_block, after_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                self.builder.position_at_end(body_block);
                self.visit_block(block)?;
                self.builder
                    .build_unconditional_branch(cond_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                self.builder.position_at_end(after_block);

                Ok(())
            }

            other => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Compiling statement '{:?}' is not yet supported.", other),
                statement.position,
            ))),
        }
    }

    fn visit_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        self.position = expression.position;

        match &expression.value {
            Expression::Literal(literal) => self.visit_literal(literal),

            Expression::Variable(variable) => self.visit_variable(variable, expression.position),

            Expression::Addition(lhs, rhs) => {
                self.visit_expression(lhs)?;
                let left = self.read_last_value()?.into_int_value();

                self.visit_expression(rhs)?;
                let right = self.read_last_value()?.into_int_value();

                let result = self
                    .builder
                    .build_int_add(left, right, "addtmp")
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), expression.position)) as Box<dyn IError>)?;

                self.last_value = Some(result.into());

                Ok(())
            }

            Expression::Modulo(lhs, rhs) => {
                self.visit_expression(lhs)?;
                let left = self.read_last_value()?.into_int_value();

                self.visit_expression(rhs)?;
                let right = self.read_last_value()?.into_int_value();

                let result = self
                    .builder
                    .build_int_signed_rem(left, right, "remtmp")
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), expression.position)) as Box<dyn IError>)?;

                self.last_value = Some(result.into());

                Ok(())
            }

            Expression::Less(lhs, rhs) => {
                self.visit_expression(lhs)?;
                let left = self.read_last_value()?.into_int_value();

                self.visit_expression(rhs)?;
                let right = self.read_last_value()?.into_int_value();

                let result = self
                    .builder
                    .build_int_compare(IntPredicate::SLT, left, right, "cmptmp")
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), expression.position)) as Box<dyn IError>)?;

                self.last_value = Some(result.into());

                Ok(())
            }

            Expression::Equal(lhs, rhs) => {
                self.visit_expression(lhs)?;
                let left = self.read_last_value()?.into_int_value();

                self.visit_expression(rhs)?;
                let right = self.read_last_value()?.into_int_value();

                let result = self
                    .builder
                    .build_int_compare(IntPredicate::EQ, left, right, "cmptmp")
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), expression.position)) as Box<dyn IError>)?;

                self.last_value = Some(result.into());

                Ok(())
            }

            Expression::Casting { value, to_type } => {
                self.visit_expression(value)?;
                let source_value = self.read_last_value()?;

                match (&to_type.value, source_value) {
                    (Type::Str, BasicValueEnum::IntValue(int_value)) => {
                        let snprintf_fn = self.snprintf_fn.expect("snprintf should be declared before visiting the program");

                        let buffer_type = self.context.i8_type().array_type(24);
                        let buffer_ptr = self.builder.build_alloca(buffer_type, "int_to_str_buf").map_err(|err| {
                            Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), expression.position)) as Box<dyn IError>
                        })?;

                        let format_str = self.builder.build_global_string_ptr("%lld", "int_fmt").map_err(|err| {
                            Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), expression.position)) as Box<dyn IError>
                        })?;

                        let size = self.i64_type().const_int(24, false);

                        self.builder
                            .build_call(
                                snprintf_fn,
                                &[buffer_ptr.into(), size.into(), format_str.as_pointer_value().into(), int_value.into()],
                                "snprintf_call",
                            )
                            .map_err(|err| {
                                Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), expression.position)) as Box<dyn IError>
                            })?;

                        self.last_value = Some(buffer_ptr.into());

                        Ok(())
                    }

                    (other_type, _) => Err(Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("Casting to type '{:?}' is not yet supported.", other_type),
                        expression.position,
                    ))),
                }
            }

            other => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Compiling expression '{:?}' is not yet supported.", other),
                expression.position,
            ))),
        }
    }

    fn visit_parameter(&mut self, _parameter: &'a Node<Parameter>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_argument(&mut self, _argument: &'a Node<Argument>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_type(&mut self, _node_type: &'a Node<Type>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_block(&mut self, block: &'a Node<Block>) -> Result<(), Box<dyn IError>> {
        for statement in &block.value.0 {
            self.visit_statement(statement)?;
        }

        Ok(())
    }

    fn visit_switch_expression(&mut self, _switch_expression: &'a Node<SwitchExpression>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_switch_case(&mut self, _switch_case: &'a Node<SwitchCase>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_literal(&mut self, literal: &'a Literal) -> Result<(), Box<dyn IError>> {
        match literal {
            Literal::I64(value) => {
                let const_value = self.i64_type().const_int(*value as u64, true);
                self.last_value = Some(const_value.into());
                Ok(())
            }
            Literal::String(value) => {
                let string_value = self
                    .builder
                    .build_global_string_ptr(value.as_str(), "str")
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), self.position)) as Box<dyn IError>)?;

                self.last_value = Some(string_value.as_pointer_value().into());

                Ok(())
            }

            Literal::True => {
                self.last_value = Some(self.context.bool_type().const_int(1, false).into());
                Ok(())
            }

            Literal::False => {
                self.last_value = Some(self.context.bool_type().const_int(0, false).into());
                Ok(())
            }

            other => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Compiling literal '{:?}' is not yet supported.", other),
                self.position,
            ))),
        }
    }

    fn visit_vector_literal(&mut self, _vector: &'a Vec<Box<Node<Expression>>>) -> Result<(), Box<dyn IError>> {
        Err(Box::new(CompilerError::at(
            ErrorSeverity::HIGH,
            String::from("Compiling vector literals is not yet supported."),
            self.position,
        )))
    }

    fn visit_variable(&mut self, variable: &'a String, position: Position) -> Result<(), Box<dyn IError>> {
        let ptr = self.get_variable(variable.as_str())?;

        let value = self
            .builder
            .build_load(self.i64_type(), ptr, variable.as_str())
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), position)) as Box<dyn IError>)?;

        self.last_value = Some(value);

        Ok(())
    }
}
