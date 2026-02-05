use crate::chunk::{Chunk, OpCode};

use self::vm::Vm;

mod array;
mod chunk;
mod stack;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_constant(1.2, 1);
    chunk.write_constant(3.4, 123);
    chunk.write(OpCode::Add as u8, 123);
    chunk.write_constant(5.6, 123);
    chunk.write(OpCode::Divide as u8, 300);
    chunk.write(OpCode::Negate as u8, 300);

    chunk.write(OpCode::Return as u8, 300);

    Vm::interpret(&chunk);
}
