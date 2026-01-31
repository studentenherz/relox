use crate::chunk::{Chunk, OpCode};

use self::value::Value;

mod array;
mod chunk;
mod value;

fn main() {
    let mut chunk = Chunk::new();
    for i in 1..10 {
        let constant = chunk.add_constant(1.1 * i as Value);
        chunk.write(OpCode::Constant as u8, i);
        chunk.write(constant as u8, i);
        chunk.write(OpCode::Return as u8, i);
    }
    chunk.disassemble("test chunk");
}
