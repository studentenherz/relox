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

    fn run_instruction(&mut self, instruction: Instruction) -> RuntimeResult<()> {
        match instruction {
            Instruction::Return => {
                let value = self.stack.pop()?;
                println!("{}", value);
                return Ok(());
            }
            Instruction::Equal => self.try_run_equal_op()?,
            Instruction::Greater => self.try_run_comparison_op(|left, right| left > right)?,
            Instruction::Less => self.try_run_comparison_op(|left, right| left < right)?,

            Instruction::Add => self.try_run_arithmetic_op(|left, right| left + right)?,
            Instruction::Subtract => self.try_run_arithmetic_op(|left, right| left - right)?,
            Instruction::Multiply => self.try_run_arithmetic_op(|left, right| left * right)?,
            Instruction::Divide => self.try_run_arithmetic_op(|left, right| left / right)?,
            Instruction::Constant(value) => self.stack.push(value)?,
            Instruction::Not => {
                let value = self.stack.top()?;
                let not = Value::Boolean(!(bool::from(value)));
                let value = self.stack.top_mut()?;
                *value = not;
            }
            Instruction::Negate => {
                let value = self.stack.top_mut()?;
                Self::try_negate(value)?;
            }
            Instruction::Unknown(byte) => {
                return Err(RuntimeError::new(format!("Unknown opcode {}", byte)));
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), InterpretError> {
        while self.ip.has_next() {
            #[cfg(feature = "debug-trace-execution")]
            {
                println!("          \n{:?}", self.stack);
                self.ip.disassemble_instruction();
            }

            let instruction = unsafe { self.ip.next().unwrap_unchecked() };

            if let Err(mut err) = self.run_instruction(instruction) {
                eprintln!("{}", err.with_line(self.ip.line()));
                return Err(InterpretError::Runtime);
            }
        }

        Ok(())
    }

    fn try_run_arithmetic_op(&mut self, op: fn(f64, f64) -> f64) -> RuntimeResult<()> {
        let right = self.stack.pop()?;
        let left = self.stack.pop()?;

        match (left, right) {
            (Value::Number(left), Value::Number(right)) => {
                let result = op(left, right);
                self.stack.push(Value::Number(result))?;
            }
            _ => return Err(RuntimeError::new("Operands must be numbers.")),
        }

        Ok(())
    }

    fn try_run_comparison_op(&mut self, op: fn(f64, f64) -> bool) -> RuntimeResult<()> {
        let right = self.stack.pop()?;
        let left = self.stack.pop()?;

        match (left, right) {
            (Value::Number(left), Value::Number(right)) => {
                let result = op(left, right);
                self.stack.push(Value::Boolean(result))?;
            }
            _ => return Err(RuntimeError::new("Operands must be numbers.")),
        }

        Ok(())
    }

    fn try_run_equal_op(&mut self) -> RuntimeResult<()> {
        let right = self.stack.pop()?;
        let left = self.stack.pop()?;

        self.stack.push(Value::Boolean(left == right))?;

        Ok(())
    }

    fn try_negate(value: &mut Value) -> RuntimeResult<()> {
        match value {
            Value::Number(number) => *number = -*number,
            _ => return Err(RuntimeError::new("Operand must be a number.")),
        }

        Ok(())
    }
}
