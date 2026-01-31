use crate::chunk::{Chunk, OpCode};

use self::value::Value;

mod array;
mod chunk;
mod value;

fn main() {
    let mut chunk = Chunk::new();
    for i in 1..258 {
        let constant = 1.1 * i as Value;
        chunk.write_constant(constant, i);
        chunk.write(OpCode::Return as u8, i);
    }
    chunk.disassemble("test chunk");
}
