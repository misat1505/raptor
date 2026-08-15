use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    backend::interpreter::{alu::ALU, stack::Stack, value::Value},
    backend::std_functions::std_functions::StdFunction,
    common::{
        errors::{ComputationError, ErrorSeverity, IError, InterpreterError},
        position::Position,
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::{
        Argument, Block, Expression, FunctionDeclaration, Literal, Node, Parameter, PassedBy, Program, Statement, SwitchCase, SwitchExpression,
    },
};

#[derive(Debug, PartialEq)]
enum AbortState {
    Break,
    Continue,
    Return,
}

pub struct Interpreter<'a> {
    program: &'a Program,
    stack: Stack<'a>,
    last_result: Option<Value>,
    abort_state: Option<AbortState>,
    position: Position,
    last_arguments: Vec<Rc<RefCell<Value>>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a Program) -> Self {
        Interpreter {
            program,
            stack: Stack::new(),
            abort_state: None,
            last_result: None,
            position: Position {
                filename: None,
                line: 0,
                column: 0,
                offset: 0,
            },
            last_arguments: vec![],
        }
    }

    pub fn interpret(&mut self) -> Result<(), Box<dyn IError>> {
        if let Some((name, function)) = self.program.extern_functions.iter().next() {
            return Err(Box::new(InterpreterError::at(
                ErrorSeverity::HIGH,
                format!("Extern function `{}` cannot be used in interpretation mode.", name),
                function.position,
            )));
        }

        self.visit_program(self.program)
    }

    fn read_last_result(&mut self) -> Result<Value, Box<dyn IError>> {
        {
            let this = self.last_result.take();
            match this {
                Some(v) => Ok(v),
                None => Err((|| {
                    Box::new(InterpreterError::at(
                        ErrorSeverity::HIGH,
                        String::from("No value produced where it is needed."),
                        self.position,
                    ))
                })()),
            }
        }
    }

    fn evaluate_binary_op<F>(&mut self, lhs: &'a Box<Node<Expression>>, rhs: &'a Box<Node<Expression>>, op: F) -> Result<(), Box<dyn IError>>
    where
        F: Fn(Value, Value) -> Result<Value, ComputationError>,
    {
        self.visit_expression(lhs)?;
        let left_value = self.read_last_result()?;

        self.visit_expression(rhs)?;
        let right_value = self.read_last_result()?;

        let value = op(left_value, right_value)
            .map_err(|err| Box::new(ComputationError::at(ErrorSeverity::HIGH, err.message(), self.position)) as Box<dyn IError>)?;

        self.last_result = Some(value);

        Ok(())
    }

    fn evaluate_unary_op<F>(&mut self, value: &'a Box<Node<Expression>>, op: F) -> Result<(), Box<dyn IError>>
    where
        F: Fn(Value) -> Result<Value, ComputationError>,
    {
        self.visit_expression(value)?;
        let computed_value = self.read_last_result()?;

        let value =
            op(computed_value).map_err(|err| Box::new(ComputationError::at(ErrorSeverity::HIGH, err.message(), self.position)) as Box<dyn IError>)?;

        self.last_result = Some(value);

        Ok(())
    }
}

impl<'a> Visitor<'a> for Interpreter<'a> {
    fn visit_program(&mut self, program: &'a Program) -> Result<(), Box<dyn IError>> {
        for statement in &program.statements {
            self.visit_statement(statement)?;

            if let Some(abort) = &self.abort_state {
                return match abort {
                    AbortState::Break => Err(Box::new(InterpreterError::at(
                        ErrorSeverity::HIGH,
                        String::from("Break called outside 'for' or 'switch'."),
                        self.position,
                    ))),
                    AbortState::Continue => Err(Box::new(InterpreterError::at(
                        ErrorSeverity::HIGH,
                        String::from("Continue called outside 'for' or 'while'."),
                        self.position,
                    ))),
                    AbortState::Return => Err(Box::new(InterpreterError::at(
                        ErrorSeverity::HIGH,
                        String::from("Return called outside a function."),
                        self.position,
                    ))),
                };
            }
        }

        Ok(())
    }

    fn visit_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        self.position = expression.position;

        match &expression.value {
            Expression::Casting { value, to_type } => {
                self.visit_expression(value)?;

                let computed_value = self.read_last_result()?;

                let value = ALU::cast_to_type(computed_value, &to_type.value)
                    .map_err(|err| Box::new(ComputationError::at(ErrorSeverity::HIGH, err.message(), expression.position)) as Box<dyn IError>)?;

                self.last_result = Some(value);
            }

            Expression::BooleanNegation(value) => self.evaluate_unary_op(value, ALU::boolean_negate)?,
            Expression::ArithmeticNegation(value) => self.evaluate_unary_op(value, ALU::arithmetic_negate)?,
            Expression::Addition(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::add)?,
            Expression::Subtraction(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::subtract)?,
            Expression::Multiplication(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::multiplication)?,
            Expression::Division(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::division)?,
            Expression::Modulo(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::modulo)?,
            Expression::Alternative(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::alternative)?,
            Expression::Concatenation(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::concatenation)?,
            Expression::Greater(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::greater)?,
            Expression::GreaterEqual(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::greater_or_equal)?,
            Expression::Less(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::less)?,
            Expression::LessEqual(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::less_or_equal)?,
            Expression::Equal(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::equal)?,
            Expression::NotEqual(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, ALU::not_equal)?,
            Expression::Literal(literal) => self.visit_literal(literal)?,
            Expression::Vector(vector) => self.visit_vector_literal(vector)?,
            Expression::Variable(variable) => self.visit_variable(variable, expression.position)?,
            Expression::FunctionCall { identifier, arguments } => self.call_function(identifier, arguments)?,

            Expression::Index { collection, index } => self.visit_index(collection, index)?,
        }

        Ok(())
    }

    fn visit_statement(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        self.position = statement.position;

        match &statement.value {
            Statement::FunctionCall { identifier, arguments } => {
                self.call_function(identifier, arguments)?;
            }

            Statement::Declaration { var_type, identifier, value } => {
                self.visit_type(var_type)?;

                let mut computed_value = match value {
                    Some(val) => {
                        self.visit_expression(val)?;

                        self.read_last_result().map_err(|_| {
                            Box::new(InterpreterError::at(
                                ErrorSeverity::HIGH,
                                format!("Cannot declare variable '{}' with no value.", identifier.value),
                                statement.position,
                            )) as Box<dyn IError>
                        })?
                    }

                    None => Value::default_value(&var_type.value)
                        .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), statement.position)) as Box<dyn IError>)?,
                };

                match (&var_type.value, &computed_value) {
                    (Type::I64, Value::I64(_)) | (Type::F64, Value::F64(_)) | (Type::Str, Value::String(_)) | (Type::Bool, Value::Bool(_)) => {}

                    (Type::Vector(declared_inner), Value::Vector { values, .. }) => {
                        for value in values.borrow().iter() {
                            let actual_type = value.borrow().to_type();

                            if actual_type != *declared_inner.as_ref() {
                                return Err(Box::new(InterpreterError::expected_found(
                                    ErrorSeverity::HIGH,
                                    format!("Cannot assign value to vector '{}'.", identifier.value),
                                    format!("{:?}", declared_inner.as_ref()),
                                    format!("{:?}", actual_type),
                                    statement.position,
                                )));
                            }
                        }

                        if let Value::Vector { kind: _, ref values } = computed_value {
                            if values.borrow().is_empty() {
                                computed_value = Value::Vector {
                                    kind: Box::new(var_type.value.clone()),
                                    values: values.clone(),
                                };
                            }
                        }
                    }

                    (declared_type, computed_value) => {
                        return Err(Box::new(InterpreterError::expected_found(
                            ErrorSeverity::HIGH,
                            format!("Cannot assign value to variable '{}'.", identifier.value),
                            format!("{:?}", declared_type),
                            format!("{:?}", computed_value.to_type()),
                            statement.position,
                        )));
                    }
                }

                self.stack
                    .declare_variable(identifier.value.as_str(), Rc::new(RefCell::new(computed_value)))
                    .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), statement.position)) as Box<dyn IError>)?;
            }

            Statement::Assignment { identifier, value, indices } => {
                if indices.is_empty() {
                    self.visit_expression(value)?;

                    let value = self.read_last_result().map_err(|_| {
                        Box::new(InterpreterError::at(
                            ErrorSeverity::HIGH,
                            format!("Cannot assign no value to variable '{}'.", identifier.value),
                            statement.position,
                        )) as Box<dyn IError>
                    })?;

                    self.stack
                        .assign_variable(identifier.value.as_str(), Rc::new(RefCell::new(value)))
                        .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), statement.position)) as Box<dyn IError>)?;
                } else {
                    self.visit_index_assignment(identifier, indices, value)?;
                }
            }

            Statement::Conditional {
                condition,
                if_block,
                else_block,
            } => {
                self.visit_expression(condition)?;

                let computed_condition = self.read_last_result()?;

                let boolean_value = computed_condition
                    .try_into_bool()
                    .map_err(|_| self.condition_error(computed_condition, "if statement"))?;

                if boolean_value {
                    self.visit_block(if_block)?;
                } else if let Some(else_blk) = else_block {
                    self.visit_block(else_blk)?;
                }
            }

            Statement::WhileLoop { condition, block } => {
                self.visit_expression(condition)?;

                let mut computed_condition = self.read_last_result()?;

                let mut boolean_value = computed_condition
                    .try_into_bool()
                    .map_err(|_| self.condition_error(computed_condition, "while statement"))?;

                while boolean_value {
                    self.visit_block(block)?;

                    if let Some(abort) = &self.abort_state {
                        match abort {
                            AbortState::Break => {
                                self.abort_state = None;
                                break;
                            }
                            AbortState::Return => {
                                break;
                            }
                            AbortState::Continue => {
                                self.abort_state = None;
                            }
                        }
                    }

                    self.visit_expression(condition)?;

                    computed_condition = self.read_last_result()?;

                    boolean_value = computed_condition
                        .try_into_bool()
                        .map_err(|_| self.condition_error(computed_condition, "while statement"))?;
                }
            }

            Statement::ForLoop {
                declaration,
                condition,
                assignment,
                block,
            } => {
                self.stack.push_scope();

                if let Some(decl) = declaration {
                    self.visit_statement(decl)?;
                }

                self.visit_expression(condition)?;

                let mut computed_condition = self.read_last_result()?;

                let mut boolean_value = computed_condition
                    .try_into_bool()
                    .map_err(|_| self.condition_error(computed_condition, "for statement"))?;

                while boolean_value {
                    self.visit_block(block)?;

                    if let Some(abort) = &self.abort_state {
                        match abort {
                            AbortState::Break => {
                                self.abort_state = None;
                                break;
                            }
                            AbortState::Return => {
                                break;
                            }
                            AbortState::Continue => {
                                self.abort_state = None;
                            }
                        }
                    }

                    if let Some(assign) = assignment {
                        self.visit_statement(assign)?;
                    }

                    self.visit_expression(condition)?;

                    computed_condition = self.read_last_result()?;

                    boolean_value = computed_condition
                        .try_into_bool()
                        .map_err(|_| self.condition_error(computed_condition, "for statement"))?;
                }

                self.stack.pop_scope();
            }

            Statement::Switch { expressions, cases } => {
                self.stack.push_scope();

                for expr in expressions {
                    self.visit_switch_expression(expr)?;
                }

                for case in cases {
                    self.visit_switch_case(case)?;

                    if let Some(abort) = &self.abort_state {
                        match abort {
                            AbortState::Break => {
                                self.abort_state = None;
                                break;
                            }
                            AbortState::Return => {
                                break;
                            }
                            AbortState::Continue => {
                                break;
                            }
                        }
                    }
                }

                self.stack.pop_scope();
            }

            Statement::Return(value) => {
                let mut returned_value = None;

                if let Some(val) = value {
                    self.visit_expression(val)?;
                    returned_value = Some(self.read_last_result()?);
                }

                self.abort_state = Some(AbortState::Return);
                self.last_result = returned_value;
            }

            Statement::Break => {
                self.abort_state = Some(AbortState::Break);
            }

            Statement::Continue => {
                self.abort_state = Some(AbortState::Continue);
            }
        }

        Ok(())
    }

    fn visit_argument(&mut self, argument: &'a Node<Argument>) -> Result<(), Box<dyn IError>> {
        self.visit_expression(&argument.value.value)?;
        Ok(())
    }

    fn visit_block(&mut self, block: &'a Node<Block>) -> Result<(), Box<dyn IError>> {
        self.stack.push_scope();

        for statement in &block.value.0 {
            if let Some(_) = self.abort_state {
                break;
            }

            self.visit_statement(statement)?;
        }

        self.stack.pop_scope();

        Ok(())
    }

    fn visit_parameter(&mut self, parameter: &'a Node<Parameter>) -> Result<(), Box<dyn IError>> {
        self.visit_type(&parameter.value.parameter_type)?;
        Ok(())
    }

    fn visit_switch_case(&mut self, switch_case: &'a Node<SwitchCase>) -> Result<(), Box<dyn IError>> {
        self.visit_expression(&switch_case.value.condition)?;

        let computed_value = self.read_last_result()?;

        let boolean_value = computed_value
            .try_into_bool()
            .map_err(|_| self.condition_error(computed_value, "switch case"))?;

        if boolean_value {
            self.visit_block(&switch_case.value.block)?;
        }

        Ok(())
    }

    fn visit_switch_expression(&mut self, switch_expression: &'a Node<SwitchExpression>) -> Result<(), Box<dyn IError>> {
        if let Some(alias) = &switch_expression.value.alias {
            self.visit_expression(&switch_expression.value.expression)?;

            let computed_value = self.read_last_result()?;

            self.stack
                .declare_variable(alias.value.as_str(), Rc::new(RefCell::new(computed_value)))
                .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), switch_expression.position)) as Box<dyn IError>)?;
        }

        Ok(())
    }

    fn visit_type(&mut self, _node_type: &Node<Type>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_literal(&mut self, literal: &Literal) -> Result<(), Box<dyn IError>> {
        let value = match literal {
            Literal::F64(value) => Value::F64(*value),
            Literal::I64(value) => Value::I64(*value),
            Literal::String(value) => Value::String(value.to_string()),
            Literal::False => Value::Bool(false),
            Literal::True => Value::Bool(true),
        };

        self.last_result = Some(value);

        Ok(())
    }

    fn visit_vector_literal(&mut self, expressions: &'a Vec<Box<Node<Expression>>>) -> Result<(), Box<dyn IError>> {
        let values = Rc::new(RefCell::new(Vec::new()));

        for expression in expressions {
            self.visit_expression(expression)?;

            values.borrow_mut().push(Rc::new(RefCell::new(self.read_last_result()?)));
        }

        let kind = if let Some(first) = values.borrow().first() {
            Box::new(Type::Vector(Box::new(first.borrow().to_type())))
        } else {
            Box::new(Type::Void)
        };

        self.last_result = Some(Value::Vector { kind, values });

        Ok(())
    }

    fn visit_variable(&mut self, variable: &'a String, position: Position) -> Result<(), Box<dyn IError>> {
        let value = self
            .stack
            .get_variable(variable.as_str())
            .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), position)) as Box<dyn IError>)?;

        self.last_result = Some(value.borrow().clone());

        Ok(())
    }
}

impl<'a> Interpreter<'a> {
    #[allow(dead_code)]
    pub fn stack(&mut self) -> Stack<'_> {
        self.stack.clone()
    }

    fn condition_error(&self, value: Value, place: &'a str) -> Box<dyn IError> {
        Box::new(InterpreterError::expected_found(
            ErrorSeverity::HIGH,
            format!("Condition in '{}' has to evaluate to a valid boolean.", place),
            format!("{:?}", Type::Bool),
            format!("{:?}", value.to_type()),
            self.position,
        ))
    }

    fn execute_std_function(
        std_function: &StdFunction,
        arguments: &Vec<Rc<RefCell<Value>>>,
        position: Position,
    ) -> Result<Option<Value>, Box<dyn IError>> {
        (std_function.execute)(arguments)
            .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), position)) as Box<dyn IError>)
    }

    fn call_function(&mut self, identifier: &Node<String>, arguments: &'a Vec<Box<Node<Argument>>>) -> Result<(), Box<dyn IError>> {
        let name = identifier.value.as_str();

        let mut args: Vec<Rc<RefCell<Value>>> = vec![];

        for arg in arguments {
            self.position = arg.position;

            match arg.value.passed_by {
                PassedBy::Value => {
                    self.visit_expression(&arg.value.value)?;

                    let value = self.read_last_result()?;

                    match value {
                        Value::Vector { ref kind, ref values } => {
                            let shallow_copy = Rc::new(RefCell::new(values.borrow().iter().map(Rc::clone).collect::<Vec<_>>()));
                            let shallow_copy_vector = Value::Vector {
                                kind: kind.clone(),
                                values: shallow_copy,
                            };
                            args.push(Rc::new(RefCell::new(shallow_copy_vector)));
                        }
                        _ => args.push(Rc::new(RefCell::new(value))),
                    }
                }

                PassedBy::Reference => {
                    let reference = self.resolve_reference(&arg.value.value)?;
                    args.push(reference);
                }
            }
        }

        self.last_arguments = args;

        if let Some(std_function) = self.program.std_functions.get(name) {
            if arguments.len() != std_function.params.len() {
                self.last_arguments.clear();

                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    format!("invalid number of arguments for function `{}`", name),
                    std_function.params.len().to_string(),
                    arguments.len().to_string(),
                    identifier.position,
                )));
            }

            if let Some(return_value) = Self::execute_std_function(std_function, &self.last_arguments, identifier.position)? {
                self.last_result = Some(return_value);
            }

            self.last_arguments.clear();

            return Ok(());
        }

        if let Some(function_declaration) = self.program.functions.get(name) {
            let expected_arguments = function_declaration.value.parameters.len();

            if arguments.len() != expected_arguments {
                self.last_arguments.clear();

                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    format!("invalid number of arguments for function `{}`", name),
                    expected_arguments.to_string(),
                    arguments.len().to_string(),
                    identifier.position,
                )));
            }

            self.execute_function(&function_declaration.value)?;

            if let Some(AbortState::Return) = self.abort_state {
                self.abort_state = None;
            }

            self.last_arguments.clear();

            return Ok(());
        }

        self.last_arguments.clear();

        Err(Box::new(InterpreterError::at(
            ErrorSeverity::HIGH,
            format!("use of undeclared function `{}`", name),
            identifier.position,
        )))
    }

    fn execute_function(&mut self, function_declaration: &'a FunctionDeclaration) -> Result<(), Box<dyn IError>> {
        let name = function_declaration.identifier.value.as_str();

        let statements = &function_declaration.block.value.0;

        self.stack.push_stack_frame().map_err(|err| {
            Box::new(InterpreterError::at(
                ErrorSeverity::HIGH,
                err.message(),
                function_declaration.identifier.position,
            )) as Box<dyn IError>
        })?;

        for idx in 0..self.last_arguments.len() {
            let parameter = function_declaration.parameters.get(idx).ok_or_else(|| {
                Box::new(InterpreterError::at(
                    ErrorSeverity::HIGH,
                    format!("Invalid parameter index {} while calling function '{}'.", idx, name),
                    function_declaration.identifier.position,
                )) as Box<dyn IError>
            })?;

            let desired_type = &parameter.value.parameter_type.value;
            let param_name = &parameter.value.identifier.value;
            let value = &self.last_arguments[idx];

            if !desired_type.accepts(&value.borrow()) {
                self.stack.pop_stack_frame();

                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    format!("Function '{}' parameter '{}': wrong argument type.", name, param_name),
                    format!("{:?}", desired_type),
                    format!("{:?}", value.borrow().to_type()),
                    parameter.position,
                )));
            }

            self.stack
                .declare_variable(param_name.as_str(), Rc::clone(value))
                .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), parameter.position)) as Box<dyn IError>)?;
        }

        for statement in statements {
            if let Some(AbortState::Return) = self.abort_state {
                self.abort_state = None;
                break;
            }

            self.visit_statement(statement)?;

            if let Some(abort) = &self.abort_state {
                match abort {
                    AbortState::Break => {
                        self.stack.pop_stack_frame();

                        return Err(Box::new(InterpreterError::at(
                            ErrorSeverity::HIGH,
                            String::from("Break called outside 'for' or 'switch'."),
                            statement.position,
                        )));
                    }
                    AbortState::Continue => {
                        self.stack.pop_stack_frame();

                        return Err(Box::new(InterpreterError::at(
                            ErrorSeverity::HIGH,
                            String::from("Continue called outside 'for' or 'while'."),
                            statement.position,
                        )));
                    }
                    _ => {}
                }
            }
        }

        match &self.last_result {
            None if function_declaration.return_type.value == Type::Void => {}

            Some(value) if function_declaration.return_type.value.accepts(value) => {}

            result => {
                let result_type = match result {
                    None => Type::Void,
                    Some(value) => value.to_type(),
                };

                self.stack.pop_stack_frame();

                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    format!("Bad return type from function '{}'.", name),
                    format!("{:?}", function_declaration.return_type.value),
                    format!("{:?}", result_type),
                    function_declaration.return_type.position,
                )));
            }
        }

        self.stack.pop_stack_frame();

        Ok(())
    }

    fn visit_index(&mut self, collection: &'a Node<Expression>, index: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        self.visit_expression(collection)?;

        let collection_value = self.read_last_result()?;

        let values = match collection_value {
            Value::Vector { values, .. } => values,

            other => {
                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    String::from("Cannot index into this value."),
                    String::from("Vector"),
                    format!("{:?}", other.to_type()),
                    collection.position,
                )));
            }
        };

        self.visit_expression(index)?;

        let index_value = self.read_last_result()?;

        let idx = match index_value {
            Value::I64(i) if i >= 0 => i as usize,

            other => {
                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    String::from("Array index must be a non-negative i64."),
                    String::from("I64"),
                    format!("{:?}", other.to_type()),
                    index.position,
                )));
            }
        };

        let borrowed = values.borrow();

        let element_cell = borrowed.get(idx).ok_or_else(|| {
            Box::new(InterpreterError::at(
                ErrorSeverity::HIGH,
                format!("Index {} out of bounds.", idx),
                index.position,
            )) as Box<dyn IError>
        })?;

        self.last_result = Some(element_cell.borrow().clone());

        Ok(())
    }

    fn visit_index_assignment(
        &mut self,
        identifier: &'a Node<String>,
        indices: &'a Vec<Node<Expression>>,
        value: &'a Node<Expression>,
    ) -> Result<(), Box<dyn IError>> {
        let var_ref = self
            .stack
            .get_variable(identifier.value.as_str())
            .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), identifier.position)) as Box<dyn IError>)?;

        let (last_index_expr, earlier_indices) = indices.split_last().expect("parser guarantees at least one index in IndexAssignment");

        let mut current_values = match &*var_ref.borrow() {
            Value::Vector { values, .. } => values.clone(),

            other => {
                return Err(Box::new(InterpreterError::expected_found(
                    ErrorSeverity::HIGH,
                    format!("Cannot index into variable '{}'.", identifier.value),
                    String::from("Vector"),
                    format!("{:?}", other.to_type()),
                    identifier.position,
                )));
            }
        };

        for index_expr in earlier_indices {
            self.visit_expression(index_expr)?;

            let idx = self.expect_index()?;

            let borrowed = current_values.borrow();

            let next_cell = borrowed.get(idx).ok_or_else(|| {
                Box::new(InterpreterError::at(
                    ErrorSeverity::HIGH,
                    format!("Index {} out of bounds.", idx),
                    index_expr.position,
                )) as Box<dyn IError>
            })?;

            let next_values = match &*next_cell.borrow() {
                Value::Vector { values, .. } => values.clone(),

                other => {
                    return Err(Box::new(InterpreterError::expected_found(
                        ErrorSeverity::HIGH,
                        String::from("Cannot index into this value."),
                        String::from("Vector"),
                        format!("{:?}", other.to_type()),
                        index_expr.position,
                    )));
                }
            };

            drop(borrowed);

            current_values = next_values;
        }

        self.visit_expression(last_index_expr)?;

        let idx = self.expect_index()?;

        self.visit_expression(value)?;

        let new_value = self.read_last_result()?;

        let mut borrowed = current_values.borrow_mut();

        let target_cell = borrowed.get_mut(idx).ok_or_else(|| {
            Box::new(InterpreterError::at(
                ErrorSeverity::HIGH,
                format!("Index {} out of bounds.", idx),
                last_index_expr.position,
            )) as Box<dyn IError>
        })?;

        *target_cell = Rc::new(RefCell::new(new_value));

        Ok(())
    }

    fn expect_index(&mut self) -> Result<usize, Box<dyn IError>> {
        match self.read_last_result()? {
            Value::I64(i) if i >= 0 => Ok(i as usize),

            other => Err(Box::new(InterpreterError::expected_found(
                ErrorSeverity::HIGH,
                String::from("Array index must be a non-negative i64."),
                String::from("I64"),
                format!("{:?}", other.to_type()),
                self.position,
            ))),
        }
    }

    fn resolve_reference(&mut self, expression: &'a Node<Expression>) -> Result<Rc<RefCell<Value>>, Box<dyn IError>> {
        match &expression.value {
            Expression::Variable(var_name) => self
                .stack
                .get_variable(var_name.as_str())
                .map(Rc::clone)
                .map_err(|err| Box::new(InterpreterError::at(ErrorSeverity::HIGH, err.message(), expression.position)) as Box<dyn IError>),

            Expression::Index { collection, index } => {
                let collection_ref = self.resolve_reference(collection)?;

                self.visit_expression(index)?;

                let idx = self.expect_index()?;

                let values = match &*collection_ref.borrow() {
                    Value::Vector { values, .. } => values.clone(),

                    other => {
                        return Err(Box::new(InterpreterError::expected_found(
                            ErrorSeverity::HIGH,
                            String::from("Cannot index into this value."),
                            String::from("Vector"),
                            format!("{:?}", other.to_type()),
                            collection.position,
                        )));
                    }
                };

                let borrowed = values.borrow();

                let element_cell = borrowed.get(idx).ok_or_else(|| {
                    Box::new(InterpreterError::at(
                        ErrorSeverity::HIGH,
                        format!("Index {} out of bounds.", idx),
                        index.position,
                    )) as Box<dyn IError>
                })?;

                Ok(Rc::clone(element_cell))
            }

            _ => Err(Box::new(InterpreterError::at(
                ErrorSeverity::HIGH,
                String::from("Cannot pass this kind of expression by reference — expected a variable or indexed value."),
                expression.position,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::frontend::ast::FunctionDeclaration;

    use super::*;

    fn default_position() -> Position {
        Position {
            filename: None,
            line: 0,
            column: 0,
            offset: 0,
        }
    }

    fn setup_program() -> Program {
        Program {
            statements: vec![],
            functions: HashMap::new(),
            std_functions: HashMap::new(),
            extern_functions: HashMap::new(),
        }
    }

    fn create_interpreter<'a>(program: &'a Program) -> Interpreter<'a> {
        Interpreter::new(program)
    }

    macro_rules! test_node {
        ($value:expr) => {
            Node {
                value: $value,
                position: default_position(),
            }
        };
    }

    #[test]
    fn interpret_casting() {
        let ast = test_node!(Expression::Casting {
            value: Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
            to_type: test_node!(Type::F64),
        });

        let exp = Some(Value::F64(2.0));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_boolean_negation() {
        let ast = test_node!(Expression::BooleanNegation(Box::new(test_node!(Expression::Literal(Literal::False)))));

        let exp = Some(Value::Bool(true));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_arithmetic_negation() {
        let ast = test_node!(Expression::ArithmeticNegation(Box::new(test_node!(Expression::Literal(Literal::I64(5))))));

        let exp = Some(Value::I64(-5));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_addition() {
        let ast = test_node!(Expression::Addition(
            Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2))))
        ));

        let exp = Some(Value::I64(7));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_subtraction() {
        let ast = test_node!(Expression::Subtraction(
            Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2))))
        ));

        let exp = Some(Value::I64(3));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_multiplication() {
        let ast = test_node!(Expression::Multiplication(
            Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2))))
        ));

        let exp = Some(Value::I64(10));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_division() {
        let ast = test_node!(Expression::Division(
            Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(2))))
        ));

        let exp = Some(Value::I64(2));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_concatenation() {
        let ast = test_node!(Expression::Concatenation(
            Box::new(test_node!(Expression::Literal(Literal::True))),
            Box::new(test_node!(Expression::Literal(Literal::False)))
        ));

        let exp = Some(Value::Bool(false));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_alternative() {
        let ast = test_node!(Expression::Alternative(
            Box::new(test_node!(Expression::Literal(Literal::True))),
            Box::new(test_node!(Expression::Literal(Literal::False)))
        ));

        let exp = Some(Value::Bool(true));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_greater() {
        let ast = test_node!(Expression::Greater(
            Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(5))))
        ));

        let exp = Some(Value::Bool(false));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_greater_equal() {
        let ast = test_node!(Expression::GreaterEqual(
            Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(5))))
        ));

        let exp = Some(Value::Bool(true));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_less() {
        let ast = test_node!(Expression::Less(
            Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(5))))
        ));

        let exp = Some(Value::Bool(false));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }
    #[test]
    fn interpret_less_equal() {
        let ast = test_node!(Expression::LessEqual(
            Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(5))))
        ));

        let exp = Some(Value::Bool(true));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_equal() {
        let ast = test_node!(Expression::Equal(
            Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(5))))
        ));

        let exp = Some(Value::Bool(true));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_not_equal() {
        let ast = test_node!(Expression::NotEqual(
            Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(5))))
        ));

        let exp = Some(Value::Bool(false));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_literal() {
        let ast = test_node!(Expression::Literal(Literal::I64(5)));

        let exp = Some(Value::I64(5));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn interpret_variable() {
        let ast = test_node!(Expression::Variable(String::from("x")));

        let exp = Some(Value::I64(5));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(5))));

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, exp);
    }

    #[test]
    fn declare_variable() {
        // i64 x = 5;
        let ast = test_node!(Statement::Declaration {
            var_type: test_node!(Type::I64),
            identifier: test_node!(String::from("x")),
            value: Some(test_node!(Expression::Literal(Literal::I64(5)))),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_statement(&ast);
        assert_eq!(interpreter.stack.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(5))));
    }

    #[test]
    fn declare_variable_with_default_value() {
        // i64 x;
        let ast = test_node!(Statement::Declaration {
            var_type: test_node!(Type::I64),
            identifier: test_node!(String::from("x")),
            value: None,
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_statement(&ast);
        assert_eq!(interpreter.stack.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(0))));
    }

    #[test]
    fn declare_variable_bad_type() {
        // i64 x = false;
        let ast = test_node!(Statement::Declaration {
            var_type: test_node!(Type::I64),
            identifier: test_node!(String::from("x")),
            value: Some(test_node!(Expression::Literal(Literal::False))),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        assert!(interpreter.visit_statement(&ast).is_err());
    }

    #[test]
    fn redeclare_variable_fails() {
        let ast = test_node!(Statement::Declaration {
            var_type: test_node!(Type::I64),
            identifier: test_node!(String::from("x")),
            value: None,
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_statement(&ast);
        assert_eq!(interpreter.stack.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(0))));

        assert!(interpreter.visit_statement(&ast).is_err());
    }

    #[test]
    fn declare_with_none_value_fails() {
        // i64 x = print("hello world");
        let ast = test_node!(Statement::Declaration {
            var_type: test_node!(Type::I64),
            identifier: test_node!(String::from("x")),
            value: Some(test_node!(Expression::FunctionCall {
                identifier: test_node!(String::from("print")),
                arguments: vec![Box::new(test_node!(Argument {
                    value: test_node!(Expression::Literal(Literal::String(String::from("hello world")))),
                    passed_by: PassedBy::Value,
                })),],
            })),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        assert!(interpreter.visit_statement(&ast).is_err());
    }

    #[test]
    fn declare_with_bad_type_fails() {
        // i64 x = true;
        let ast = test_node!(Statement::Declaration {
            var_type: test_node!(Type::I64),
            identifier: test_node!(String::from("x")),
            value: Some(test_node!(Expression::Literal(Literal::True))),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        assert!(interpreter.visit_statement(&ast).is_err());
    }

    #[test]
    fn assigns_to_variable() {
        // i64 x = 0;
        // x = 5;
        let ast = test_node!(Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Literal(Literal::I64(1))),
            indices: vec![]
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(0))));

        assert!(interpreter.visit_statement(&ast).is_ok());
        assert_eq!(interpreter.stack.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(1))));
    }

    #[test]
    fn assigns_bad_type_fails() {
        // i64 x = 0;
        // x = false;
        let ast = test_node!(Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Literal(Literal::False)),
            indices: vec![]
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(0))));

        assert!(interpreter.visit_statement(&ast).is_err());
    }

    #[test]
    fn assign_with_none_value_fails() {
        // x = print("hello world");
        let ast = test_node!(Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::FunctionCall {
                identifier: test_node!(String::from("print")),
                arguments: vec![Box::new(test_node!(Argument {
                    value: test_node!(Expression::Literal(Literal::String(String::from("hello world")))),
                    passed_by: PassedBy::Value,
                })),],
            }),
            indices: vec![]
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(0))));

        assert!(interpreter.visit_statement(&ast).is_err());
    }

    #[test]
    fn if_true_branch() {
        // i64 x = 0;
        // if (true) {x = 1;} else {x = 2;}
        let ast = test_node!(Statement::Conditional {
            condition: test_node!(Expression::Literal(Literal::True)),
            if_block: test_node!(Block(vec![test_node!(Statement::Assignment {
                identifier: test_node!(String::from("x")),
                value: test_node!(Expression::Literal(Literal::I64(1))),
                indices: vec![]
            }),])),
            else_block: Some(test_node!(Block(vec![test_node!(Statement::Assignment {
                identifier: test_node!(String::from("x")),
                value: test_node!(Expression::Literal(Literal::I64(2))),
                indices: vec![]
            }),]))),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(0))));

        assert!(interpreter.visit_statement(&ast).is_ok());
        assert_eq!(interpreter.stack.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(1))));
    }

    #[test]
    fn if_false_branch() {
        // i64 x = 0;
        // if (false) {x = 1;} else {x = 2;}
        let ast = test_node!(Statement::Conditional {
            condition: test_node!(Expression::Literal(Literal::False)),
            if_block: test_node!(Block(vec![test_node!(Statement::Assignment {
                identifier: test_node!(String::from("x")),
                value: test_node!(Expression::Literal(Literal::I64(1))),
                indices: vec![]
            }),])),
            else_block: Some(test_node!(Block(vec![test_node!(Statement::Assignment {
                identifier: test_node!(String::from("x")),
                value: test_node!(Expression::Literal(Literal::I64(2))),
                indices: vec![]
            }),]))),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(0))));

        assert!(interpreter.visit_statement(&ast).is_ok());
        assert_eq!(interpreter.stack.get_variable("x").unwrap().clone(), Rc::new(RefCell::new(Value::I64(2))));
    }

    #[test]
    fn if_bad_condition_type_fails() {
        // i64 x = 0;
        // if (2137) {}
        let ast = test_node!(Statement::Conditional {
            condition: test_node!(Expression::Literal(Literal::I64(2137))),
            if_block: test_node!(Block(vec![])),
            else_block: None,
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(0))));

        assert!(interpreter.visit_statement(&ast).is_err());
    }

    #[test]
    fn for_loop() {
        // i64 total = 0;
        // for (i64 i = 1; i <= 5; i = i + 1) {total = total + i;}
        let ast = test_node!(Statement::ForLoop {
            declaration: Some(Box::new(test_node!(Statement::Declaration {
                var_type: test_node!(Type::I64),
                identifier: test_node!(String::from("i")),
                value: Some(test_node!(Expression::Literal(Literal::I64(1)))),
            }))),
            condition: test_node!(Expression::LessEqual(
                Box::new(test_node!(Expression::Variable(String::from("i")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(5))))
            )),
            assignment: Some(Box::new(test_node!(Statement::Assignment {
                identifier: test_node!(String::from("i")),
                value: test_node!(Expression::Addition(
                    Box::new(test_node!(Expression::Variable(String::from("i")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(1))))
                )),
                indices: vec![]
            }))),
            block: test_node!(Block(vec![test_node!(Statement::Assignment {
                identifier: test_node!(String::from("total")),
                value: test_node!(Expression::Addition(
                    Box::new(test_node!(Expression::Variable(String::from("total")))),
                    Box::new(test_node!(Expression::Variable(String::from("i"))))
                )),
                indices: vec![]
            }),])),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("total", Rc::new(RefCell::new(Value::I64(0))));

        assert!(interpreter.visit_statement(&ast).is_ok());
        assert_eq!(
            interpreter.stack.get_variable("total").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(15)))
        );
    }

    #[test]
    fn for_loop_second_variant() {
        // i64 total = 0;
        // i64 i = 1;
        // for (;i <= 5;) {total = total + i; i = i + 1}
        let ast = test_node!(Statement::ForLoop {
            declaration: None,
            condition: test_node!(Expression::LessEqual(
                Box::new(test_node!(Expression::Variable(String::from("i")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(5))))
            )),
            assignment: None,
            block: test_node!(Block(vec![
                test_node!(Statement::Assignment {
                    identifier: test_node!(String::from("total")),
                    value: test_node!(Expression::Addition(
                        Box::new(test_node!(Expression::Variable(String::from("total")))),
                        Box::new(test_node!(Expression::Variable(String::from("i"))))
                    )),
                    indices: vec![]
                }),
                test_node!(Statement::Assignment {
                    identifier: test_node!(String::from("i")),
                    value: test_node!(Expression::Addition(
                        Box::new(test_node!(Expression::Variable(String::from("i")))),
                        Box::new(test_node!(Expression::Literal(Literal::I64(1))))
                    )),
                    indices: vec![]
                }),
            ])),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("total", Rc::new(RefCell::new(Value::I64(0))));
        let _ = interpreter.stack.declare_variable("i", Rc::new(RefCell::new(Value::I64(1))));

        assert!(interpreter.visit_statement(&ast).is_ok());
        assert_eq!(
            interpreter.stack.get_variable("total").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(15)))
        );
    }

    #[test]
    fn for_loop_bad_condition_type() {
        // for (;1;) {}
        let ast = test_node!(Statement::ForLoop {
            declaration: None,
            condition: test_node!(Expression::Literal(Literal::I64(1))),
            assignment: None,
            block: test_node!(Block(vec![])),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        assert!(interpreter.visit_statement(&ast).is_err());
    }

    #[test]
    fn for_loop_with_break() {
        // i64 i = 0;
        // for (;true; i = i + 1) {if (i == 5) {break;}}
        let ast = test_node!(Statement::ForLoop {
            declaration: None,
            condition: test_node!(Expression::Literal(Literal::True)),
            assignment: Some(Box::new(test_node!(Statement::Assignment {
                identifier: test_node!(String::from("i")),
                value: test_node!(Expression::Addition(
                    Box::new(test_node!(Expression::Variable(String::from("i")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(1))))
                )),
                indices: vec![]
            }))),
            block: test_node!(Block(vec![test_node!(Statement::Conditional {
                condition: test_node!(Expression::Equal(
                    Box::new(test_node!(Expression::Variable(String::from("i")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(5))))
                )),
                if_block: test_node!(Block(vec![test_node!(Statement::Break)])),
                else_block: None,
            })])),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("i", Rc::new(RefCell::new(Value::I64(0))));

        assert!(interpreter.visit_statement(&ast).is_ok());
        assert_eq!(interpreter.abort_state, None);
        assert_eq!(interpreter.stack.get_variable("i").unwrap().clone(), Rc::new(RefCell::new(Value::I64(5))));
    }

    #[test]
    fn test_function_call() {
        let ast = test_node!(Statement::FunctionCall {
            identifier: test_node!(String::from("add")),
            arguments: vec![
                Box::new(test_node!(Argument {
                    value: test_node!(Expression::Literal(Literal::I64(3))),
                    passed_by: PassedBy::Value,
                })),
                Box::new(test_node!(Argument {
                    value: test_node!(Expression::Literal(Literal::I64(4))),
                    passed_by: PassedBy::Value,
                })),
            ],
        });

        let mut functions: HashMap<String, Rc<Node<FunctionDeclaration>>> = HashMap::new();

        functions.insert(
            String::from("add"),
            Rc::new(test_node!(FunctionDeclaration {
                identifier: test_node!(String::from("add")),
                parameters: vec![
                    test_node!(Parameter {
                        passed_by: PassedBy::Value,
                        parameter_type: test_node!(Type::I64),
                        identifier: test_node!(String::from("a")),
                    }),
                    test_node!(Parameter {
                        passed_by: PassedBy::Value,
                        parameter_type: test_node!(Type::I64),
                        identifier: test_node!(String::from("b")),
                    }),
                ],
                return_type: test_node!(Type::I64),
                block: test_node!(Block(vec![test_node!(Statement::Return(Some(test_node!(Expression::Addition(
                    Box::new(test_node!(Expression::Variable(String::from("a")))),
                    Box::new(test_node!(Expression::Variable(String::from("b")))),
                )))))])),
            })),
        );

        let program = Program {
            statements: vec![],
            std_functions: HashMap::new(),
            functions,
            extern_functions: HashMap::new(),
        };
        let mut interpreter = Interpreter::new(&program);
        assert!(interpreter.visit_statement(&ast).is_ok());
        assert_eq!(interpreter.last_result, Some(Value::I64(7)));
        assert_eq!(interpreter.abort_state, None);
    }

    fn create_test_switch_case() -> Node<Statement> {
        // switch (x) {
        //      (x < 15) {
        //          result = 15;
        //      } (x < 10) {
        //          result = 10;
        //          break;
        //      } (x < 5) {
        //          result = 5;
        //      }
        // }

        fn create_assignment(val: i64) -> Node<Statement> {
            test_node!(Statement::Assignment {
                identifier: test_node!(String::from("result")),
                value: test_node!(Expression::Literal(Literal::I64(val))),
                indices: vec![]
            })
        }

        fn create_condition(val: i64) -> Node<Expression> {
            test_node!(Expression::Less(
                Box::new(test_node!(Expression::Variable(String::from("x")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(val)))),
            ))
        }

        test_node!(Statement::Switch {
            expressions: vec![test_node!(SwitchExpression {
                expression: test_node!(Expression::Variable(String::from("x"))),
                alias: None,
            }),],
            cases: vec![
                test_node!(SwitchCase {
                    condition: create_condition(15),
                    block: test_node!(Block(vec![create_assignment(15)])),
                }),
                test_node!(SwitchCase {
                    condition: create_condition(10),
                    block: test_node!(Block(vec![create_assignment(10), test_node!(Statement::Break),])),
                }),
                test_node!(SwitchCase {
                    condition: create_condition(5),
                    block: test_node!(Block(vec![create_assignment(5)])),
                }),
            ],
        })
    }

    #[test]
    fn switch_enters() {
        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(12))));
        let _ = interpreter
            .stack
            .declare_variable("result", Rc::new(RefCell::new(Value::default_value(&Type::I64).unwrap())));

        let switch_case = &create_test_switch_case();
        let _ = interpreter.visit_statement(switch_case);

        assert_eq!(
            interpreter.stack.get_variable("result").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(15)))
        );
        assert_eq!(interpreter.abort_state, None);
    }

    #[test]
    fn switch_breaks() {
        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(3))));
        let _ = interpreter
            .stack
            .declare_variable("result", Rc::new(RefCell::new(Value::default_value(&Type::I64).unwrap())));

        let switch_case = &create_test_switch_case();
        let _ = interpreter.visit_statement(switch_case);

        assert_eq!(
            interpreter.stack.get_variable("result").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(10)))
        );
        assert_eq!(interpreter.abort_state, None);
    }

    #[test]
    fn switch_no_entry() {
        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("x", Rc::new(RefCell::new(Value::I64(2137))));
        let _ = interpreter
            .stack
            .declare_variable("result", Rc::new(RefCell::new(Value::default_value(&Type::I64).unwrap())));

        let switch_case = &create_test_switch_case();
        let _ = interpreter.visit_statement(switch_case);

        assert_eq!(
            interpreter.stack.get_variable("result").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(0)))
        );
        assert_eq!(interpreter.abort_state, None);
    }

    #[test]
    fn switch_bad_condition_type() {
        // switch () {
        //      (1) -> {}
        // }
        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let ast = test_node!(Statement::Switch {
            expressions: vec![],
            cases: vec![test_node!(SwitchCase {
                condition: test_node!(Expression::Literal(Literal::I64(1))),
                block: test_node!(Block(vec![])),
            }),],
        });

        assert!(interpreter.visit_statement(&ast).is_err())
    }

    #[test]
    fn break_called_outside_for_or_switch() {
        let program = Program {
            functions: HashMap::new(),
            std_functions: HashMap::new(),
            statements: vec![test_node!(Statement::Conditional {
                condition: test_node!(Expression::Literal(Literal::True)),
                if_block: test_node!(Block(vec![test_node!(Statement::Break),])),
                else_block: None,
            })],
            extern_functions: HashMap::new(),
        };

        let mut interpreter = Interpreter::new(&program);
        assert!(interpreter.interpret().is_err())
    }

    #[test]
    fn break_called_outside_for_or_switch_in_function() {
        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let ast = FunctionDeclaration {
            identifier: test_node!(String::from("fun")),
            parameters: vec![],
            return_type: test_node!(Type::Void),
            block: test_node!(Block(vec![test_node!(Statement::Break),])),
        };

        assert!(interpreter.execute_function(&ast).is_err())
    }

    #[test]
    fn return_called_outside_for_or_switch() {
        let program = Program {
            functions: HashMap::new(),
            std_functions: HashMap::new(),
            statements: vec![test_node!(Statement::Conditional {
                condition: test_node!(Expression::Literal(Literal::True)),
                if_block: test_node!(Block(vec![test_node!(Statement::Return(None)),])),
                else_block: None,
            })],
            extern_functions: HashMap::new(),
        };

        let mut interpreter = Interpreter::new(&program);
        assert!(interpreter.interpret().is_err())
    }

    #[test]
    fn bad_arg_type() {
        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let ast = FunctionDeclaration {
            identifier: test_node!(String::from("fun")),
            parameters: vec![test_node!(Parameter {
                passed_by: PassedBy::Value,
                parameter_type: test_node!(Type::I64),
                identifier: test_node!(String::from("x")),
            })],
            return_type: test_node!(Type::Void),
            block: test_node!(Block(vec![])),
        };

        interpreter.last_arguments = vec![Rc::new(RefCell::new(Value::F64(3.2)))];

        assert!(interpreter.execute_function(&ast).is_err())
    }

    #[test]
    fn bad_return_type() {
        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let ast = FunctionDeclaration {
            identifier: test_node!(String::from("fun")),
            parameters: vec![],
            return_type: test_node!(Type::Void),
            block: test_node!(Block(vec![test_node!(Statement::Return(Some(test_node!(Expression::Literal(
                Literal::I64(1)
            ))))),])),
        };

        assert!(interpreter.execute_function(&ast).is_err())
    }

    #[test]
    fn interpret_modulo() {
        let ast = test_node!(Expression::Modulo(
            Box::new(test_node!(Expression::Literal(Literal::I64(7)))),
            Box::new(test_node!(Expression::Literal(Literal::I64(3))))
        ));

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, Some(Value::I64(1)));
    }

    #[test]
    fn while_loop() {
        // i64 i = 0;
        // while (i < 5) { i = i + 1; }
        let ast = test_node!(Statement::WhileLoop {
            condition: test_node!(Expression::Less(
                Box::new(test_node!(Expression::Variable(String::from("i")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(5))))
            )),
            block: test_node!(Block(vec![test_node!(Statement::Assignment {
                identifier: test_node!(String::from("i")),
                value: test_node!(Expression::Addition(
                    Box::new(test_node!(Expression::Variable(String::from("i")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(1))))
                )),
                indices: vec![]
            }),])),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("i", Rc::new(RefCell::new(Value::I64(0))));

        assert!(interpreter.visit_statement(&ast).is_ok());
        assert_eq!(interpreter.stack.get_variable("i").unwrap().clone(), Rc::new(RefCell::new(Value::I64(5))));
        assert_eq!(interpreter.abort_state, None);
    }

    #[test]
    fn for_loop_with_continue() {
        // i64 total = 0;
        // for (i64 i = 0; i < 5; i = i + 1) { if (i == 2) { continue; } total = total + i; }
        let ast = test_node!(Statement::ForLoop {
            declaration: Some(Box::new(test_node!(Statement::Declaration {
                var_type: test_node!(Type::I64),
                identifier: test_node!(String::from("i")),
                value: Some(test_node!(Expression::Literal(Literal::I64(0)))),
            }))),
            condition: test_node!(Expression::Less(
                Box::new(test_node!(Expression::Variable(String::from("i")))),
                Box::new(test_node!(Expression::Literal(Literal::I64(5))))
            )),
            assignment: Some(Box::new(test_node!(Statement::Assignment {
                identifier: test_node!(String::from("i")),
                value: test_node!(Expression::Addition(
                    Box::new(test_node!(Expression::Variable(String::from("i")))),
                    Box::new(test_node!(Expression::Literal(Literal::I64(1))))
                )),
                indices: vec![]
            }))),
            block: test_node!(Block(vec![
                test_node!(Statement::Conditional {
                    condition: test_node!(Expression::Equal(
                        Box::new(test_node!(Expression::Variable(String::from("i")))),
                        Box::new(test_node!(Expression::Literal(Literal::I64(2))))
                    )),
                    if_block: test_node!(Block(vec![test_node!(Statement::Continue)])),
                    else_block: None,
                }),
                test_node!(Statement::Assignment {
                    identifier: test_node!(String::from("total")),
                    value: test_node!(Expression::Addition(
                        Box::new(test_node!(Expression::Variable(String::from("total")))),
                        Box::new(test_node!(Expression::Variable(String::from("i"))))
                    )),
                    indices: vec![]
                }),
            ])),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);
        let _ = interpreter.stack.declare_variable("total", Rc::new(RefCell::new(Value::I64(0))));

        assert!(interpreter.visit_statement(&ast).is_ok());
        // 0+1+3+4 = 8 (pomija i == 2)
        assert_eq!(
            interpreter.stack.get_variable("total").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(8)))
        );
        assert_eq!(interpreter.abort_state, None);
    }

    #[test]
    fn continue_called_outside_for_or_while() {
        let program = Program {
            functions: HashMap::new(),
            std_functions: HashMap::new(),
            statements: vec![test_node!(Statement::Conditional {
                condition: test_node!(Expression::Literal(Literal::True)),
                if_block: test_node!(Block(vec![test_node!(Statement::Continue),])),
                else_block: None,
            })],
            extern_functions: HashMap::new(),
        };

        let mut interpreter = Interpreter::new(&program);
        assert!(interpreter.interpret().is_err())
    }

    #[test]
    fn continue_called_outside_for_or_while_in_function() {
        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let ast = FunctionDeclaration {
            identifier: test_node!(String::from("fun")),
            parameters: vec![],
            return_type: test_node!(Type::Void),
            block: test_node!(Block(vec![test_node!(Statement::Continue),])),
        };

        assert!(interpreter.execute_function(&ast).is_err())
    }

    #[test]
    fn declare_vector_variable() {
        // i64[] x = [1, 2, 3];
        let ast = test_node!(Statement::Declaration {
            var_type: test_node!(Type::Vector(Box::new(Type::I64))),
            identifier: test_node!(String::from("x")),
            value: Some(test_node!(Expression::Vector(vec![
                Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
                Box::new(test_node!(Expression::Literal(Literal::I64(2)))),
                Box::new(test_node!(Expression::Literal(Literal::I64(3)))),
            ]))),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        assert!(interpreter.visit_statement(&ast).is_ok());
    }

    #[test]
    fn declare_vector_variable_wrong_inner_type_fails() {
        // i64[] x = ["a"];
        let ast = test_node!(Statement::Declaration {
            var_type: test_node!(Type::Vector(Box::new(Type::I64))),
            identifier: test_node!(String::from("x")),
            value: Some(test_node!(Expression::Vector(vec![Box::new(test_node!(Expression::Literal(
                Literal::String(String::from("a"))
            ))),]))),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        assert!(interpreter.visit_statement(&ast).is_err());
    }

    #[test]
    fn index_into_vector() {
        // i64[] x = [10, 20, 30];
        // x[1]
        let ast = test_node!(Expression::Index {
            collection: Box::new(test_node!(Expression::Variable(String::from("x")))),
            index: Box::new(test_node!(Expression::Literal(Literal::I64(1)))),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let values = Rc::new(RefCell::new(vec![
            Rc::new(RefCell::new(Value::I64(10))),
            Rc::new(RefCell::new(Value::I64(20))),
            Rc::new(RefCell::new(Value::I64(30))),
        ]));
        let _ = interpreter.stack.declare_variable(
            "x",
            Rc::new(RefCell::new(Value::Vector {
                kind: Box::new(Type::I64),
                values,
            })),
        );

        let _ = interpreter.visit_expression(&ast);
        assert_eq!(interpreter.last_result, Some(Value::I64(20)));
    }

    #[test]
    fn index_out_of_bounds_fails() {
        let ast = test_node!(Expression::Index {
            collection: Box::new(test_node!(Expression::Variable(String::from("x")))),
            index: Box::new(test_node!(Expression::Literal(Literal::I64(5)))),
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let values = Rc::new(RefCell::new(vec![Rc::new(RefCell::new(Value::I64(10)))]));
        let _ = interpreter.stack.declare_variable(
            "x",
            Rc::new(RefCell::new(Value::Vector {
                kind: Box::new(Type::I64),
                values,
            })),
        );

        assert!(interpreter.visit_expression(&ast).is_err());
    }

    #[test]
    fn assign_by_index() {
        // x[1] = 99;
        let ast = test_node!(Statement::Assignment {
            identifier: test_node!(String::from("x")),
            value: test_node!(Expression::Literal(Literal::I64(99))),
            indices: vec![test_node!(Expression::Literal(Literal::I64(1)))]
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        let values = Rc::new(RefCell::new(vec![
            Rc::new(RefCell::new(Value::I64(10))),
            Rc::new(RefCell::new(Value::I64(20))),
        ]));
        let _ = interpreter.stack.declare_variable(
            "x",
            Rc::new(RefCell::new(Value::Vector {
                kind: Box::new(Type::I64),
                values: values.clone(),
            })),
        );

        assert!(interpreter.visit_statement(&ast).is_ok());
        assert_eq!(values.borrow()[1].borrow().clone(), Value::I64(99));
    }

    #[test]
    fn call_undeclared_function_fails() {
        let ast = test_node!(Statement::FunctionCall {
            identifier: test_node!(String::from("does_not_exist")),
            arguments: vec![],
        });

        let program = setup_program();
        let mut interpreter = create_interpreter(&program);

        assert!(interpreter.visit_statement(&ast).is_err());
    }

    #[test]
    fn call_function_wrong_arg_count_fails() {
        let ast = test_node!(Statement::FunctionCall {
            identifier: test_node!(String::from("add")),
            arguments: vec![Box::new(test_node!(Argument {
                value: test_node!(Expression::Literal(Literal::I64(1))),
                passed_by: PassedBy::Value,
            })),],
        });

        let mut functions: HashMap<String, Rc<Node<FunctionDeclaration>>> = HashMap::new();
        functions.insert(
            String::from("add"),
            Rc::new(test_node!(FunctionDeclaration {
                identifier: test_node!(String::from("add")),
                parameters: vec![
                    test_node!(Parameter {
                        passed_by: PassedBy::Value,
                        parameter_type: test_node!(Type::I64),
                        identifier: test_node!(String::from("a")),
                    }),
                    test_node!(Parameter {
                        passed_by: PassedBy::Value,
                        parameter_type: test_node!(Type::I64),
                        identifier: test_node!(String::from("b")),
                    }),
                ],
                return_type: test_node!(Type::I64),
                block: test_node!(Block(vec![])),
            })),
        );

        let program = Program {
            statements: vec![],
            std_functions: HashMap::new(),
            functions,
            extern_functions: HashMap::new(),
        };
        let mut interpreter = Interpreter::new(&program);
        assert!(interpreter.visit_statement(&ast).is_err());
    }

    #[test]
    fn call_function_by_reference() {
        // fn increment(&i64 x): void { x = x + 1; }
        // i64 y = 5; increment(&y);
        let mut functions: HashMap<String, Rc<Node<FunctionDeclaration>>> = HashMap::new();
        functions.insert(
            String::from("increment"),
            Rc::new(test_node!(FunctionDeclaration {
                identifier: test_node!(String::from("increment")),
                parameters: vec![test_node!(Parameter {
                    passed_by: PassedBy::Reference,
                    parameter_type: test_node!(Type::I64),
                    identifier: test_node!(String::from("x")),
                }),],
                return_type: test_node!(Type::Void),
                block: test_node!(Block(vec![test_node!(Statement::Assignment {
                    identifier: test_node!(String::from("x")),
                    value: test_node!(Expression::Addition(
                        Box::new(test_node!(Expression::Variable(String::from("x")))),
                        Box::new(test_node!(Expression::Literal(Literal::I64(1))))
                    )),
                    indices: vec![]
                }),])),
            })),
        );

        let ast = test_node!(Statement::FunctionCall {
            identifier: test_node!(String::from("increment")),
            arguments: vec![Box::new(test_node!(Argument {
                value: test_node!(Expression::Variable(String::from("y"))),
                passed_by: PassedBy::Reference,
            })),],
        });

        let program = Program {
            statements: vec![],
            std_functions: HashMap::new(),
            functions,
            extern_functions: HashMap::new(),
        };
        let mut interpreter = Interpreter::new(&program);
        let _ = interpreter.stack.declare_variable("y", Rc::new(RefCell::new(Value::I64(5))));

        assert!(interpreter.visit_statement(&ast).is_ok());
        assert_eq!(interpreter.stack.get_variable("y").unwrap().clone(), Rc::new(RefCell::new(Value::I64(6))));
    }
}
