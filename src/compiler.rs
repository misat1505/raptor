use std::collections::HashMap;

use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassBuilderOptions;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, FloatType, IntType};
use inkwell::values::{BasicMetadataValueEnum, FunctionValue, PointerValue};
use inkwell::{AddressSpace, OptimizationLevel};

use crate::ast::{FunctionDeclaration, PassedBy};
use crate::libc_functions::LibcFunctions;
use crate::llvm_alu::LlvmAlu;
use crate::llvm_value::LlvmValue;
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
    functions: HashMap<String, FunctionValue<'ctx>>,
    libc: LibcFunctions<'ctx>,

    // płaska tabela zmiennych: nazwa -> wskaźnik z `alloca`.
    // TODO: docelowo zastąpić stosem zakresów, analogicznie do ScopeManager w interpreterze.
    variables: HashMap<String, (PointerValue<'ctx>, Type)>,

    last_value: Option<LlvmValue<'ctx>>,

    position: Position,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(program: &'a Program, context: &'ctx Context) -> Self {
        let module = context.create_module("main_module");
        let builder = context.create_builder();
        let libc = LibcFunctions::new(context, &module);

        Compiler {
            program,
            context,
            module,
            builder,
            main_fn: None,
            functions: HashMap::new(),
            libc,
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
        self.declare_functions()?;

        self.compile_functions()?;

        let main_fn = self.main_fn.expect("main function should be declared");
        let main_entry = main_fn.get_first_basic_block().expect("main should have an entry block");

        self.builder.position_at_end(main_entry);

        self.visit_program(self.program)?;

        self.finish_main_function();
        self.verify_module()?;

        Ok(())
    }

    fn declare_functions(&mut self) -> Result<(), Box<dyn IError>> {
        for (name, declaration) in &self.program.functions {
            let function_decl = &declaration.value;

            let mut param_types: Vec<BasicMetadataTypeEnum> = Vec::with_capacity(function_decl.parameters.len());

            for parameter in &function_decl.parameters {
                let param_type: BasicMetadataTypeEnum = match parameter.value.passed_by {
                    PassedBy::Reference => self.context.ptr_type(AddressSpace::default()).into(),
                    PassedBy::Value => {
                        let llvm_type = LlvmValue::type_to_basic_type_enum(&parameter.value.parameter_type.value, self.context).ok_or_else(|| {
                            Box::new(CompilerError::at(
                                ErrorSeverity::HIGH,
                                format!(
                                    "Compiling parameters of type '{:?}' is not yet supported.",
                                    parameter.value.parameter_type.value
                                ),
                                parameter.position,
                            )) as Box<dyn IError>
                        })?;

                        llvm_type.into()
                    }
                };

                param_types.push(param_type);
            }

            let fn_type = match &function_decl.return_type.value {
                Type::Void => self.context.void_type().fn_type(&param_types, false),
                return_type => {
                    let llvm_return_type = LlvmValue::type_to_basic_type_enum(return_type, self.context).ok_or_else(|| {
                        Box::new(CompilerError::at(
                            ErrorSeverity::HIGH,
                            format!("Compiling functions returning '{:?}' is not yet supported.", return_type),
                            function_decl.return_type.position,
                        )) as Box<dyn IError>
                    })?;

                    llvm_return_type.fn_type(&param_types, false)
                }
            };

            let function = self.module.add_function(name, fn_type, None);
            self.functions.insert(name.clone(), function);
        }

        Ok(())
    }

    fn compile_functions(&mut self) -> Result<(), Box<dyn IError>> {
        for (name, declaration) in &self.program.functions {
            let function = *self
                .functions
                .get(name)
                .expect("function should have been predeclared by declare_functions");

            self.compile_function_body(function, &declaration.value)?;
        }

        Ok(())
    }

    fn compile_function_body(&mut self, function: FunctionValue<'ctx>, function_decl: &'a FunctionDeclaration) -> Result<(), Box<dyn IError>> {
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        let saved_variables = std::mem::take(&mut self.variables);

        for (index, parameter) in function_decl.parameters.iter().enumerate() {
            let identifier = parameter.value.identifier.value.as_str();
            let param_type = &parameter.value.parameter_type.value;

            let param_value = function
                .get_nth_param(index as u32)
                .expect("parameter index should be valid, matches signature built in declare_functions");

            match parameter.value.passed_by {
                PassedBy::Value => {
                    let llvm_type = LlvmValue::type_to_basic_type_enum(param_type, self.context).ok_or_else(|| {
                        Box::new(CompilerError::at(
                            ErrorSeverity::HIGH,
                            format!("Compiling parameters of type '{:?}' is not yet supported.", param_type),
                            parameter.position,
                        )) as Box<dyn IError>
                    })?;

                    // kopiujemy wartość parametru do lokalnego alloca, żeby dało się ją przypisywać jak zwykłą zmienną
                    let ptr = self
                        .builder
                        .build_alloca(llvm_type, identifier)
                        .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), parameter.position)) as Box<dyn IError>)?;

                    self.builder
                        .build_store(ptr, param_value)
                        .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), parameter.position)) as Box<dyn IError>)?;

                    self.variables.insert(identifier.to_string(), (ptr, param_type.clone()));
                }

                PassedBy::Reference => {
                    // parametr od razu jest wskaźnikiem do zmiennej wołającego
                    let ptr = param_value.into_pointer_value();
                    self.variables.insert(identifier.to_string(), (ptr, param_type.clone()));
                }
            }
        }

        self.visit_block(&function_decl.block)?;

        let current_block = self.builder.get_insert_block().expect("builder should be positioned inside the function");

        if current_block.get_terminator().is_none() {
            match &function_decl.return_type.value {
                Type::Void => {
                    self.builder.build_return(None).map_err(|err| {
                        Box::new(CompilerError::at(
                            ErrorSeverity::HIGH,
                            err.to_string(),
                            function_decl.return_type.position,
                        )) as Box<dyn IError>
                    })?;
                }
                // funkcja niepusta bez jawnego return na końcu ścieżki — na razie unreachable,
                // docelowo to powinno być wykrywane wcześniej jako błąd "not all paths return a value"
                _ => {
                    self.builder.build_unreachable().map_err(|err| {
                        Box::new(CompilerError::at(
                            ErrorSeverity::HIGH,
                            err.to_string(),
                            function_decl.return_type.position,
                        )) as Box<dyn IError>
                    })?;
                }
            }
        }

        self.variables = saved_variables;

        Ok(())
    }

    fn build_function_call(
        &mut self,
        identifier: &'a Node<String>,
        arguments: &'a Vec<Box<Node<Argument>>>,
        position: Position,
    ) -> Result<(), Box<dyn IError>> {
        let name = identifier.value.as_str();

        let function = *self.functions.get(name).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Compiling calls to '{}' is not yet supported.", name),
                position,
            )) as Box<dyn IError>
        })?;

        let mut compiled_args: Vec<BasicMetadataValueEnum> = Vec::with_capacity(arguments.len());

        for argument in arguments {
            match argument.value.passed_by {
                PassedBy::Value => {
                    self.visit_expression(&argument.value.value)?;
                    let value = self.read_last_value()?;
                    compiled_args.push(value.as_basic_value_enum().into());
                }

                PassedBy::Reference => {
                    let ptr = self.resolve_reference(&argument.value.value)?;
                    compiled_args.push(ptr.into());
                }
            }
        }

        let call_site = self
            .builder
            .build_call(function, &compiled_args, "call")
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), position)) as Box<dyn IError>)?;

        self.last_value = match call_site.try_as_basic_value().basic() {
            Some(return_value) => {
                let return_type = &self
                    .program
                    .functions
                    .get(name)
                    .expect("function existence already checked above")
                    .value
                    .return_type
                    .value;

                Some(LlvmValue::from_basic_value_enum(return_value, return_type))
            }
            None => None,
        };

        Ok(())
    }

    fn resolve_reference(&mut self, expression: &'a Node<Expression>) -> Result<PointerValue<'ctx>, Box<dyn IError>> {
        match &expression.value {
            Expression::Variable(name) => {
                let (ptr, _) = self.get_variable(name.as_str())?;
                Ok(ptr)
            }

            other => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Cannot pass expression '{:?}' by reference.", other),
                expression.position,
            ))),
        }
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

    fn f64_type(&self) -> FloatType<'ctx> {
        self.context.f64_type()
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

    fn read_last_value(&mut self) -> Result<LlvmValue<'ctx>, Box<dyn IError>> {
        self.last_value.take().ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                String::from("No value produced where it is needed."),
                self.position,
            )) as Box<dyn IError>
        })
    }

    fn get_variable(&self, name: &str) -> Result<(PointerValue<'ctx>, Type), Box<dyn IError>> {
        self.variables.get(name).cloned().ok_or_else(|| {
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

    fn build_binary_op<F>(&mut self, lhs: &'a Node<Expression>, rhs: &'a Node<Expression>, op: F, position: Position) -> Result<(), Box<dyn IError>>
    where
        F: Fn(&Builder<'ctx>, &LibcFunctions<'ctx>, LlvmValue<'ctx>, LlvmValue<'ctx>, Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>>,
    {
        self.visit_expression(lhs)?;
        let left_value = self.read_last_value()?;

        self.visit_expression(rhs)?;
        let right_value = self.read_last_value()?;

        let value = op(&self.builder, &self.libc, left_value, right_value, position)?;

        self.last_value = Some(value);

        Ok(())
    }

    fn build_unary_op<F>(&mut self, value: &'a Node<Expression>, op: F, position: Position) -> Result<(), Box<dyn IError>>
    where
        F: Fn(&Builder<'ctx>, &LibcFunctions<'ctx>, LlvmValue<'ctx>, Position) -> Result<LlvmValue<'ctx>, Box<dyn IError>>,
    {
        self.visit_expression(value)?;
        let computed_value = self.read_last_value()?;

        let value = op(&self.builder, &self.libc, computed_value, position)?;

        self.last_value = Some(value);

        Ok(())
    }

    fn branch_if_no_terminator(&mut self, target: BasicBlock<'ctx>, position: Position) -> Result<(), Box<dyn IError>> {
        let current_block = self.builder.get_insert_block().expect("builder should be positioned inside a block");

        if current_block.get_terminator().is_none() {
            self.builder
                .build_unconditional_branch(target)
                .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), position)) as Box<dyn IError>)?;
        }

        Ok(())
    }
}

impl<'a, 'ctx> Visitor<'a> for Compiler<'a, 'ctx> {
    fn visit_program(&mut self, program: &'a Program) -> Result<(), Box<dyn IError>> {
        for statement in &program.statements {
            self.visit_statement(statement)?;
        }

        Ok(())
    }

    fn visit_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        self.position = expression.position;

        match &expression.value {
            Expression::FunctionCall { identifier, arguments } => self.build_function_call(identifier, arguments, expression.position),
            Expression::Literal(literal) => self.visit_literal(literal),
            Expression::Variable(variable) => self.visit_variable(variable, expression.position),
            Expression::BooleanNegation(expr) => self.build_unary_op(expr, LlvmAlu::boolean_negate, expression.position),
            Expression::ArithmeticNegation(expr) => self.build_unary_op(expr, LlvmAlu::arithmetic_negate, expression.position),
            Expression::Addition(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::add, expression.position),
            Expression::Subtraction(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::subtract, expression.position),
            Expression::Multiplication(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::multiplication, expression.position),
            Expression::Division(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::division, expression.position),
            Expression::Modulo(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::modulo, expression.position),
            Expression::Concatenation(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::concatenation, expression.position),
            Expression::Alternative(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::alternative, expression.position),
            Expression::Greater(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::greater, expression.position),
            Expression::GreaterEqual(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::greater_or_equal, expression.position),
            Expression::Less(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::less, expression.position),
            Expression::LessEqual(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::less_or_equal, expression.position),
            Expression::Equal(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::equal, expression.position),
            Expression::NotEqual(lhs, rhs) => self.build_binary_op(lhs, rhs, LlvmAlu::not_equal, expression.position),
            Expression::Casting { value, to_type } => {
                self.visit_expression(value)?;
                let source_value = self.read_last_value()?;

                self.last_value = Some(LlvmAlu::cast_to_type(
                    &self.builder,
                    &self.libc,
                    source_value,
                    &to_type.value,
                    expression.position,
                )?);

                Ok(())
            }

            other => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Compiling expression '{:?}' is not yet supported.", other),
                expression.position,
            ))),
        }
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

                    let format_str = self
                        .builder
                        .build_global_string_ptr("%s\n", "fmt")
                        .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                    self.builder
                        .build_call(
                            self.libc.printf_fn,
                            &[format_str.as_pointer_value().into(), text_value.as_basic_value_enum().into()],
                            "printf_call",
                        )
                        .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                    Ok(())
                }

                _ => self.build_function_call(identifier, arguments, statement.position),
            },
            Statement::Declaration { var_type, identifier, value } => {
                let llvm_type = LlvmValue::type_to_basic_type_enum(&var_type.value, self.context).ok_or_else(|| {
                    Box::new(CompilerError::at(
                        ErrorSeverity::HIGH,
                        format!("Compiling declarations of type '{:?}' is not yet supported.", var_type.value),
                        statement.position,
                    )) as Box<dyn IError>
                })?;

                let ptr = self
                    .builder
                    .build_alloca(llvm_type, identifier.value.as_str())
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                if let Some(val_expr) = value {
                    self.visit_expression(val_expr)?;
                    let init_value = self.read_last_value()?;
                    self.builder
                        .build_store(ptr, init_value.as_basic_value_enum())
                        .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;
                }

                self.variables.insert(identifier.value.clone(), (ptr, var_type.value.clone()));

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

                let (ptr, _var_type) = self.get_variable(identifier.value.as_str())?;
                self.builder
                    .build_store(ptr, new_value.as_basic_value_enum())
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
                let cond_value = self.read_last_value()?.into_int_value(statement.position)?;
                self.builder
                    .build_conditional_branch(cond_value, body_block, after_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                self.builder.position_at_end(body_block);
                self.visit_block(block)?;

                if let Some(assign) = assignment {
                    self.visit_statement(assign)?;
                }

                // TODO: `break`/`continue` będą wymagały osobnego stosu bloków docelowych.
                self.branch_if_no_terminator(cond_block, statement.position)?;

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
                let cond_value = self.read_last_value()?.into_int_value(statement.position)?;
                self.builder
                    .build_conditional_branch(cond_value, true_block, false_block.unwrap_or(after_block))
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                self.builder.position_at_end(true_block);
                self.visit_block(if_block)?;
                self.branch_if_no_terminator(after_block, statement.position)?;

                if let Some(b) = false_block {
                    self.builder.position_at_end(b);
                    self.visit_block(&else_block.as_ref().unwrap())?;
                    self.branch_if_no_terminator(after_block, statement.position)?;
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
                let cond_value = self.read_last_value()?.into_int_value(statement.position)?;
                self.builder
                    .build_conditional_branch(cond_value, body_block, after_block)
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>)?;

                self.builder.position_at_end(body_block);
                self.visit_block(block)?;
                self.branch_if_no_terminator(after_block, statement.position)?;

                self.builder.position_at_end(after_block);

                Ok(())
            }
            Statement::Return(value) => {
                match value {
                    Some(expr) => {
                        self.visit_expression(expr)?;
                        let return_value = self.read_last_value()?;

                        self.builder.build_return(Some(&return_value.as_basic_value_enum())).map_err(|err| {
                            Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>
                        })?;
                    }
                    None => {
                        self.builder.build_return(None).map_err(|err| {
                            Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), statement.position)) as Box<dyn IError>
                        })?;
                    }
                }

                Ok(())
            }

            other => Err(Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Compiling statement '{:?}' is not yet supported.", other),
                statement.position,
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
                self.last_value = Some(LlvmValue::I64(const_value));
                Ok(())
            }
            Literal::F64(value) => {
                let const_value = self.f64_type().const_float(*value);
                self.last_value = Some(LlvmValue::F64(const_value));
                Ok(())
            }
            Literal::String(value) => {
                let string_value = self
                    .builder
                    .build_global_string_ptr(value.as_str(), "str")
                    .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), self.position)) as Box<dyn IError>)?;

                self.last_value = Some(LlvmValue::Str(string_value.as_pointer_value()));

                Ok(())
            }

            Literal::True => {
                self.last_value = Some(LlvmValue::Bool(self.context.bool_type().const_int(1, false)));
                Ok(())
            }

            Literal::False => {
                self.last_value = Some(LlvmValue::Bool(self.context.bool_type().const_int(0, false)));
                Ok(())
            }
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
        let (ptr, var_type) = self.get_variable(variable.as_str())?;
        let llvm_type = LlvmValue::type_to_basic_type_enum(&var_type, self.context).ok_or_else(|| {
            Box::new(CompilerError::at(
                ErrorSeverity::HIGH,
                format!("Compiling variables of type '{:?}' is not yet supported.", var_type),
                position,
            )) as Box<dyn IError>
        })?;

        let raw_value = self
            .builder
            .build_load(llvm_type, ptr, variable.as_str())
            .map_err(|err| Box::new(CompilerError::at(ErrorSeverity::HIGH, err.to_string(), position)) as Box<dyn IError>)?;

        self.last_value = Some(LlvmValue::from_basic_value_enum(raw_value, &var_type));

        Ok(())
    }
}
