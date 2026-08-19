use crate::{
    common::{
        errors::{ErrorSeverity, IError, SemanticCheckerError},
        span::Span,
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::{Argument, Block, Expression, Literal, Node, Parameter, Program, Statement, SwitchCase, SwitchExpression},
    semantic::semantic_checker::{checker::HoverInfo, functions::FunctionCallType, SemanticChecker},
};

impl<'a> Visitor<'a> for SemanticChecker<'a> {
    #![allow(unused_must_use)]

    fn visit_program(&mut self, program: &'a Program) -> Result<(), Box<dyn IError>> {
        for statement in &program.statements {
            self.visit_statement(statement);
        }

        for (_name, function) in &program.functions {
            self.stack.push_stack_frame().map_err(|err| Box::new(err));

            self.current_function_return_type = Some(function.value.return_type.value.clone());

            for param in &function.value.parameters {
                let param_name = &param.value.identifier.value;
                let param_type = &param.value.parameter_type.value;

                if let Err(err) = self.stack.declare_variable(param_name, param_type.clone(), param.span) {
                    self.errors
                        .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), param.span)));
                }
            }

            self.visit_block(&function.value.block);

            self.current_function_return_type = None;
            self.stack.pop_stack_frame();
        }

        Ok(())
    }

    fn visit_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        self.check_expression(expression)
    }

    fn visit_statement(&mut self, statement: &'a Node<Statement>) -> Result<(), Box<dyn IError>> {
        match statement.value {
            Statement::FunctionCall { .. } => self.check_function_call(FunctionCallType::Statement(statement)),
            Statement::Declaration { .. } => self.check_declaration(statement)?,
            Statement::Assignment { .. } => self.check_assignment(statement)?,
            Statement::Conditional { .. } => self.check_conditional(statement)?,
            Statement::WhileLoop { .. } => self.check_while_loop(statement)?,
            Statement::ForLoop { .. } => self.check_for_loop(statement)?,
            Statement::Switch { .. } => self.check_switch(statement)?,
            Statement::Return { .. } => self.check_return(statement)?,
            Statement::Break { .. } => self.check_break(statement)?,
            Statement::Continue { .. } => self.check_continue(statement)?,
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
            self.visit_statement(statement);
        }

        self.stack.pop_scope();

        Ok(())
    }

    fn visit_parameter(&mut self, parameter: &'a Node<Parameter>) -> Result<(), Box<dyn IError>> {
        self.visit_type(&parameter.value.parameter_type)?;
        Ok(())
    }

    fn visit_switch_case(&mut self, switch_case: &'a Node<SwitchCase>) -> Result<(), Box<dyn IError>> {
        self.check_switch_case(switch_case)
    }

    fn visit_switch_expression(&mut self, switch_expression: &'a Node<SwitchExpression>) -> Result<(), Box<dyn IError>> {
        self.check_switch_expression(switch_expression)
    }

    fn visit_type(&mut self, _node_type: &'a Node<Type>) -> Result<(), Box<dyn IError>> {
        Ok(())
    }

    fn visit_literal(&mut self, literal: &'a Literal) -> Result<(), Box<dyn IError>> {
        let t = match literal {
            Literal::F64(_) => Type::F64,
            Literal::I64(_) => Type::I64,
            Literal::String(_) => Type::Str,
            Literal::False | Literal::True => Type::Bool,
        };

        self.last_result = Some(t);
        Ok(())
    }

    fn visit_variable(&mut self, variable: &'a String, span: Span) -> Result<(), Box<dyn IError>> {
        let value = self.stack.get_variable(variable.as_str(), span).map_err(|err| {
            let error = SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), span);

            self.errors.push(Box::new(error.clone()));

            Box::new(error) as Box<dyn IError>
        })?;

        self.hovers.push(HoverInfo {
            contents: format!("```raptor\n{:?} {}\n```", value, variable),
            span,
        });

        self.last_result = Some(value.clone());

        Ok(())
    }

    fn visit_vector_literal(&mut self, vector: &'a Vec<Box<Node<Expression>>>) -> Result<(), Box<dyn IError>> {
        let mut element_type: Option<Type> = None;

        for expression in vector {
            self.visit_expression(expression)?;

            if let Ok(t) = self.read_last_result(expression.span) {
                match &element_type {
                    None => {
                        element_type = Some(t);
                    }

                    Some(expected) if *expected != t => {
                        self.errors.push(Box::new(SemanticCheckerError::type_mismatch(
                            ErrorSeverity::HIGH,
                            String::from("vector elements have mismatched types"),
                            expected,
                            &t,
                            expression.span,
                        )));
                    }

                    _ => {}
                }
            }
        }

        let vector_type = Type::Vector(Box::new(element_type.unwrap_or(Type::Void)));

        if let (Some(first), Some(last)) = (vector.first(), vector.last()) {
            let span = Span::new(first.span.start(), last.span.end());

            self.hovers.push(HoverInfo {
                contents: format!("```raptor\n{:?}\n```", vector_type),
                span,
            });
        }

        self.last_result = Some(vector_type);

        Ok(())
    }
}
