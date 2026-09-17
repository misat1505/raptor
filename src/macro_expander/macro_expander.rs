use std::rc::Rc;

use crate::{
    common::{
        errors::{ErrorSeverity, IError, MacroExpanderError},
        types::Type,
    },
    frontend::ast::{
        Argument, Block, DeclaredType, EnumDeclaration, Expression, FunctionDeclaration, Literal, MatchArm, Parameter, PassedBy, Program, Statement,
        StructDeclaration,
    },
};

macro_rules! macro_node {
    ($value:expr) => {
        crate::frontend::ast::Node {
            value: $value,
            span: crate::common::span::Span::default(),
        }
    };
}

pub struct MacroExpander<'a> {
    pub(in crate::macro_expander::macro_expander) program: &'a mut Program,
    supported_derives: Vec<String>,
    pub errors: Vec<Box<dyn IError>>,
}

impl<'a> MacroExpander<'a> {
    pub fn new(program: &'a mut Program) -> Self {
        let supported_derives = vec!["Debug".to_owned(), "Json".to_owned()];

        MacroExpander {
            program,
            supported_derives,
            errors: vec![],
        }
    }

    pub fn run(&mut self) {
        self.expand_derive_macros();
    }

    fn expand_derive_macros(&mut self) {
        let declared_types = self
            .program
            .declared_types
            .iter()
            .map(|(_, type_declaration)| type_declaration.value.clone())
            .collect::<Vec<_>>();

        for declared_type in declared_types {
            self.expand_derive_macros_of_type(&declared_type);
        }
    }

    fn expand_derive_macros_of_type(&mut self, declared_type: &DeclaredType) {
        let (type_name, derives) = match declared_type {
            DeclaredType::Enum(enum_declaration) => (enum_declaration.identifier.value.clone(), enum_declaration.derives.clone()),
            DeclaredType::Struct(struct_declaration) => (struct_declaration.identifier.value.clone(), struct_declaration.derives.clone()),
        };

        for derive in derives {
            let is_supported = self.supported_derives.iter().any(|supported_derive| supported_derive == &derive.value);

            if !is_supported {
                let available_derives = self.supported_derives.join(", ");

                self.errors.push(Box::new(MacroExpanderError::at(
                    ErrorSeverity::HIGH,
                    format!(
                        "Use of unsupported derive macro '{}' on type '{}'. Available derive macros: {}.",
                        derive.value, type_name, available_derives
                    ),
                    derive.span,
                )));

                continue;
            }

            if derive.value == "Debug" {
                let debug_fn_name = format!("{}_debug", to_snake_case(type_name.as_str()));
                let param_name = to_snake_case(type_name.as_str());

                let block = match declared_type {
                    DeclaredType::Enum(enum_declaration) => self.enum_debug_block(enum_declaration),
                    DeclaredType::Struct(struct_declaration) => self.struct_debug_block(struct_declaration),
                };

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

            eprintln!("Constructing derive '{}' on type '{}'", derive.value, type_name);
        }
    }

    fn struct_debug_block(&self, _struct_declaration: &StructDeclaration) -> Block {
        Block(vec![])
    }

    fn enum_debug_block(&self, enum_declaration: &EnumDeclaration) -> Block {
        let param_name = to_snake_case(enum_declaration.identifier.value.as_str());

        let mut match_arms = vec![];

        for member in &enum_declaration.members {
            let variant_name = member.value.identifier.value.clone();

            let variant_value_name = match member.value.member_type.as_ref() {
                None => None,

                Some(mt) => {
                    let name = match &mt.value {
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
                    };

                    Some(name.to_owned())
                }
            };

            let variant_return = match &variant_value_name {
                None => Statement::Return(Some(macro_node!(Expression::Literal(Literal::String(format!(
                    "{}::{}",
                    enum_declaration.identifier.value, variant_name
                )))))),

                Some(vvn) => {
                    let inner_expr = match member.value.member_type.as_ref().unwrap().value {
                        Type::Str => Expression::Addition(
                            Box::new(macro_node!(Expression::Literal(Literal::String("\"".to_owned(),)))),
                            Box::new(macro_node!(Expression::Addition(
                                Box::new(macro_node!(Expression::Variable(vvn.clone()))),
                                Box::new(macro_node!(Expression::Literal(Literal::String("\"".to_owned())))),
                            ))),
                        ),
                        Type::Char => Expression::Addition(
                            Box::new(macro_node!(Expression::Literal(Literal::String("\'".to_owned(),)))),
                            Box::new(macro_node!(Expression::Addition(
                                Box::new(macro_node!(Expression::Variable(vvn.clone()))),
                                Box::new(macro_node!(Expression::Literal(Literal::String("\'".to_owned())))),
                            ))),
                        ),
                        Type::Bool | Type::F64 | Type::I16 | Type::I32 | Type::I64 | Type::I8 | Type::U16 | Type::U32 | Type::U64 | Type::U8 => {
                            Expression::Casting {
                                value: Box::new(macro_node!(Expression::Variable(vvn.clone()))),
                                to_type: macro_node!(Type::Str),
                            }
                        }
                        Type::Unresolved(ident) => {
                            let member_debug_fn_name = to_snake_case(&ident);

                            Expression::FunctionCall {
                                identifier: macro_node!(member_debug_fn_name),
                                arguments: vec![Box::new(macro_node!(Argument {
                                    value: macro_node!(Expression::Variable(vvn.clone())),
                                    passed_by: PassedBy::Reference
                                }))],
                            }
                        }
                        // Type::Vector(inner) => Expression::FunctionCall { identifier: macro_node!("vector_stringify"), arguments: () }
                        Type::Any | Type::Void | Type::Enum { .. } | Type::Struct { .. } => unreachable!(),
                    };

                    Statement::Return(Some(macro_node!(Expression::Addition(
                        Box::new(macro_node!(Expression::Literal(Literal::String(format!(
                            "{}::{}(",
                            enum_declaration.identifier.value, variant_name
                        ))))),
                        Box::new(macro_node!(Expression::Addition(
                            Box::new(macro_node!(inner_expr)),
                            Box::new(macro_node!(Expression::Literal(Literal::String(")".to_owned())))),
                        )))
                    ))))
                }
            };

            let variant_value_node = variant_value_name.map(|value| macro_node!(value));

            let arm = MatchArm {
                enum_name: macro_node!(enum_declaration.identifier.value.clone()),
                variant_name: macro_node!(variant_name),
                variant_value: variant_value_node,
                block: macro_node!(Block(vec![macro_node!(variant_return),])),
            };

            match_arms.push(macro_node!(arm));
        }

        Block(vec![macro_node!(Statement::Match {
            expression: macro_node!(Expression::Variable(param_name)),
            match_arms,
            rest_arm: None,
        })])
    }
}

pub fn to_snake_case(value: &str) -> String {
    value
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                format!("_{}", c.to_lowercase())
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect()
}
