use crate::{
    common::{
        errors::{ErrorSeverity, IError, MacroExpanderError},
        span::Span,
        types::Type,
    },
    frontend::ast::{
        Argument, Block, DeclaredType, Expression, FunctionDeclaration, Literal, MatchArm, Node, Parameter, PassedBy, Statement, StructDeclaration,
        VariableDeclarationKind,
    },
    macro_expander::macro_expander::{macro_node, to_snake_case, MacroExpander},
};

impl<'a> MacroExpander<'a> {
    pub(in crate::macro_expander) fn derive_debug(&mut self, declared_type: &DeclaredType, derive_span: Span) {
        if !self.check_debug_dependencies(declared_type, derive_span) {
            return;
        }

        let (type_name, block) = match declared_type {
            DeclaredType::Enum(enum_declaration) => (enum_declaration.identifier.value.clone(), self.enum_debug_block(enum_declaration)),
            DeclaredType::Struct(struct_declaration) => (struct_declaration.identifier.value.clone(), self.struct_debug_block(struct_declaration)),
        };

        let debug_fn_name = format!("{}_debug", to_snake_case(type_name.as_str()));
        let param_name = to_snake_case(type_name.as_str());

        let debug_fn = FunctionDeclaration {
            identifier: macro_node!(debug_fn_name),
            parameters: vec![macro_node!(Parameter {
                passed_by: PassedBy::Reference,
                parameter_type: macro_node!(Type::Unresolved(type_name.clone())),
                identifier: macro_node!(param_name),
            })],
            return_type: macro_node!(Type::Str),
            block: macro_node!(block),
        };

        self.insert_function(debug_fn, derive_span);
    }

    fn check_debug_dependencies(&mut self, declared_type: &DeclaredType, derive_span: Span) -> bool {
        let (parent_type, members) = match declared_type {
            DeclaredType::Struct(struct_declaration) => (
                struct_declaration.identifier.value.clone(),
                struct_declaration
                    .members
                    .iter()
                    .map(|member| member.value.member_type.value.clone())
                    .collect::<Vec<_>>(),
            ),
            DeclaredType::Enum(enum_declaration) => (
                enum_declaration.identifier.value.clone(),
                enum_declaration
                    .members
                    .iter()
                    .filter_map(|member| member.value.member_type.as_ref())
                    .map(|member_type| member_type.value.clone())
                    .collect::<Vec<_>>(),
            ),
        };

        let mut valid = true;

        for member_type in members {
            if !self.check_debug_type(&parent_type, &member_type, derive_span) {
                valid = false;
            }
        }

        valid
    }

    fn check_debug_type(&mut self, parent_type: &str, member_type: &Type, derive_span: Span) -> bool {
        match member_type {
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

            Type::Vector(inner_type) => self.check_debug_type(parent_type, inner_type, derive_span),

            Type::Unresolved(dependency_type)
            | Type::Struct {
                identifier: dependency_type, ..
            }
            | Type::Enum {
                identifier: dependency_type, ..
            } => self.check_debug_dependency(parent_type, dependency_type, derive_span),

            Type::Any => {
                self.errors.push(Box::new(MacroExpanderError::at(
                    ErrorSeverity::HIGH,
                    format!("Type '{}' contains a field with type 'any', which cannot be debugged.", parent_type),
                    derive_span,
                )));
                false
            }

            Type::Void => {
                self.errors.push(Box::new(MacroExpanderError::at(
                    ErrorSeverity::HIGH,
                    format!("Type '{}' contains a field with type 'void', which cannot be debugged.", parent_type),
                    derive_span,
                )));
                false
            }
        }
    }

    fn check_debug_dependency(&mut self, parent_type: &str, dependency_type: &str, derive_span: Span) -> bool {
        let debug_fn_name = format!("{}_debug", to_snake_case(dependency_type));

        if self.program.functions.contains_key(&debug_fn_name) {
            return true;
        }

        let has_debug = self
            .program
            .declared_types
            .iter()
            .find_map(|(_, declared_type)| match &declared_type.value {
                DeclaredType::Struct(struct_declaration) if struct_declaration.identifier.value == dependency_type => {
                    Some(struct_declaration.derives.iter().any(|derive| derive.value == "Debug"))
                }

                DeclaredType::Enum(enum_declaration) if enum_declaration.identifier.value == dependency_type => {
                    Some(enum_declaration.derives.iter().any(|derive| derive.value == "Debug"))
                }

                _ => None,
            })
            .unwrap_or(false);

        if has_debug {
            return true;
        }

        self.errors.push(Box::new(MacroExpanderError::at(
            ErrorSeverity::HIGH,
            format!(
                "Type '{}' used by '{}' cannot be debugged. Hint: add 'Debug' to the derives of '{}', or implement the function 'fn {}_debug(&{} value): str'.",
                dependency_type,
                parent_type,
                dependency_type,
                to_snake_case(dependency_type),
                dependency_type,
            ),
            derive_span,
        )));

        false
    }

    fn struct_debug_block(&self, struct_declaration: &StructDeclaration) -> Block {
        let struct_name = struct_declaration.identifier.value.clone();
        let variable_name = to_snake_case(struct_name.as_str());

        if struct_declaration
            .members
            .iter()
            .any(|member| matches!(member.value.member_type.value, Type::Vector(_)))
        {
            return self.struct_debug_block_with_vectors(struct_declaration, variable_name.as_str());
        }

        let mut expressions = Vec::new();

        expressions.push(Expression::Literal(Literal::String(format!("{} {{ ", struct_name))));

        for (index, member) in struct_declaration.members.iter().enumerate() {
            let field_name = member.value.identifier.value.clone();
            let field_type = member.value.member_type.value.clone();

            if index > 0 {
                expressions.push(Expression::Literal(Literal::String(", ".to_owned())));
            }

            expressions.push(Expression::Literal(Literal::String(format!("{}: ", field_name))));

            let field_expression = Expression::FieldAccess {
                instance: Box::new(macro_node!(Expression::Variable(variable_name.clone()))),
                field: macro_node!(field_name),
            };

            expressions.push(self.debug_expression_for_expression(&field_type, field_expression));
        }

        expressions.push(Expression::Literal(Literal::String(" }".to_owned())));

        let return_expression = self.combine_string_expressions(expressions);

        Block(vec![macro_node!(Statement::Return(Some(macro_node!(return_expression))))])
    }

    fn struct_debug_block_with_vectors(&self, struct_declaration: &StructDeclaration, variable_name: &str) -> Block {
        let struct_name = struct_declaration.identifier.value.clone();
        let result_variable = "struct_str".to_owned();
        let mut statements = Vec::new();

        statements.push(macro_node!(Statement::Declaration {
            identifier: macro_node!(result_variable.clone()),
            kind: VariableDeclarationKind::LET {
                var_type: None,
                value: macro_node!(Expression::Literal(Literal::String(format!("{} {{ ", struct_name)))),
            },
        }));

        for (index, member) in struct_declaration.members.iter().enumerate() {
            let field_name = member.value.identifier.value.clone();
            let field_type = member.value.member_type.value.clone();

            if index > 0 {
                statements.push(macro_node!(Statement::Assignment {
                    identifier: macro_node!(result_variable.clone()),
                    accessors: vec![],
                    value: macro_node!(Expression::Addition(
                        Box::new(macro_node!(Expression::Variable(result_variable.clone()))),
                        Box::new(macro_node!(Expression::Literal(Literal::String(", ".to_owned())))),
                    )),
                }));
            }

            statements.push(macro_node!(Statement::Assignment {
                identifier: macro_node!(result_variable.clone()),
                accessors: vec![],
                value: macro_node!(Expression::Addition(
                    Box::new(macro_node!(Expression::Variable(result_variable.clone()))),
                    Box::new(macro_node!(Expression::Literal(Literal::String(format!("{}: ", field_name))))),
                )),
            }));

            let field_expression = Expression::FieldAccess {
                instance: Box::new(macro_node!(Expression::Variable(variable_name.to_owned()))),
                field: macro_node!(field_name.clone()),
            };

            if let Type::Vector(inner_type) = field_type {
                let vector_variable = format!("{}_{}_str", variable_name, field_name);

                let vector_statements = self.vector_debug_statements(&vector_variable, field_expression, &inner_type);

                statements.extend(vector_statements);

                statements.push(macro_node!(Statement::Assignment {
                    identifier: macro_node!(result_variable.clone()),
                    accessors: vec![],
                    value: macro_node!(Expression::Addition(
                        Box::new(macro_node!(Expression::Variable(result_variable.clone()))),
                        Box::new(macro_node!(Expression::Variable(vector_variable))),
                    )),
                }));
            } else {
                let value_expression = self.debug_expression_for_expression(&field_type, field_expression);

                statements.push(macro_node!(Statement::Assignment {
                    identifier: macro_node!(result_variable.clone()),
                    accessors: vec![],
                    value: macro_node!(Expression::Addition(
                        Box::new(macro_node!(Expression::Variable(result_variable.clone()))),
                        Box::new(macro_node!(value_expression)),
                    )),
                }));
            }
        }

        statements.push(macro_node!(Statement::Assignment {
            identifier: macro_node!(result_variable.clone()),
            accessors: vec![],
            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(result_variable.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::String(" }".to_owned())))),
            )),
        }));

        statements.push(macro_node!(Statement::Return(Some(macro_node!(Expression::Variable(result_variable))))));

        Block(statements)
    }

    fn vector_debug_statements(&self, result_variable: &str, vector_expression: Expression, inner_type: &Type) -> Vec<Node<Statement>> {
        let index_variable = format!("{}_i", result_variable);
        let mut statements = Vec::new();

        statements.push(macro_node!(Statement::Declaration {
            identifier: macro_node!(result_variable.to_owned()),
            kind: VariableDeclarationKind::LET {
                var_type: None,
                value: macro_node!(Expression::Literal(Literal::String("[".to_owned()))),
            },
        }));

        statements.push(macro_node!(Statement::Declaration {
            identifier: macro_node!(index_variable.clone()),
            kind: VariableDeclarationKind::LET {
                var_type: None,
                value: macro_node!(Expression::Literal(Literal::I64(0))),
            },
        }));

        let condition = Expression::Less(
            Box::new(macro_node!(Expression::Variable(index_variable.clone()))),
            Box::new(macro_node!(Expression::FunctionCall {
                identifier: macro_node!("vector_size".to_owned()),
                arguments: vec![Box::new(macro_node!(Argument {
                    value: macro_node!(vector_expression.clone()),
                    passed_by: PassedBy::Reference,
                }))],
            })),
        );

        let indexed_expression = Expression::Index {
            collection: Box::new(macro_node!(vector_expression.clone())),
            index: Box::new(macro_node!(Expression::Variable(index_variable.clone()))),
        };

        let mut while_statements = Vec::new();

        while_statements.push(macro_node!(Statement::Conditional {
            condition: macro_node!(Expression::Greater(
                Box::new(macro_node!(Expression::Variable(index_variable.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::I64(0)))),
            )),
            if_block: macro_node!(Block(vec![macro_node!(Statement::Assignment {
                identifier: macro_node!(result_variable.to_owned()),
                accessors: vec![],
                value: macro_node!(Expression::Addition(
                    Box::new(macro_node!(Expression::Variable(result_variable.to_owned()))),
                    Box::new(macro_node!(Expression::Literal(Literal::String(", ".to_owned())))),
                )),
            })])),
            else_block: None,
        }));

        match inner_type {
            Type::Vector(nested_inner_type) => {
                let nested_variable = format!("{}_nested", result_variable);

                let nested_statements = self.vector_debug_statements(&nested_variable, indexed_expression, nested_inner_type);

                while_statements.extend(nested_statements);

                while_statements.push(macro_node!(Statement::Assignment {
                    identifier: macro_node!(result_variable.to_owned()),
                    accessors: vec![],
                    value: macro_node!(Expression::Addition(
                        Box::new(macro_node!(Expression::Variable(result_variable.to_owned()))),
                        Box::new(macro_node!(Expression::Variable(nested_variable))),
                    )),
                }));
            }

            _ => {
                let element_expression = self.debug_expression_for_expression(inner_type, indexed_expression);

                while_statements.push(macro_node!(Statement::Assignment {
                    identifier: macro_node!(result_variable.to_owned()),
                    accessors: vec![],
                    value: macro_node!(Expression::Addition(
                        Box::new(macro_node!(Expression::Variable(result_variable.to_owned()))),
                        Box::new(macro_node!(element_expression)),
                    )),
                }));
            }
        }

        while_statements.push(macro_node!(Statement::Assignment {
            identifier: macro_node!(index_variable.clone()),
            accessors: vec![],
            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(index_variable.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::I64(1)))),
            )),
        }));

        statements.push(macro_node!(Statement::WhileLoop {
            condition: macro_node!(condition),
            block: macro_node!(Block(while_statements)),
        }));

        statements.push(macro_node!(Statement::Assignment {
            identifier: macro_node!(result_variable.to_owned()),
            accessors: vec![],
            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(result_variable.to_owned()))),
                Box::new(macro_node!(Expression::Literal(Literal::String("]".to_owned())))),
            )),
        }));

        statements
    }

    fn enum_debug_block(&self, enum_declaration: &crate::frontend::ast::EnumDeclaration) -> Block {
        let enum_name = enum_declaration.identifier.value.clone();
        let variable_name = to_snake_case(enum_name.as_str());
        let mut match_arms = Vec::new();

        for member in &enum_declaration.members {
            let variant_name = member.value.identifier.value.clone();

            match &member.value.member_type {
                None => {
                    match_arms.push(macro_node!(MatchArm {
                        enum_name: macro_node!(enum_name.clone()),
                        variant_name: macro_node!(variant_name.clone()),
                        variant_value: None,
                        block: macro_node!(Block(vec![macro_node!(Statement::Return(Some(macro_node!(Expression::Literal(
                            Literal::String(format!("{}::{}", enum_name, variant_name))
                        )))))]))
                    }));
                }

                Some(member_type) => {
                    let member_type = member_type.value.clone();
                    let payload_variable = self.debug_variable_name(&member_type);

                    if let Type::Vector(inner_type) = member_type {
                        let vector_block = self.vector_enum_debug_block(&enum_name, &variant_name, &payload_variable, &inner_type);

                        match_arms.push(macro_node!(MatchArm {
                            enum_name: macro_node!(enum_name.clone()),
                            variant_name: macro_node!(variant_name),
                            variant_value: Some(macro_node!(payload_variable)),
                            block: macro_node!(vector_block),
                        }));

                        continue;
                    }

                    let debug_expression = self.debug_expression_for_variable(&member_type, &payload_variable);

                    let return_expression = Expression::Addition(
                        Box::new(macro_node!(Expression::Literal(Literal::String(format!(
                            "{}::{}(",
                            enum_name, variant_name
                        ))))),
                        Box::new(macro_node!(Expression::Addition(
                            Box::new(macro_node!(debug_expression)),
                            Box::new(macro_node!(Expression::Literal(Literal::String(")".to_owned())))),
                        ))),
                    );

                    match_arms.push(macro_node!(MatchArm {
                        enum_name: macro_node!(enum_name.clone()),
                        variant_name: macro_node!(variant_name),
                        variant_value: Some(macro_node!(payload_variable.clone())),
                        block: macro_node!(Block(vec![macro_node!(Statement::Return(Some(macro_node!(return_expression))))])),
                    }));
                }
            }
        }

        Block(vec![macro_node!(Statement::Match {
            expression: macro_node!(Expression::Variable(variable_name)),
            match_arms,
            rest_arm: None,
        })])
    }

    fn vector_enum_debug_block(&self, enum_name: &str, variant_name: &str, vector_variable: &str, inner_type: &Type) -> Block {
        let vector_str = "vector_str".to_owned();
        let index = "i".to_owned();
        let mut statements = Vec::new();

        statements.push(macro_node!(Statement::Declaration {
            identifier: macro_node!(vector_str.clone()),
            kind: VariableDeclarationKind::LET {
                var_type: None,
                value: macro_node!(Expression::Literal(Literal::String("[".to_owned()))),
            },
        }));

        statements.push(macro_node!(Statement::Declaration {
            identifier: macro_node!(index.clone()),
            kind: VariableDeclarationKind::LET {
                var_type: None,
                value: macro_node!(Expression::Literal(Literal::I64(0))),
            },
        }));

        let indexed_expression = Expression::Index {
            collection: Box::new(macro_node!(Expression::Variable(vector_variable.to_owned()))),
            index: Box::new(macro_node!(Expression::Variable(index.clone()))),
        };

        let condition = Expression::Less(
            Box::new(macro_node!(Expression::Variable(index.clone()))),
            Box::new(macro_node!(Expression::FunctionCall {
                identifier: macro_node!("vector_size".to_owned()),
                arguments: vec![Box::new(macro_node!(Argument {
                    value: macro_node!(Expression::Variable(vector_variable.to_owned())),
                    passed_by: PassedBy::Reference,
                }))],
            })),
        );

        let mut while_statements = Vec::new();

        while_statements.push(macro_node!(Statement::Conditional {
            condition: macro_node!(Expression::Greater(
                Box::new(macro_node!(Expression::Variable(index.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::I64(0)))),
            )),
            if_block: macro_node!(Block(vec![macro_node!(Statement::Assignment {
                identifier: macro_node!(vector_str.clone()),
                accessors: vec![],
                value: macro_node!(Expression::Addition(
                    Box::new(macro_node!(Expression::Variable(vector_str.clone()))),
                    Box::new(macro_node!(Expression::Literal(Literal::String(", ".to_owned())))),
                )),
            })])),
            else_block: None,
        }));

        match inner_type {
            Type::Vector(nested_inner_type) => {
                let nested_variable = "nested_vector_str".to_owned();

                let nested_statements = self.vector_debug_statements(&nested_variable, indexed_expression, nested_inner_type);

                while_statements.extend(nested_statements);

                while_statements.push(macro_node!(Statement::Assignment {
                    identifier: macro_node!(vector_str.clone()),
                    accessors: vec![],
                    value: macro_node!(Expression::Addition(
                        Box::new(macro_node!(Expression::Variable(vector_str.clone()))),
                        Box::new(macro_node!(Expression::Variable(nested_variable))),
                    )),
                }));
            }

            _ => {
                let element_expression = self.debug_expression_for_expression(inner_type, indexed_expression);

                while_statements.push(macro_node!(Statement::Assignment {
                    identifier: macro_node!(vector_str.clone()),
                    accessors: vec![],
                    value: macro_node!(Expression::Addition(
                        Box::new(macro_node!(Expression::Variable(vector_str.clone()))),
                        Box::new(macro_node!(element_expression)),
                    )),
                }));
            }
        }

        while_statements.push(macro_node!(Statement::Assignment {
            identifier: macro_node!(index.clone()),
            accessors: vec![],
            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(index.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::I64(1)))),
            )),
        }));

        statements.push(macro_node!(Statement::WhileLoop {
            condition: macro_node!(condition),
            block: macro_node!(Block(while_statements)),
        }));

        statements.push(macro_node!(Statement::Assignment {
            identifier: macro_node!(vector_str.clone()),
            accessors: vec![],
            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(vector_str.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::String("]".to_owned())))),
            )),
        }));

        let return_expression = Expression::Addition(
            Box::new(macro_node!(Expression::Literal(Literal::String(format!(
                "{}::{}(",
                enum_name, variant_name
            ))))),
            Box::new(macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(vector_str))),
                Box::new(macro_node!(Expression::Literal(Literal::String(")".to_owned())))),
            ))),
        );

        statements.push(macro_node!(Statement::Return(Some(macro_node!(return_expression)))));

        Block(statements)
    }

    fn debug_expression_for_variable(&self, member_type: &Type, variable_name: &str) -> Expression {
        self.debug_expression_for_expression(member_type, Expression::Variable(variable_name.to_owned()))
    }

    fn debug_expression_for_expression(&self, member_type: &Type, expression: Expression) -> Expression {
        match member_type {
            Type::Str => Expression::Addition(
                Box::new(macro_node!(Expression::Literal(Literal::String("\"".to_owned())))),
                Box::new(macro_node!(Expression::Addition(
                    Box::new(macro_node!(expression)),
                    Box::new(macro_node!(Expression::Literal(Literal::String("\"".to_owned())))),
                ))),
            ),

            Type::Char => Expression::Addition(
                Box::new(macro_node!(Expression::Literal(Literal::String("'".to_owned())))),
                Box::new(macro_node!(Expression::Addition(
                    Box::new(macro_node!(expression)),
                    Box::new(macro_node!(Expression::Literal(Literal::String("'".to_owned())))),
                ))),
            ),

            Type::Bool | Type::F64 | Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::U8 | Type::U16 | Type::U32 | Type::U64 => {
                Expression::Casting {
                    value: Box::new(macro_node!(expression)),
                    to_type: macro_node!(Type::Str),
                }
            }

            Type::Unresolved(identifier) => self.debug_function_call(identifier, expression),

            Type::Struct { identifier, .. } => self.debug_function_call(identifier, expression),

            Type::Enum { identifier, .. } => self.debug_function_call(identifier, expression),

            Type::Vector(_) => {
                unreachable!("Vector requires statement-level debug generation")
            }

            Type::Any | Type::Void => {
                unreachable!("Any and Void cannot be debugged")
            }
        }
    }

    fn debug_function_call(&self, identifier: &str, expression: Expression) -> Expression {
        let debug_fn_name = format!("{}_debug", to_snake_case(identifier));

        Expression::FunctionCall {
            identifier: macro_node!(debug_fn_name),
            arguments: vec![Box::new(macro_node!(Argument {
                value: macro_node!(expression),
                passed_by: PassedBy::Reference,
            }))],
        }
    }

    fn debug_variable_name(&self, member_type: &Type) -> String {
        match member_type {
            Type::Any => "any_var",
            Type::Bool => "bool_var",
            Type::Str => "str_var",
            Type::Char => "char_var",
            Type::I8 => "i8_var",
            Type::I16 => "i16_var",
            Type::I32 => "i32_var",
            Type::I64 => "i64_var",
            Type::U8 => "u8_var",
            Type::U16 => "u16_var",
            Type::U32 => "u32_var",
            Type::U64 => "u64_var",
            Type::F64 => "f64_var",
            Type::Void => "void_var",
            Type::Vector(_) => "vector_var",
            Type::Struct { .. } => "struct_var",
            Type::Enum { .. } => "enum_var",
            Type::Unresolved(_) => "unresolved_var",
        }
        .to_owned()
    }

    fn combine_string_expressions(&self, expressions: Vec<Expression>) -> Expression {
        let mut iter = expressions.into_iter();

        let first = iter.next().expect("cannot combine empty expression list");

        iter.fold(first, |left, right| {
            Expression::Addition(Box::new(macro_node!(left)), Box::new(macro_node!(right)))
        })
    }
}
