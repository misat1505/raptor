use crate::{
    common::{
        errors::{ErrorSeverity, IError, MacroExpanderError},
        span::Span,
        types::Type,
    },
    frontend::ast::{
        Argument, Block, DeclaredType, Expression, FunctionDeclaration, MatchArm, Node, Parameter, PassedBy, Statement, StructDeclaration,
        StructLiteral, StructLiteralField, VariableDeclarationKind,
    },
    macro_expander::macro_expander::{macro_node, to_snake_case, MacroExpander},
};

impl<'a> MacroExpander<'a> {
    pub(in crate::macro_expander) fn derive_clone(&mut self, declared_type: &DeclaredType, derive_span: Span) {
        let (type_name, clone_function) = match declared_type {
            DeclaredType::Struct(struct_declaration) => {
                let type_name = struct_declaration.identifier.value.clone();
                let function = self.generate_struct_clone(struct_declaration, derive_span);
                (type_name, function)
            }
            DeclaredType::Enum(enum_declaration) => {
                let type_name = enum_declaration.identifier.value.clone();
                let function = self.generate_enum_clone(enum_declaration, derive_span);
                (type_name, function)
            }
        };

        if let Some(function) = clone_function {
            self.insert_function(function, derive_span);
        } else {
            let _ = type_name;
        }
    }

    fn generate_struct_clone(&mut self, declaration: &StructDeclaration, derive_span: Span) -> Option<FunctionDeclaration> {
        let type_name = declaration.identifier.value.clone();
        let variable_name = to_snake_case(&type_name);

        for member in &declaration.members {
            if !self.can_clone_type(&member.value.member_type.value, &type_name, derive_span) {
                return None;
            }
        }

        let mut statements = Vec::new();
        let mut fields = Vec::new();
        let mut counter = 0;

        for member in &declaration.members {
            let field_name = member.value.identifier.value.clone();
            let field_expression = Expression::FieldAccess {
                instance: Box::new(macro_node!(Expression::Variable(variable_name.clone()))),
                field: macro_node!(field_name.clone()),
            };

            let (mut clone_statements, cloned_expression) =
                self.clone_expression(field_expression, &member.value.member_type.value, &type_name, derive_span, &mut counter);

            statements.append(&mut clone_statements);
            fields.push(macro_node!(StructLiteralField {
                identifier: macro_node!(field_name),
                value: macro_node!(cloned_expression),
            }));
        }

        let struct_literal = Expression::StructLiteral(macro_node!(StructLiteral {
            identifier: macro_node!(type_name.clone()),
            fields,
        }));

        statements.push(macro_node!(Statement::Return(Some(macro_node!(struct_literal)))));

        Some(FunctionDeclaration {
            identifier: macro_node!(format!("{}_clone", to_snake_case(&type_name))),
            parameters: vec![macro_node!(Parameter {
                passed_by: PassedBy::Reference,
                parameter_type: macro_node!(Type::Unresolved(type_name.clone())),
                identifier: macro_node!(variable_name),
            })],
            return_type: macro_node!(Type::Unresolved(type_name)),
            block: macro_node!(Block(statements)),
        })
    }

    fn generate_enum_clone(&mut self, declaration: &crate::frontend::ast::EnumDeclaration, derive_span: Span) -> Option<FunctionDeclaration> {
        let enum_name = declaration.identifier.value.clone();
        let variable_name = to_snake_case(&enum_name);

        for member in &declaration.members {
            if let Some(member_type) = &member.value.member_type {
                if !self.can_clone_type(&member_type.value, &enum_name, derive_span) {
                    return None;
                }
            }
        }

        let mut match_arms = Vec::new();
        let mut counter = 0;

        for member in &declaration.members {
            let variant_name = member.value.identifier.value.clone();

            match &member.value.member_type {
                None => {
                    let expression = Expression::EnumLiteral {
                        enum_name: macro_node!(enum_name.clone()),
                        variant_name: macro_node!(variant_name.clone()),
                        variant_value: None,
                    };

                    match_arms.push(macro_node!(MatchArm {
                        enum_name: macro_node!(enum_name.clone()),
                        variant_name: macro_node!(variant_name),
                        variant_value: None,
                        block: macro_node!(Block(vec![macro_node!(Statement::Return(Some(macro_node!(expression))))])),
                    }));
                }
                Some(member_type) => {
                    let payload_name = format!("value_{}", counter);
                    counter += 1;

                    let payload_expression = Expression::Variable(payload_name.clone());

                    let (clone_statements, cloned_expression) =
                        self.clone_expression(payload_expression, &member_type.value, &enum_name, derive_span, &mut counter);

                    let expression = Expression::EnumLiteral {
                        enum_name: macro_node!(enum_name.clone()),
                        variant_name: macro_node!(variant_name.clone()),
                        variant_value: Some(Box::new(macro_node!(cloned_expression))),
                    };

                    let mut block_statements = clone_statements;
                    block_statements.push(macro_node!(Statement::Return(Some(macro_node!(expression)))));

                    match_arms.push(macro_node!(MatchArm {
                        enum_name: macro_node!(enum_name.clone()),
                        variant_name: macro_node!(variant_name),
                        variant_value: Some(macro_node!(payload_name)),
                        block: macro_node!(Block(block_statements)),
                    }));
                }
            }
        }

        Some(FunctionDeclaration {
            identifier: macro_node!(format!("{}_clone", to_snake_case(&enum_name))),
            parameters: vec![macro_node!(Parameter {
                passed_by: PassedBy::Reference,
                parameter_type: macro_node!(Type::Unresolved(enum_name.clone())),
                identifier: macro_node!(variable_name.clone()),
            })],
            return_type: macro_node!(Type::Unresolved(enum_name.clone())),
            block: macro_node!(Block(vec![macro_node!(Statement::Match {
                expression: macro_node!(Expression::Variable(variable_name)),
                match_arms,
                rest_arm: None,
            })])),
        })
    }

    fn can_clone_type(&mut self, ty: &Type, parent_type: &str, derive_span: Span) -> bool {
        match ty {
            Type::Bool
            | Type::Str
            | Type::Char
            | Type::I8
            | Type::I16
            | Type::I32
            | Type::I64
            | Type::U8
            | Type::U16
            | Type::U32
            | Type::U64
            | Type::F64 => true,

            Type::Vector(inner_type) => self.can_clone_type(inner_type, parent_type, derive_span),

            Type::Unresolved(type_name) => {
                let clone_function = format!("{}_clone", to_snake_case(type_name));

                if self.program.functions.contains_key(&clone_function) {
                    return true;
                }

                let has_clone = self
                    .program
                    .declared_types
                    .get(type_name)
                    .map(|declared_type| match &declared_type.value {
                        DeclaredType::Struct(declaration) => declaration.derives.iter().any(|derive| derive.value == "Clone"),
                        DeclaredType::Enum(declaration) => declaration.derives.iter().any(|derive| derive.value == "Clone"),
                    })
                    .unwrap_or(false);

                if has_clone {
                    true
                } else {
                    self.errors.push(Box::new(MacroExpanderError::at(
                        ErrorSeverity::HIGH,
                        format!(
                            "Type '{}' used by '{}' cannot be cloned. Hint: add 'Clone' to the derives of '{}', or implement the function 'fn {}_clone(&{} value): {}'.",
                            type_name,
                            parent_type,
                            type_name,
                            to_snake_case(type_name),
                            type_name,
                            type_name,
                        ),
                        derive_span,
                    )));
                    false
                }
            }

            Type::Any => {
                self.errors.push(Box::new(MacroExpanderError::at(
                    ErrorSeverity::HIGH,
                    format!("Type '{}' cannot be cloned because it contains a value of type 'Any'.", parent_type),
                    derive_span,
                )));
                false
            }

            Type::Void => {
                self.errors.push(Box::new(MacroExpanderError::at(
                    ErrorSeverity::HIGH,
                    format!("Type '{}' cannot be cloned because it contains a value of type 'Void'.", parent_type),
                    derive_span,
                )));
                false
            }

            Type::Struct { identifier, .. } => {
                let clone_function = format!("{}_clone", to_snake_case(identifier));

                if self.program.functions.contains_key(&clone_function) {
                    return true;
                }

                let has_clone = self
                    .program
                    .declared_types
                    .get(identifier)
                    .map(|declared_type| match &declared_type.value {
                        DeclaredType::Struct(declaration) => declaration.derives.iter().any(|derive| derive.value == "Clone"),
                        DeclaredType::Enum(declaration) => declaration.derives.iter().any(|derive| derive.value == "Clone"),
                    })
                    .unwrap_or(false);

                if has_clone {
                    true
                } else {
                    self.errors.push(Box::new(MacroExpanderError::at(
                        ErrorSeverity::HIGH,
                        format!(
                            "Type '{}' used by '{}' cannot be cloned. Hint: add 'Clone' to the derives of '{}', or implement the function 'fn {}_clone(&{} value): {}.'.",
                            identifier,
                            parent_type,
                            identifier,
                            to_snake_case(identifier),
                            identifier,
                            identifier,
                        ),
                        derive_span,
                    )));
                    false
                }
            }

            Type::Enum { identifier, .. } => {
                let clone_function = format!("{}_clone", to_snake_case(identifier));

                if self.program.functions.contains_key(&clone_function) {
                    return true;
                }

                let has_clone = self
                    .program
                    .declared_types
                    .get(identifier)
                    .map(|declared_type| match &declared_type.value {
                        DeclaredType::Struct(declaration) => declaration.derives.iter().any(|derive| derive.value == "Clone"),
                        DeclaredType::Enum(declaration) => declaration.derives.iter().any(|derive| derive.value == "Clone"),
                    })
                    .unwrap_or(false);

                if has_clone {
                    true
                } else {
                    self.errors.push(Box::new(MacroExpanderError::at(
                        ErrorSeverity::HIGH,
                        format!(
                            "Type '{}' used by '{}' cannot be cloned. Hint: add 'Clone' to the derives of '{}', or implement the function 'fn {}_clone(&{} value): {}.'.",
                            identifier,
                            parent_type,
                            identifier,
                            to_snake_case(identifier),
                            identifier,
                            identifier,
                        ),
                        derive_span,
                    )));
                    false
                }
            }
        }
    }

    fn clone_expression(
        &mut self,
        expression: Expression,
        ty: &Type,
        parent_type: &str,
        derive_span: Span,
        counter: &mut usize,
    ) -> (Vec<Node<Statement>>, Expression) {
        match ty {
            Type::Bool
            | Type::Str
            | Type::Char
            | Type::I8
            | Type::I16
            | Type::I32
            | Type::I64
            | Type::U8
            | Type::U16
            | Type::U32
            | Type::U64
            | Type::F64 => (vec![], expression),

            Type::Unresolved(type_name) => {
                let function_name = format!("{}_clone", to_snake_case(type_name));

                if !self.program.functions.contains_key(&function_name) {
                    let has_clone = self
                        .program
                        .declared_types
                        .get(type_name)
                        .map(|declared_type| match &declared_type.value {
                            DeclaredType::Struct(declaration) => declaration.derives.iter().any(|derive| derive.value == "Clone"),
                            DeclaredType::Enum(declaration) => declaration.derives.iter().any(|derive| derive.value == "Clone"),
                        })
                        .unwrap_or(false);

                    if !has_clone {
                        let _ = self.can_clone_type(ty, parent_type, derive_span);
                        return (vec![], expression);
                    }
                }

                let call = Expression::FunctionCall {
                    identifier: macro_node!(function_name),
                    arguments: vec![Box::new(macro_node!(Argument {
                        value: macro_node!(expression),
                        passed_by: PassedBy::Reference,
                    }))],
                };

                (vec![], call)
            }

            Type::Struct { identifier, .. } | Type::Enum { identifier, .. } => {
                let function_name = format!("{}_clone", to_snake_case(identifier));

                let call = Expression::FunctionCall {
                    identifier: macro_node!(function_name),
                    arguments: vec![Box::new(macro_node!(Argument {
                        value: macro_node!(expression),
                        passed_by: PassedBy::Reference,
                    }))],
                };

                (vec![], call)
            }

            Type::Vector(inner_type) => {
                let vector_name = format!("__clone_vector_{}", *counter);
                *counter += 1;

                let index_name = format!("__clone_index_{}", *counter);
                *counter += 1;

                let mut loop_statements = Vec::new();

                let indexed_expression = Expression::Index {
                    collection: Box::new(macro_node!(expression.clone())),
                    index: Box::new(macro_node!(Expression::Variable(index_name.clone()))),
                };

                let (mut nested_statements, cloned_element) =
                    self.clone_expression(indexed_expression, inner_type, parent_type, derive_span, counter);

                loop_statements.append(&mut nested_statements);

                loop_statements.push(macro_node!(Statement::FunctionCall {
                    identifier: macro_node!("vector_push".to_owned()),
                    arguments: vec![
                        Box::new(macro_node!(Argument {
                            value: macro_node!(Expression::Variable(vector_name.clone())),
                            passed_by: PassedBy::Reference,
                        })),
                        Box::new(macro_node!(Argument {
                            value: macro_node!(cloned_element),
                            passed_by: PassedBy::Value,
                        })),
                    ],
                }));

                loop_statements.push(macro_node!(Statement::Assignment {
                    identifier: macro_node!(index_name.clone()),
                    accessors: vec![],
                    value: macro_node!(Expression::Addition(
                        Box::new(macro_node!(Expression::Variable(index_name.clone()))),
                        Box::new(macro_node!(Expression::Literal(crate::frontend::ast::Literal::I64(1)))),
                    )),
                }));

                let vector_size_call = Expression::FunctionCall {
                    identifier: macro_node!("vector_size".to_owned()),
                    arguments: vec![Box::new(macro_node!(Argument {
                        value: macro_node!(expression.clone()),
                        passed_by: PassedBy::Reference,
                    }))],
                };

                let condition = Expression::Less(
                    Box::new(macro_node!(Expression::Variable(index_name.clone()))),
                    Box::new(macro_node!(vector_size_call)),
                );

                let vector_type = Type::Vector(Box::new((**inner_type).clone()));

                let declaration = macro_node!(Statement::Declaration {
                    identifier: macro_node!(vector_name.clone()),
                    kind: VariableDeclarationKind::LET {
                        var_type: Some(macro_node!(vector_type)),
                        value: macro_node!(Expression::Vector(vec![])),
                    },
                });

                let index_declaration = macro_node!(Statement::Declaration {
                    identifier: macro_node!(index_name.clone()),
                    kind: VariableDeclarationKind::LET {
                        var_type: Some(macro_node!(Type::I64)),
                        value: macro_node!(Expression::Literal(crate::frontend::ast::Literal::I64(0))),
                    },
                });

                let while_statement = macro_node!(Statement::WhileLoop {
                    condition: macro_node!(condition),
                    block: macro_node!(Block(loop_statements)),
                });

                (vec![declaration, index_declaration, while_statement], Expression::Variable(vector_name))
            }

            Type::Any | Type::Void => {
                let _ = self.can_clone_type(ty, parent_type, derive_span);
                (vec![], expression)
            }
        }
    }
}
