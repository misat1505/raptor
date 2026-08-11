use crate::{ast::Program, errors::IError};

pub struct Compiler<'a> {
    program: &'a Program,
}

impl<'a> Compiler<'a> {
    pub fn new(program: &'a Program) -> Self {
        Compiler { program }
    }

    pub fn compile(&mut self) -> Result<(), Box<dyn IError>> {
        self.visit_program(self.program)
    }

    fn visit_program(&mut self, program: &'a Program) -> Result<(), Box<dyn IError>> {
        for _statement in &program.statements {}

        Ok(())
    }
}
