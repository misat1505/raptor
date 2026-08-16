use std::{collections::HashMap, rc::Rc};

use crate::{
    backend::std_functions::std_functions::get_std_functions,
    common::errors::{ErrorSeverity, IError, ParserError},
    frontend::{
        ast::{ExternFunctionDeclaration, FunctionDeclaration, Node, Program, Statement},
        lexer::lexer::ILexer,
        parser::Parser,
        tokens::TokenCategory,
    },
};

impl<L: ILexer> Parser<L> {
    pub fn parse_program(&mut self) -> Result<Program, Box<dyn IError>> {
        /// program = { function_declaration | extern_function_declaration | assign_or_call
        ///           | if_statement | for_statement | while_statement | switch_statement
        ///           | declaration, ";" };
        let mut statements: Vec<Node<Statement>> = vec![];
        let mut functions: HashMap<String, Rc<Node<FunctionDeclaration>>> = HashMap::new();
        let std_functions = get_std_functions();
        let mut extern_functions: HashMap<String, Rc<Node<ExternFunctionDeclaration>>> = HashMap::new();

        loop {
            if let Some(statement) = self.parse_program_statement()? {
                statements.push(statement);
            } else if let Some(function_declaration) = self.parse_function_declaration()? {
                let function_name = function_declaration.value.identifier.value.clone();
                if functions.contains_key(&function_name)
                    || std_functions.contains_key(&function_name)
                    || extern_functions.contains_key(&function_name)
                {
                    return Err(Box::new(ParserError::new(
                        ErrorSeverity::HIGH,
                        format!("Redeclaration of function '{}'.\nAt: {:?}.", function_name, function_declaration.position),
                    )));
                }
                functions.insert(function_name, Rc::new(function_declaration));
            } else if let Some(function_declaration) = self.parse_extern_function_declaration()? {
                let function_name = function_declaration
                    .value
                    .alias
                    .as_ref()
                    .unwrap_or(&function_declaration.value.identifier)
                    .value
                    .clone();

                if functions.contains_key(&function_name)
                    || std_functions.contains_key(&function_name)
                    || extern_functions.contains_key(&function_name)
                {
                    return Err(Box::new(ParserError::new(
                        ErrorSeverity::HIGH,
                        format!("Redeclaration of function '{}'.\nAt: {:?}.", function_name, function_declaration.position),
                    )));
                }

                extern_functions.insert(function_name, Rc::new(function_declaration));
            } else {
                break;
            }
        }

        self.consume_must_be(TokenCategory::ETX)?;

        Ok(Program {
            statements,
            functions,
            std_functions,
            extern_functions,
        })
    }

    pub fn parse_program_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        /// program_statement = assign_or_call | if_statement | for_statement | while_statement
        ///                    | switch_statement | declaration, ";";
        let generators = [
            Self::parse_assign_or_call,
            Self::parse_if_statement,
            Self::parse_for_statement,
            Self::parse_while_statement,
            Self::parse_switch_statement,
            Self::parse_variable_declaration,
        ];

        for generator in &generators {
            if let Some(statement) = generator(self)? {
                return Ok(Some(statement));
            }
        }

        Ok(None)
    }

    pub fn parse_statement(&mut self) -> Result<Option<Node<Statement>>, Box<dyn IError>> {
        let generators = [
            Self::parse_assign_or_call,
            Self::parse_if_statement,
            Self::parse_for_statement,
            Self::parse_while_statement,
            Self::parse_switch_statement,
            Self::parse_return_statement,
            Self::parse_break_statement,
            Self::parse_continue_statement,
            Self::parse_variable_declaration,
        ];

        for generator in &generators {
            if let Some(statement) = generator(self)? {
                return Ok(Some(statement));
            }
        }

        Ok(None)
    }
}
