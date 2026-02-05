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

    for i in 1..100_000_000 {
        chunk.write(OpCode::Negate as u8, i + 2);
    }

    chunk.write(OpCode::Return as u8, 10_000_010);

    Vm::interpret(&chunk);
}
