use crate::chunk::{Chunk, ChunkIter, Instruction};
use crate::errors::{InterpretError, RuntimeError};
use crate::stack::Stack;
use crate::value::Value;

const STACK_MAX: usize = 256;

type RuntimeResult<T> = Result<T, RuntimeError>;

pub struct Vm<'a> {
    ip: ChunkIter<'a>,
    stack: Stack<Value, STACK_MAX>,
}

impl<'a> Vm<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            ip: chunk.iter(),
            stack: Stack::new(),
        }
    }

    fn run_binary_operation(&mut self, op: fn(Value, Value) -> Value) -> RuntimeResult<()> {
        let right = self.stack.pop()?;
        let left = self.stack.pop()?;

        let result = op(left, right);
        self.stack.push(result)?;

        Ok(())
    }

    fn run_instruction(&mut self, instruction: Instruction) -> RuntimeResult<()> {
        match instruction {
            Instruction::Return => {
                let value = self.stack.pop()?;
                println!("{}", value);
                return Ok(());
            }
            Instruction::Add => self.run_binary_operation(|left, right| left + right)?,
            Instruction::Subtract => self.run_binary_operation(|left, right| left - right)?,
            Instruction::Multiply => self.run_binary_operation(|left, right| left * right)?,
            Instruction::Divide => self.run_binary_operation(|left, right| left / right)?,
            Instruction::Constant(value) => {
                self.stack.push(value)?;
            }
            Instruction::Negate => {
                let value = self.stack.top()?;
                *value = -*value;
            }
            Instruction::Unknown(byte) => {
                return Err(RuntimeError::new(format!("Unknown opcode {}", byte)));
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), InterpretError> {
        while self.ip.has_next() {
            #[cfg(feature = "tracing")]
            {
                println!("          \n{:?}", self.stack);
                self.ip.disassemble_instruction();
            }

            let instruction = unsafe { self.ip.next().unwrap_unchecked() };

            if let Err(err) = self.run_instruction(instruction) {
                eprintln!("{}", err.with_line(self.ip.line()));
                return Err(InterpretError::Runtime);
            }
        }

        Ok(())
    }
}
