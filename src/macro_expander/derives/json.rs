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
    pub(in crate::macro_expander) fn derive_json(&mut self, declared_type: &DeclaredType) {
        let (type_name, block) = match declared_type {
            DeclaredType::Enum(enum_declaration) => (enum_declaration.identifier.value.clone(), self.enum_json_block(enum_declaration)),

            DeclaredType::Struct(struct_declaration) => (struct_declaration.identifier.value.clone(), self.struct_json_block(struct_declaration)),
        };

        let json_fn_name = format!("{}_json_encode", to_snake_case(type_name.as_str()));
        let param_name = to_snake_case(type_name.as_str());

        let json_fn = FunctionDeclaration {
            identifier: macro_node!(json_fn_name.clone()),

            parameters: vec![macro_node!(Parameter {
                passed_by: PassedBy::Reference,

                parameter_type: macro_node!(Type::Unresolved(type_name.clone())),

                identifier: macro_node!(param_name),
            })],

            return_type: macro_node!(Type::Str),

            block: macro_node!(block),
        };

        self.program.functions.insert(json_fn_name, Rc::new(macro_node!(json_fn)));
    }

    // -------------------------------------------------------------------------
    // STRUCT JSON
    // -------------------------------------------------------------------------

    fn struct_json_block(&self, struct_declaration: &StructDeclaration) -> Block {
        let struct_name = struct_declaration.identifier.value.clone();
        let variable_name = to_snake_case(struct_name.as_str());

        let result_variable = "json_str".to_owned();

        let mut statements = Vec::new();

        /*
         * let json_str = "{";
         */
        statements.push(macro_node!(Statement::Declaration {
            identifier: macro_node!(result_variable.clone()),

            kind: VariableDeclarationKind::LET {
                var_type: None,

                value: macro_node!(Expression::Literal(Literal::String("{".to_owned()))),
            },
        }));

        for (index, member) in struct_declaration.members.iter().enumerate() {
            let field_name = member.value.identifier.value.clone();
            let field_type = member.value.member_type.value.clone();

            /*
             * ,
             */
            if index > 0 {
                statements.push(macro_node!(
                    self.append_to_string_statement(&result_variable, Expression::Literal(Literal::String(",".to_owned())),)
                ));
            }

            /*
             * "field_name":
             */
            let field_name_expression = Expression::Literal(Literal::String(format!("\"{}\":", field_name)));

            statements.push(macro_node!(self.append_to_string_statement(&result_variable, field_name_expression,)));

            /*
             * node.field
             */
            let field_expression = Expression::FieldAccess {
                instance: Box::new(macro_node!(Expression::Variable(variable_name.clone()))),

                field: macro_node!(field_name.clone()),
            };

            match field_type {
                /*
                 * Vectors require a loop, therefore they
                 * are generated as statements.
                 */
                Type::Vector(inner_type) => {
                    let vector_variable = format!("{}_{}_json_encode", variable_name, field_name);

                    let vector_statements = self.json_vector_statements(&vector_variable, field_expression, &inner_type);

                    statements.extend(vector_statements);

                    statements.push(macro_node!(
                        self.append_to_string_statement(&result_variable, Expression::Variable(vector_variable),)
                    ));
                }

                other_type => {
                    let value_expression = self.json_expression_for_expression(&other_type, field_expression);

                    statements.push(macro_node!(self.append_to_string_statement(&result_variable, value_expression,)));
                }
            }
        }

        /*
         * }
         */
        statements.push(macro_node!(
            self.append_to_string_statement(&result_variable, Expression::Literal(Literal::String("}".to_owned())),)
        ));

        /*
         * return json_str;
         */
        statements.push(macro_node!(Statement::Return(Some(macro_node!(Expression::Variable(result_variable))))));

        Block(statements)
    }

    // -------------------------------------------------------------------------
    // VECTOR JSON
    // -------------------------------------------------------------------------

    fn json_vector_statements(&self, result_variable: &str, vector_expression: Expression, inner_type: &Type) -> Vec<Node<Statement>> {
        let index_variable = format!("{}_i", result_variable);

        let mut statements = Vec::new();

        /*
         * let foo_json = "[";
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
         * while (
         *     foo_i < vector_size(&vector)
         * )
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
         *     foo_json = foo_json + ",";
         * }
         */
        let separator_if = Statement::Conditional {
            condition: macro_node!(Expression::Greater(
                Box::new(macro_node!(Expression::Variable(index_variable.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::I64(0)))),
            )),

            if_block: macro_node!(Block(vec![macro_node!(
                self.append_to_string_statement(result_variable, Expression::Literal(Literal::String(",".to_owned())),)
            )])),

            else_block: None,
        };

        /*
         * foo_json = foo_json + json(value);
         */
        let element_expression = self.json_expression_for_expression(inner_type, indexed_expression);

        let append_element = macro_node!(self.append_to_string_statement(result_variable, element_expression,));

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

        let while_block = Block(vec![macro_node!(separator_if), append_element, macro_node!(increment)]);

        statements.push(macro_node!(Statement::WhileLoop {
            condition: macro_node!(condition),

            block: macro_node!(while_block),
        }));

        /*
         * foo_json = foo_json + "]";
         */
        statements.push(macro_node!(
            self.append_to_string_statement(result_variable, Expression::Literal(Literal::String("]".to_owned())),)
        ));

        statements
    }

    // -------------------------------------------------------------------------
    // ENUM JSON
    // -------------------------------------------------------------------------

    fn enum_json_block(&self, enum_declaration: &EnumDeclaration) -> Block {
        let param_name = to_snake_case(enum_declaration.identifier.value.as_str());

        let mut match_arms = Vec::new();

        for member in &enum_declaration.members {
            let variant_name = member.value.identifier.value.clone();

            let member_type = member.value.member_type.as_ref().map(|member_type| member_type.value.clone());

            let variant_value_name = member_type.as_ref().map(|member_type| self.json_variable_name(member_type));

            let arm_block = match member_type.as_ref() {
                /*
                 * Active
                 *
                 * {"Active":null}
                 */
                None => {
                    let return_expression = Expression::Literal(Literal::String(format!("\"{}\"", variant_name)));

                    Block(vec![macro_node!(Statement::Return(Some(macro_node!(return_expression))))])
                }

                /*
                 * Variant containing Vector.
                 */
                Some(Type::Vector(inner_type)) => {
                    let vector_variable = variant_value_name.clone().unwrap();

                    self.vector_enum_json_block(&variant_name, &vector_variable, inner_type)
                }

                /*
                 * Variant containing a normal value.
                 */
                Some(member_type) => {
                    let value_variable = variant_value_name.clone().unwrap();

                    let value_expression = self.json_expression_for_variable(member_type, &value_variable);

                    let return_expression = Expression::Addition(
                        Box::new(macro_node!(Expression::Literal(Literal::String(format!("{{\"{}\":", variant_name))))),
                        Box::new(macro_node!(Expression::Addition(
                            Box::new(macro_node!(value_expression)),
                            Box::new(macro_node!(Expression::Literal(Literal::String("}".to_owned())))),
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

        /*
         * match enum_value {
         *     ...
         * }
         */
        Block(vec![macro_node!(Statement::Match {
            expression: macro_node!(Expression::Variable(param_name)),

            match_arms,

            rest_arm: None,
        })])
    }

    // -------------------------------------------------------------------------
    // ENUM VECTOR JSON
    // -------------------------------------------------------------------------

    fn vector_enum_json_block(&self, variant_name: &str, vector_variable: &str, inner_type: &Type) -> Block {
        let vector_str = "vector_json_encode".to_owned();

        let index = "i".to_owned();

        let mut statements = Vec::new();

        /*
         * let vector_json = "[";
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

        let element_expression = self.json_expression_for_expression(inner_type, indexed_expression);

        /*
         * separator
         */
        let separator = Statement::Conditional {
            condition: macro_node!(Expression::Greater(
                Box::new(macro_node!(Expression::Variable(index.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::I64(0)))),
            )),

            if_block: macro_node!(Block(vec![macro_node!(
                self.append_to_string_statement(&vector_str, Expression::Literal(Literal::String(",".to_owned())),)
            )])),

            else_block: None,
        };

        let append_expression = macro_node!(self.append_to_string_statement(&vector_str, element_expression,));

        /*
         * i = i + 1
         */
        let increment = Statement::Assignment {
            identifier: macro_node!(index.clone()),

            accessors: vec![],

            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(index.clone()))),
                Box::new(macro_node!(Expression::Literal(Literal::I64(1)))),
            )),
        };

        let while_block = Block(vec![macro_node!(separator), append_expression, macro_node!(increment)]);

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
         * vector_json = vector_json + "]";
         */
        statements.push(macro_node!(
            self.append_to_string_statement(&vector_str, Expression::Literal(Literal::String("]".to_owned())),)
        ));

        /*
         * return {"Variant":[...]}
         */
        let return_expression = Expression::Addition(
            Box::new(macro_node!(Expression::Literal(Literal::String(format!("{{\"{}\":", variant_name))))),
            Box::new(macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(vector_str))),
                Box::new(macro_node!(Expression::Literal(Literal::String("}".to_owned())))),
            ))),
        );

        statements.push(macro_node!(Statement::Return(Some(macro_node!(return_expression)))));

        Block(statements)
    }

    // -------------------------------------------------------------------------
    // GENERIC JSON EXPRESSION
    // -------------------------------------------------------------------------

    fn json_expression_for_variable(&self, member_type: &Type, variable_name: &str) -> Expression {
        self.json_expression_for_expression(member_type, Expression::Variable(variable_name.to_owned()))
    }

    fn json_expression_for_expression(&self, member_type: &Type, expression: Expression) -> Expression {
        match member_type {
            /*
             * String:
             *
             * "value"
             *
             * Same mechanism as Debug for now.
             */
            Type::Str => self.json_string_expression(expression),

            /*
             * JSON does not have a char type,
             * so serialize char as a string.
             */
            Type::Char => self.json_string_expression(Expression::Casting {
                value: Box::new(macro_node!(expression)),

                to_type: macro_node!(Type::Str),
            }),

            /*
             * bool -> "true" / "false"
             *
             * We cast because the result is ultimately
             * concatenated into a string.
             */
            Type::Bool => Expression::Casting {
                value: Box::new(macro_node!(expression)),

                to_type: macro_node!(Type::Str),
            },

            /*
             * Numbers -> string representation.
             */
            Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::F64 => Expression::Casting {
                value: Box::new(macro_node!(expression)),

                to_type: macro_node!(Type::Str),
            },

            /*
             * Nested user-defined types.
             */
            Type::Unresolved(identifier) => self.json_function_call(identifier, expression),

            Type::Struct { identifier, .. } => self.json_function_call(identifier, expression),

            Type::Enum { identifier, .. } => self.json_function_call(identifier, expression),

            /*
             * Vector requires statement-level
             * generation.
             */
            Type::Vector(_) => {
                unreachable!("Vector requires statement-level JSON generation")
            }

            Type::Void => {
                unreachable!("Void cannot be serialized to JSON")
            }

            Type::Any => {
                unreachable!("Any cannot be serialized to JSON")
            }
        }
    }

    // -------------------------------------------------------------------------
    // JSON STRING
    // -------------------------------------------------------------------------

    fn json_string_expression(&self, expression: Expression) -> Expression {
        /*
         * IMPORTANT:
         *
         * This intentionally behaves like Debug:
         *
         *     "\"" + value + "\""
         *
         * There is no json_escape() call here because
         * json_escape is not currently a language/runtime
         * function.
         *
         * Once you add a string escaping builtin, this
         * function is the only place that needs to change.
         */
        Expression::Addition(
            Box::new(macro_node!(Expression::Literal(Literal::String("\"".to_owned())))),
            Box::new(macro_node!(Expression::Addition(
                Box::new(macro_node!(expression)),
                Box::new(macro_node!(Expression::Literal(Literal::String("\"".to_owned())))),
            ))),
        )
    }

    // -------------------------------------------------------------------------
    // JSON FUNCTION CALL
    // -------------------------------------------------------------------------

    fn json_function_call(&self, identifier: &str, expression: Expression) -> Expression {
        let json_fn_name = format!("{}_json_encode", to_snake_case(identifier));

        Expression::FunctionCall {
            identifier: macro_node!(json_fn_name),

            arguments: vec![Box::new(macro_node!(Argument {
                value: macro_node!(expression),

                passed_by: PassedBy::Reference,
            }))],
        }
    }

    // -------------------------------------------------------------------------
    // VARIABLE NAMES
    // -------------------------------------------------------------------------

    fn json_variable_name(&self, member_type: &Type) -> String {
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

    // -------------------------------------------------------------------------
    // HELPERS
    // -------------------------------------------------------------------------

    fn append_to_string_statement(&self, result_variable: &str, expression: Expression) -> Statement {
        Statement::Assignment {
            identifier: macro_node!(result_variable.to_owned()),

            accessors: vec![],

            value: macro_node!(Expression::Addition(
                Box::new(macro_node!(Expression::Variable(result_variable.to_owned()))),
                Box::new(macro_node!(expression)),
            )),
        }
    }
}
