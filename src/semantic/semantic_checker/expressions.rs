use crate::{
    common::{
        errors::{ErrorSeverity, IError, SemanticCheckerError},
        span::Span,
        types::Type,
        visitor::Visitor,
    },
    frontend::ast::{Expression, Node},
    semantic::{
        semantic_checker::{checker::HoverInfo, functions::FunctionCallType, SemanticChecker},
        type_alu::TypeALU,
    },
};

impl<'a> SemanticChecker<'a> {
    pub(in crate::semantic::semantic_checker) fn check_index_assignment(
        &mut self,
        identifier: &'a Node<String>,
        indices: &'a Vec<Node<Expression>>,
        value: &'a Node<Expression>,
        span: Span,
    ) {
        let mut current_type = match self.stack.get_variable(identifier.value.as_str(), identifier.span) {
            Ok(t) => t.clone(),

            Err(err) => {
                self.errors
                    .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), identifier.span)));
                return;
            }
        };

        self.hovers.push(HoverInfo {
            contents: format!("```raptor\n{:?} {}\n```", current_type, identifier.value),
            span: identifier.span,
        });

        for index_expr in indices {
            let _ = self.visit_expression(index_expr);

            if let Ok(idx_type) = self.read_last_result(index_expr.span) {
                if idx_type != Type::I64 {
                    self.errors.push(Box::new(SemanticCheckerError::type_mismatch(
                        ErrorSeverity::HIGH,
                        String::from("array index must be `i64`"),
                        &Type::I64,
                        &idx_type,
                        index_expr.span,
                    )));
                    return;
                }
            }

            match current_type {
                Type::Vector(inner) => current_type = *inner,
                Type::Str => current_type = Type::Char,

                other => {
                    self.errors.push(Box::new(SemanticCheckerError::at(
                        ErrorSeverity::HIGH,
                        format!("Cannot index into value of type `{:?}`", other),
                        span,
                    )));
                    return;
                }
            }
        }

        let _ = self.visit_expression(value);

        if let Ok(actual_type) = self.read_last_result(value.span) {
            if actual_type != current_type {
                self.errors.push(Box::new(SemanticCheckerError::type_mismatch(
                    ErrorSeverity::HIGH,
                    format!("Cannot assign `{:?}` to array element.", actual_type),
                    &current_type,
                    &actual_type,
                    span,
                )));
            }
        }
    }

    pub(in crate::semantic::semantic_checker) fn check_expression(&mut self, expression: &'a Node<Expression>) -> Result<(), Box<dyn IError>> {
        match &expression.value {
            Expression::FunctionCall { .. } => {
                self.check_function_call(FunctionCallType::Expression(expression));
                return Ok(());
            }

            _ => {}
        }

        match &expression.value {
            Expression::Alternative(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::alternative)?,

            Expression::Concatenation(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::concatenation)?,

            Expression::Greater(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::greater)?,

            Expression::GreaterEqual(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::greater_or_equal)?,

            Expression::Less(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::less)?,

            Expression::LessEqual(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::less_or_equal)?,

            Expression::Equal(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::equal)?,

            Expression::NotEqual(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::not_equal)?,

            Expression::Addition(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::add)?,

            Expression::Subtraction(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::subtract)?,

            Expression::Multiplication(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::multiplication)?,

            Expression::Division(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::division)?,

            Expression::Modulo(lhs, rhs) => self.evaluate_binary_op(lhs, rhs, TypeALU::modulo)?,

            Expression::BooleanNegation(value) => self.evaluate_unary_op(value, TypeALU::boolean_negate)?,

            Expression::ArithmeticNegation(value) => self.evaluate_unary_op(value, TypeALU::arithmetic_negate)?,

            Expression::Casting { value, to_type } => {
                self.visit_expression(value)?;

                let from_type = self.read_last_result(value.span);

                match from_type {
                    Ok(t) => match TypeALU::cast_to_type(t, &to_type.value, Span::new(value.span.start(), to_type.span.end())) {
                        Ok(result_type) => {
                            self.last_result = Some(result_type);
                        }

                        Err(err) => {
                            self.errors
                                .push(Box::new(SemanticCheckerError::at(ErrorSeverity::HIGH, err.message(), expression.span)));

                            self.last_result = None;
                        }
                    },

                    Err(_) => {
                        self.last_result = None;
                    }
                }
            }

            Expression::Literal(literal) => self.visit_literal(literal)?,

            Expression::Variable(variable) => self.visit_variable(variable, expression.span)?,

            Expression::FunctionCall { .. } => {
                unreachable!("Function call is handled separately.")
            }

            Expression::Vector(vector) => self.visit_vector_literal(vector)?,

            Expression::Index { collection, index } => {
                self.visit_expression(collection)?;
                let collection_type = self.read_last_result(collection.span);

                self.visit_expression(index)?;
                let index_type = self.read_last_result(index.span);

                match (collection_type, index_type) {
                    (Ok(Type::Vector(inner)), Ok(Type::I64)) => {
                        self.last_result = Some(*inner);
                    }
                    (Ok(Type::Str), Ok(Type::I64)) => {
                        self.last_result = Some(Type::Char);
                    }

                    (Ok(other), Ok(Type::I64)) => {
                        self.errors.push(Box::new(SemanticCheckerError::at(
                            ErrorSeverity::HIGH,
                            format!("cannot index into value of type `{:?}`", other),
                            expression.span,
                        )));

                        self.last_result = None;
                    }

                    (Ok(_), Ok(other)) => {
                        self.errors.push(Box::new(SemanticCheckerError::type_mismatch(
                            ErrorSeverity::HIGH,
                            String::from("array index must be `i64`"),
                            &Type::I64,
                            &other,
                            index.span,
                        )));

                        self.last_result = None;
                    }

                    _ => {
                        self.last_result = None;
                    }
                }
            }

            Expression::StructLiteral { .. } => {
                let err = SemanticCheckerError::at(
                    ErrorSeverity::HIGH,
                    String::from("Visiting struct literal in semantic checker is not implemented yet."),
                    expression.span,
                );

                self.errors.push(Box::new(err.clone()));

                return Err(Box::new(err));
            }
        }

        Ok(())
    }
}
