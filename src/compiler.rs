use crate::chunk::Chunk;
use crate::errors::InterpretError;
use crate::parser::Parser;

pub fn compile(source: &str) -> Result<Chunk, InterpretError> {
    let mut parser = Parser::new(source);
    let chunk = parser.parse()?;
    #[cfg(feature = "debug-print-code")]
    {
        chunk.disassemble("code");
    }

    Ok(chunk)
}
