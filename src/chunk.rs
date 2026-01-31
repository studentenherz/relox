use crate::array::Array;
use crate::value::Value;
use macros::{DebugC, TryFromU8};

#[repr(u8)]
#[derive(DebugC, TryFromU8)]
#[prefix = "OP"]
pub enum OpCode {
    Return,
    Constant,
    ConstantLong,
}

pub struct Chunk {
    instructions: Array<u8>,
    constants: Array<Value>,
    lines: Array<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: Array::new(),
            constants: Array::new(),
            lines: Array::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.instructions.push(byte);
        let line = line - 1;
        if line < self.lines.len() {
            self.lines[line] += 1;
        } else {
            self.lines.push(self.lines.last().unwrap_or(&0) + 1);
        }
    }

    fn get_line(&self, offset: usize) -> usize {
        let mut line = 0;
        while offset >= self.lines[line] {
            line += 1;
        }
        line + 1
    }

    fn write_bytes(&mut self, n: usize, mut number: usize, line: usize) {
        assert!(n <= 8);

        for _ in 0..n {
            self.write((number & 0xff) as u8, line);
            number >>= 1;
        }
    }

    fn read_bytes<'a>(n: usize, iter: &mut impl Iterator<Item = (usize, &'a u8)>) -> usize {
        assert!(n <= 8);
        let mut number = 0usize;

        for (i, (_, value)) in iter.take(n).enumerate() {
            number |= (*value as usize) << i;
        }

        number
    }

    pub fn write_constant(&mut self, value: Value, line: usize) {
        let constant_index = self.constants.len();
        self.constants.push(value);
        if constant_index < 256 {
            self.write(OpCode::Constant as u8, line);
            self.write(constant_index as u8, line);
        } else {
            self.write(OpCode::ConstantLong as u8, line);
            self.write_bytes(3, constant_index, line);
        }
    }

    fn simple_instruction(opcode: OpCode) {
        println!("{:?}", opcode)
    }

    fn constant_instruction<'a>(
        &self,
        opcode: OpCode,
        iter: &mut impl Iterator<Item = (usize, &'a u8)>,
    ) {
        let bytes = match opcode {
            OpCode::Constant => 1,
            OpCode::ConstantLong => 3,
            _ => unreachable!(),
        };
        let index = Self::read_bytes(bytes, iter);
        let value = self.constants[index];
        println!("{:<16?} {:>4} '{}'", opcode, index, value);
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut iter = self.instructions.iter().enumerate();
        let mut prev_line = 0;
        while let Some((offset, instruction)) = iter.next() {
            print!("{:04} ", offset);
            let line = self.get_line(offset);
            if offset > 0 && line == prev_line {
                print!("   | ");
            } else {
                print!("{:>4} ", line);
            }
            prev_line = line;
            match OpCode::try_from(*instruction) {
                Ok(opcode @ OpCode::Return) => {
                    Self::simple_instruction(opcode);
                }
                Ok(opcode @ (OpCode::Constant | OpCode::ConstantLong)) => {
                    self.constant_instruction(opcode, &mut iter);
                }
                Err(_) => {
                    println!("Unknown opcode {}", instruction);
                }
            }
        }
    }
}
