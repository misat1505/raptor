use std::rc::Rc;

use crate::{
    common::types::Type,
    frontend::ast::{
        Argument, Block, DeclaredType, EnumDeclaration, Expression, FunctionDeclaration, Literal, MatchArm, Node, Parameter, PassedBy, Statement,
        StructDeclaration, VariableDeclarationKind,
    },
    macro_expander::macro_expander::{macro_node, to_snake_case, MacroExpander},
};

impl<'a> MacroExpander<'a> {
    pub(in crate::macro_expander) fn derive_debug(&mut self, declared_type: &DeclaredType) {
        let (type_name, block) = match declared_type {
            DeclaredType::Enum(enum_declaration) => (enum_declaration.identifier.value.clone(), self.enum_debug_block(enum_declaration)),
            DeclaredType::Struct(struct_declaration) => (struct_declaration.identifier.value.clone(), self.struct_debug_block(struct_declaration)),
        };

        let debug_fn_name = format!("{}_debug", to_snake_case(type_name.as_str()));
        let param_name = to_snake_case(type_name.as_str());

        let debug_fn = FunctionDeclaration {
            identifier: macro_node!(debug_fn_name.clone()),

            parameters: vec![macro_node!(Parameter {
                passed_by: PassedBy::Reference,

                parameter_type: macro_node!(Type::Unresolved(type_name.clone())),

                identifier: macro_node!(param_name),
            })],

            return_type: macro_node!(Type::Str),
            block: macro_node!(block),
        };

        self.program.functions.insert(debug_fn_name, Rc::new(macro_node!(debug_fn)));
    }

    // -------------------------------------------------------------------------
    // STRUCT DEBUG
    // -------------------------------------------------------------------------

    fn struct_debug_block(&self, struct_declaration: &StructDeclaration) -> Block {
        let struct_name = struct_declaration.identifier.value.clone();

        let variable_name = to_snake_case(struct_name.as_str());

        let mut expressions = Vec::new();

        /*
         * "Node { "
         */
        expressions.push(Expression::Literal(Literal::String(format!("{} {{ ", struct_name))));

        for (index, member) in struct_declaration.members.iter().enumerate() {
            let field_name = member.value.identifier.value.clone();

            let field_type = member.value.member_type.value.clone();

            /*
             * ", "
             */
            if index > 0 {
                expressions.push(Expression::Literal(Literal::String(", ".to_owned())));
            }

            /*
             * "field_name: "
             */
            expressions.push(Expression::Literal(Literal::String(format!("{}: ", field_name))));

            /*
             * node.field
             */
            let field_expression = Expression::FieldAccess {
                instance: Box::new(macro_node!(Expression::Variable(variable_name.clone()))),

                field: macro_node!(field_name.clone()),
            };

            /*
             * Format the actual value.
             *
             * For vectors this generates a helper block
             * instead, therefore vectors are handled below.
             */
            match field_type {
                Type::Vector(inner_type) => {
                    /*
                     * We cannot put a while loop directly into
                     * an Expression. Generate a temporary string
                     * variable containing the vector representation.
                     */
                    let vector_variable = format!("{}_{}_str", variable_name, field_name);

                    let _ = self.struct_vector_debug_statements(&vector_variable, field_expression, &inner_type);

                    /*
                     * The vector block is inserted separately below.
                     *
                     * We need to build the struct function as a
                     * sequence of statements rather than a single
                     * return expression.
                     */
                    return self.struct_debug_block_with_vectors(struct_declaration, &variable_name);
                }

                other_type => {
                    expressions.push(self.debug_expression_for_expression(&other_type, field_expression));
                }
            }
        }

        /*
         * " }"
         */
        expressions.push(Expression::Literal(Literal::String(" }".to_owned())));

        let return_expression = self.combine_string_expressions(expressions);

        Block(vec![macro_node!(Statement::Return(Some(macro_node!(return_expression))))])
    }

    fn struct_debug_block_with_vectors(&self, struct_declaration: &StructDeclaration, variable_name: &str) -> Block {
        let struct_name = struct_declaration.identifier.value.clone();

        let result_variable = "struct_str".to_owned();

        let mut statements = Vec::new();

        /*
         * let struct_str = "Node { ";
         */
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

            /*
             * Separator.
             */
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

            /*
             * "field_name: "
             */
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

            match field_type {
                Type::Vector(inner_type) => {
                    let vector_variable = format!("{}_{}_str", variable_name, field_name);

                    let vector_statements = self.struct_vector_debug_statements(&vector_variable, field_expression, &inner_type);

                    statements.extend(vector_statements);

                    /*
                     * struct_str = struct_str + vector_str;
                     */
                    statements.push(macro_node!(Statement::Assignment {
                        identifier: macro_node!(result_variable.clone()),

                        accessors: vec![],

                        value: macro_node!(Expression::Addition(
                            Box::new(macro_node!(Expression::Variable(result_variable.clone()))),
                            Box::new(macro_node!(Expression::Variable(vector_variable))),
                        )),
                    }));
                }

                other_type => {
                    let value_expression = self.debug_expression_for_expression(&other_type, field_expression);

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
        }

        /*
         * struct_str = struct_str + " }";
         */
        statements.push(macro_node!(Statement::Assignment {
            identifier: macro_node!(result_variable.clone()),

            accessors: vec![],

            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(result_variable.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::String(" }".to_owned())))),
            )),
        }));

        /*
         * return struct_str;
         */
        statements.push(macro_node!(Statement::Return(Some(macro_node!(Expression::Variable(result_variable))))));

        Block(statements)
    }

    // -------------------------------------------------------------------------
    // VECTOR DEBUG
    // -------------------------------------------------------------------------

    fn struct_vector_debug_statements(&self, result_variable: &str, vector_expression: Expression, inner_type: &Type) -> Vec<Node<Statement>> {
        let index_variable = format!("{}_i", result_variable);

        let mut statements = Vec::new();

        /*
         * let foo_str = "[";
         */
        statements.push(macro_node!(Statement::Declaration {
            identifier: macro_node!(result_variable.to_owned()),

            kind: VariableDeclarationKind::LET {
                var_type: None,

                value: macro_node!(Expression::Literal(Literal::String("[".to_owned()))),
            },
        }));

        /*
         * let foo_i = 0;
         */
        statements.push(macro_node!(Statement::Declaration {
            identifier: macro_node!(index_variable.clone()),

            kind: VariableDeclarationKind::LET {
                var_type: None,

                value: macro_node!(Expression::Literal(Literal::I64(0))),
            },
        }));

        /*
         * while (i < vector_size(&vector)) {
         */
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
            collection: Box::new(macro_node!(vector_expression)),

            index: Box::new(macro_node!(Expression::Variable(index_variable.clone()))),
        };

        /*
         * if (i > 0) {
         *     foo_str = foo_str + ", ";
         * }
         */
        let separator_if = Statement::Conditional {
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
        };

        /*
         * foo_str = foo_str + debug(vector[i]);
         */
        let element_expression = self.debug_expression_for_expression(inner_type, indexed_expression);

        let append_element = Statement::Assignment {
            identifier: macro_node!(result_variable.to_owned()),

            accessors: vec![],

            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(result_variable.to_owned()))),
                Box::new(macro_node!(element_expression)),
            )),
        };

        /*
         * i = i + 1;
         */
        let increment = Statement::Assignment {
            identifier: macro_node!(index_variable.clone()),

            accessors: vec![],

            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(index_variable.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::I64(1)))),
            )),
        };

        let while_block = Block(vec![macro_node!(separator_if), macro_node!(append_element), macro_node!(increment)]);

        statements.push(macro_node!(Statement::WhileLoop {
            condition: macro_node!(condition),

            block: macro_node!(while_block),
        }));

        /*
         * foo_str = foo_str + "]";
         */
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

    // -------------------------------------------------------------------------
    // ENUM DEBUG
    // -------------------------------------------------------------------------

    fn enum_debug_block(&self, enum_declaration: &EnumDeclaration) -> Block {
        let param_name = to_snake_case(enum_declaration.identifier.value.as_str());

        let mut match_arms = vec![];

        for member in &enum_declaration.members {
            let variant_name = member.value.identifier.value.clone();

            let member_type = member.value.member_type.as_ref().map(|member_type| member_type.value.clone());

            let variant_value_name = member_type.as_ref().map(|member_type| self.debug_variable_name(member_type));

            let arm_block = match member_type.as_ref() {
                None => {
                    let return_expression = Expression::Literal(Literal::String(format!("{}::{}", enum_declaration.identifier.value, variant_name)));

                    Block(vec![macro_node!(Statement::Return(Some(macro_node!(return_expression))))])
                }

                Some(Type::Vector(inner_type)) => {
                    let vector_variable = variant_value_name.clone().unwrap();

                    self.vector_enum_debug_block(&enum_declaration.identifier.value, &variant_name, &vector_variable, inner_type)
                }

                Some(member_type) => {
                    let value_variable = variant_value_name.clone().unwrap();

                    let inner_expr = self.debug_expression_for_variable(member_type, &value_variable);

                    let return_expression = Expression::Addition(
                        Box::new(macro_node!(Expression::Literal(Literal::String(format!(
                            "{}::{}(",
                            enum_declaration.identifier.value, variant_name
                        ))))),
                        Box::new(macro_node!(Expression::Addition(
                            Box::new(macro_node!(inner_expr)),
                            Box::new(macro_node!(Expression::Literal(Literal::String(")".to_owned())))),
                        ))),
                    );

                    Block(vec![macro_node!(Statement::Return(Some(macro_node!(return_expression))))])
                }
            };

            let variant_value_node = variant_value_name.map(|value| macro_node!(value));

            let arm = MatchArm {
                enum_name: macro_node!(enum_declaration.identifier.value.clone()),

                variant_name: macro_node!(variant_name),

                variant_value: variant_value_node,

                block: macro_node!(arm_block),
            };

            match_arms.push(macro_node!(arm));
        }

        Block(vec![macro_node!(Statement::Match {
            expression: macro_node!(Expression::Variable(param_name)),

            match_arms,

            rest_arm: None,
        })])
    }

    fn vector_enum_debug_block(&self, enum_name: &str, variant_name: &str, vector_variable: &str, inner_type: &Type) -> Block {
        let vector_str = "vector_str".to_owned();

        let index = "i".to_owned();

        let mut statements = Vec::new();

        /*
         * let vector_str = "[";
         */
        statements.push(macro_node!(Statement::Declaration {
            identifier: macro_node!(vector_str.clone()),

            kind: VariableDeclarationKind::LET {
                var_type: None,

                value: macro_node!(Expression::Literal(Literal::String("[".to_owned()))),
            },
        }));

        /*
         * let i = 0;
         */
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

        let element_expression = self.debug_expression_for_expression(inner_type, indexed_expression);

        let append_expression = Statement::Assignment {
            identifier: macro_node!(vector_str.clone()),

            accessors: vec![],

            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(vector_str.clone()))),
                Box::new(macro_node!(element_expression)),
            )),
        };

        let separator = Statement::Conditional {
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
        };

        let increment = Statement::Assignment {
            identifier: macro_node!(index.clone()),

            accessors: vec![],

            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(index.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::I64(1)))),
            )),
        };

        let while_block = Block(vec![macro_node!(separator), macro_node!(append_expression), macro_node!(increment)]);

        let condition = Expression::Less(
            Box::new(macro_node!(Expression::Variable(index))),
            Box::new(macro_node!(Expression::FunctionCall {
                identifier: macro_node!("vector_size".to_owned()),

                arguments: vec![Box::new(macro_node!(Argument {
                    value: macro_node!(Expression::Variable(vector_variable.to_owned())),

                    passed_by: PassedBy::Reference,
                }))],
            })),
        );

        statements.push(macro_node!(Statement::WhileLoop {
            condition: macro_node!(condition),

            block: macro_node!(while_block),
        }));

        /*
         * vector_str = vector_str + "]";
         */
        statements.push(macro_node!(Statement::Assignment {
            identifier: macro_node!(vector_str.clone()),

            accessors: vec![],

            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(vector_str.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::String("]".to_owned())))),
            )),
        }));

        /*
         * return "Enum::Variant(" + vector_str + ")";
         */
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

    // -------------------------------------------------------------------------
    // GENERIC DEBUG EXPRESSION
    // -------------------------------------------------------------------------

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
