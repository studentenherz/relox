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
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
}

pub enum Instruction {
    Return,
    Constant(Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Unknown(u8),
}

pub struct Chunk {
    instructions: Array<u8>,
    constants: Array<Value>,
    lines: Array<(usize, usize)>,
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
        if let Some((last_line, count)) = self.lines.last_mut() {
            if line == *last_line {
                *count += 1;
                return;
            }
        }

        let last_count = self.lines.last().unwrap_or(&(0, 0)).1;
        self.lines.push((line, last_count + 1));
    }

    fn write_bytes<const N: usize>(&mut self, mut number: usize, line: usize) {
        const {
            assert!(N <= 8);
        }

        for _ in 0..N {
            self.write((number & 0xff) as u8, line);
            number >>= 8;
        }
    }

    fn read_bytes<'a, const N: usize>(iter: &mut impl Iterator<Item = (usize, &'a u8)>) -> usize {
        const {
            assert!(N <= 8);
        }
        let mut number = 0usize;

        for (i, (_, value)) in iter.take(N).enumerate() {
            number |= (*value as usize) << (i * 8);
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
            self.write_bytes::<3>(constant_index, line);
        }
    }
}

pub struct ChunkIter<'a> {
    chunk: &'a Chunk,
    inner: std::iter::Enumerate<std::slice::Iter<'a, u8>>,
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = (usize, Instruction);
    fn next(&mut self) -> Option<Self::Item> {
        let (offset, &byte) = self.inner.next()?;

        let instruction = match OpCode::try_from(byte) {
            Ok(OpCode::Return) => Instruction::Return,
            Ok(OpCode::Constant) => {
                let index = Chunk::read_bytes::<1>(&mut self.inner);
                Instruction::Constant(self.chunk.constants[index])
            }
            Ok(OpCode::ConstantLong) => {
                let index = Chunk::read_bytes::<3>(&mut self.inner);
                Instruction::Constant(self.chunk.constants[index])
            }
            Ok(OpCode::Add) => Instruction::Add,
            Ok(OpCode::Subtract) => Instruction::Subtract,
            Ok(OpCode::Multiply) => Instruction::Multiply,
            Ok(OpCode::Divide) => Instruction::Divide,
            Ok(OpCode::Negate) => Instruction::Negate,
            Err(_) => Instruction::Unknown(byte),
        };

        Some((offset, instruction))
    }
}

impl<'a> ChunkIter<'a> {
    pub fn has_next(&self) -> bool {
        let mut peekable = self.inner.clone();
        peekable.next().is_some()
    }
}

impl<'a> Chunk {
    pub fn iter(&'a self) -> ChunkIter<'a> {
        ChunkIter {
            chunk: self,
            inner: self.instructions.iter().enumerate(),
        }
    }
}

#[cfg(feature = "tracing")]
mod tracing {
    use super::*;

    impl Chunk {
        fn get_line(&self, offset: usize) -> usize {
            for &(line, count) in self.lines.iter() {
                if offset < count {
                    return line;
                }
            }

            unreachable!()
        }

        fn simple_instruction(opcode: OpCode) {
            println!("{:?}", opcode)
        }

        fn constant_instruction<'a>(
            &self,
            opcode: OpCode,
            iter: &mut impl Iterator<Item = (usize, &'a u8)>,
        ) {
            let index = match opcode {
                OpCode::Constant => Self::read_bytes::<1>(iter),
                OpCode::ConstantLong => Self::read_bytes::<3>(iter),
                _ => unreachable!(),
            };
            let value = self.constants[index];
            println!("{:<16?} {:>4} '{}'", opcode, index, value);
        }

        pub fn disassemble_instruction<'a>(
            &self,
            iter: &mut impl Iterator<Item = (usize, &'a u8)>,
        ) -> Option<()> {
            if let Some((offset, opcode)) = iter.next() {
                print!("{:04} ", offset);
                let line = self.get_line(offset);
                let prev_line = self.get_line(offset.saturating_sub(1));
                if offset > 0 && line == prev_line {
                    print!("   | ");
                } else {
                    print!("{:>4} ", line);
                }
                match OpCode::try_from(*opcode) {
                    Ok(opcode @ (OpCode::Constant | OpCode::ConstantLong)) => {
                        self.constant_instruction(opcode, iter);
                    }
                    Ok(opcode) => {
                        Self::simple_instruction(opcode);
                    }
                    Err(_) => {
                        println!("Unknown opcode {}", opcode);
                    }
                }
                return Some(());
            }
            None
        }

        pub fn _disassemble(&self, name: &str) {
            println!("== {} ==", name);

            let mut iter = self.instructions.iter().enumerate();
            while self.disassemble_instruction(&mut iter).is_some() {}
        }
    }

    impl<'a> ChunkIter<'a> {
        pub fn disassemble_instruction(&'a self) {
            let mut peekable = self.inner.clone();
            self.chunk.disassemble_instruction(&mut peekable);
        }
    }
}
