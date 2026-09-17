use crate::{
    common::errors::{ErrorSeverity, IError, MacroExpanderError},
    frontend::ast::{DeclaredType, Node, Program},
};

pub struct MacroExpander<'a> {
    pub(in crate::macro_expander::macro_expander) program: &'a Program,
    supported_derives: Vec<String>,
    pub errors: Vec<Box<dyn IError>>,
}

impl<'a> MacroExpander<'a> {
    pub fn new(program: &'a Program) -> Self {
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
        for (type_name, type_declaration) in &self.program.declared_types {
            match type_declaration.value {
                DeclaredType::Enum(ref enum_declaration) => {
                    self.expand_derive_macros_of_type(&type_name, &enum_declaration.derives);
                }
                DeclaredType::Struct(ref struct_declaration) => {
                    self.expand_derive_macros_of_type(&type_name, &struct_declaration.derives);
                }
            }
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

            eprintln!("Constructing derive '{}' on type '{}'", derive.value, type_name);
        }
    }
}
