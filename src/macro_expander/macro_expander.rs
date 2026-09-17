use crate::frontend::ast::{DeclaredType, Node, Program};

pub struct MacroExpander<'a> {
    pub(in crate::macro_expander::macro_expander) program: &'a Program,
}

impl<'a> MacroExpander<'a> {
    pub fn new(program: &'a Program) -> Self {
        MacroExpander { program }
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
            println!("Constructing derive '{}' on type '{}'", derive.value, type_name);
        }
    }
}
