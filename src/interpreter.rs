use crate::compiler::compile;
use crate::errors::InterpretError;
use crate::vm::Vm;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretError> {
        let chunk = compile(source)?;
        let mut vm = Vm::new(&chunk);
        vm.run()
    }
}

