use std::{collections::HashMap, rc::Rc};

use crate::{
    common::{errors::IError, types::Type},
    frontend::{
        ast::{DeclaredType, Node},
        lexer::lexer::ILexer,
        parser::Parser,
    },
};

impl<L: ILexer> Parser<L> {
    pub(in crate::frontend::parser) fn resolve_declared_types(
        declared_types: &HashMap<String, Rc<Node<DeclaredType>>>,
    ) -> Result<HashMap<String, Type>, Box<dyn IError>> {
        let mut types = HashMap::new();

        for (name, declared_type) in declared_types {
            let resolved_type = Self::resolve_type_from_declared_type(&declared_type.value)?;
            types.insert(name.clone(), resolved_type);
        }

        Ok(types)
    }

    fn resolve_type_from_declared_type(declared_type: &DeclaredType) -> Result<Type, Box<dyn IError>> {
        match declared_type {
            DeclaredType::Struct(struct_declaration) => {
                let mut fields = HashMap::new();

                for member in &struct_declaration.members {
                    fields.insert(member.value.identifier.value.clone(), member.value.member_type.value.clone());
                }

                Ok(Type::Struct {
                    identifier: struct_declaration.identifier.value.clone(),
                    fields,
                })
            }
        }
    }
}
