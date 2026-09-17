use std::rc::Rc;

use crate::{
    common::{
        errors::{ErrorSeverity, IError, MacroExpanderError},
        span::Span,
        types::Type,
    },
    frontend::ast::{Block, DeclaredType, FunctionDeclaration, Node, Parameter, PassedBy, Program},
};

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
        let work: Vec<(String, Vec<Node<String>>)> = self
            .program
            .declared_types
            .iter()
            .map(|(type_name, type_declaration)| {
                let derives = match &type_declaration.value {
                    DeclaredType::Enum(enum_declaration) => enum_declaration.derives.clone(),
                    DeclaredType::Struct(struct_declaration) => struct_declaration.derives.clone(),
                };
                (type_name.clone(), derives)
            })
            .collect();

        for (type_name, derives) in work {
            self.expand_derive_macros_of_type(&type_name, &derives);
        }
    }

    fn expand_derive_macros_of_type(&mut self, type_name: &String, derives: &Vec<Node<String>>) {
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
                let debug_fn_name = format!("{}_debug", to_snake_case(type_name));
                let param_name = format!("{}", to_snake_case(type_name));

                let debug_fn = FunctionDeclaration {
                    identifier: Node {
                        value: debug_fn_name.clone(),
                        span: Span::default(),
                    },
                    parameters: vec![Node {
                        value: Parameter {
                            passed_by: PassedBy::Reference,
                            parameter_type: Node {
                                value: Type::Unresolved(type_name.clone()),
                                span: Span::default(),
                            },
                            identifier: Node {
                                value: param_name,
                                span: Span::default(),
                            },
                        },
                        span: Span::default(),
                    }],
                    return_type: Node {
                        value: Type::Str,
                        span: Span::default(),
                    },
                    block: Node {
                        value: Block(vec![]),
                        span: Span::default(),
                    },
                };

                self.program.functions.insert(
                    debug_fn_name,
                    Rc::new(Node {
                        value: debug_fn,
                        span: Span::default(),
                    }),
                );
            }

            eprintln!("Constructing derive '{}' on type '{}'", derive.value, type_name);
        }
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
