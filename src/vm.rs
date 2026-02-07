use crate::chunk::{Chunk, ChunkIter, Instruction};
use crate::compiler::compile;
use crate::errors::{LoxError, LoxResult};
use crate::stack::Stack;
use crate::value::Value;

const STACK_MAX: usize = 256;

pub struct Vm<'a> {
    // chunk: &'a Chunk,
    ip: ChunkIter<'a>,
    stack: Stack<Value, STACK_MAX>,
}

impl<'a> Vm<'a> {
    fn new(chunk: &'a Chunk) -> Self {
        Self {
            // chunk,
            ip: chunk.iter(),
            stack: Stack::new(),
        }
    }

    fn run_binary_operation(&mut self, op: fn(Value, Value) -> Value) -> LoxResult<()> {
        let right = self.stack.pop()?;
        let left = self.stack.pop()?;

        let result = op(left, right);
        self.stack.push(result)?;

        Ok(())
    }

    fn run(&'a mut self) -> LoxResult<()> {
        while self.ip.has_next() {
            #[cfg(feature = "tracing")]
            {
                println!("          \n{:?}", self.stack);
                self.ip.disassemble_instruction();
            }

            let (_, instruction) = unsafe { self.ip.next().unwrap_unchecked() };

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
                    return Err(LoxError::runtime(format!("Unknown opcode {}", byte)));
                }
            }
        }

        Ok(())
    }

    pub fn interpret(source: &'a str) -> LoxResult<()> {
        compile(&source);
        Ok(())
        // let chunk = Chunk::new();
        // let mut vm = Self::new(&chunk);
        // vm.run()
    }
}
