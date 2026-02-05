use crate::chunk::{Chunk, ChunkIter, Instruction};
use crate::stack::Stack;
use crate::value::Value;

const STACK_MAX: usize = 256;

pub struct Vm<'a> {
    // chunk: &'a Chunk,
    ip: ChunkIter<'a>,
    stack: Stack<Value, STACK_MAX>,
}

pub enum InterpretResult {
    Ok,
    // CompileError,
    RuntimeError,
}

impl<'a> Vm<'a> {
    fn new(chunk: &'a Chunk) -> Self {
        Self {
            // chunk,
            ip: chunk.iter(),
            stack: Stack::new(),
        }
    }

    fn run_binary_operation(&mut self, op: fn(Value, Value) -> Value) {
        let right = self.stack.pop().expect("Stack underflow");
        let left = self.stack.pop().expect("Stack underflow");

        let result = op(left, right);
        self.stack.push(result).expect("Stack overflow");
    }

    fn run(&mut self) -> InterpretResult {
        while self.ip.has_next() {
            #[cfg(feature = "tracing")]
            {
                println!("          \n{:?}", self.stack);
                self.ip.disassemble_instruction();
            }

            let (_, instruction) = unsafe { self.ip.next().unwrap_unchecked() };

            match instruction {
                Instruction::Return => {
                    let value = self.stack.pop().expect("Stack underflow");
                    println!("{}", value);
                    return InterpretResult::Ok;
                }
                Instruction::Add => self.run_binary_operation(|left, right| left + right),
                Instruction::Subtract => self.run_binary_operation(|left, right| left - right),
                Instruction::Multiply => self.run_binary_operation(|left, right| left * right),
                Instruction::Divide => self.run_binary_operation(|left, right| left / right),
                Instruction::Constant(value) => {
                    self.stack.push(value).expect("Stack overflow");
                }
                Instruction::Negate => {
                    let value = self.stack.top().expect("Stack underflow");
                    *value = -*value;
                }
                Instruction::Unknown(byte) => {
                    eprintln!("Unknown opcode {}", byte);
                    return InterpretResult::RuntimeError;
                }
            }
        }

        InterpretResult::Ok
    }

    pub fn interpret(chunk: &'a Chunk) -> InterpretResult {
        let mut vm = Self::new(chunk);
        vm.run()
    }
}
