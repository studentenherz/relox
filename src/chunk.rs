use crate::array::Array;
use crate::value::Value;
use macros::DebugC;

#[repr(u8)]
#[derive(DebugC)]
#[prefix = "OP_"]
pub enum OpCode {
    Return,
    Constant,
}

impl TryFrom<u8> for OpCode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Return),
            1 => Ok(Self::Constant),
            _ => Err(()),
        }
    }
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

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn simple_instruction(opcode: OpCode) {
        println!("{:?}", opcode)
    }

    fn constant_instruction<'a>(
        &self,
        opcode: OpCode,
        iter: &mut impl Iterator<Item = (usize, &'a u8)>,
    ) {
        let (_, constant) = unsafe { iter.next().unwrap_unchecked() };
        let value = self.constants[*constant as usize];
        println!("{:<16?} {:4} '{}'", opcode, constant, value);
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
                print!("{:4} ", line);
            }
            prev_line = line;
            match OpCode::try_from(*instruction) {
                Ok(opcode @ OpCode::Return) => {
                    Self::simple_instruction(opcode);
                }
                Ok(opcode @ OpCode::Constant) => {
                    self.constant_instruction(opcode, &mut iter);
                }
                Err(_) => {
                    println!("Unknown opcode {}", instruction);
                }
            }
        }
    }
}
