use std::error::Error;
use std::fmt::Display;
use std::process::exit;

use crate::parser::ParserError;
use crate::stack::StackError;

#[derive(Debug)]
pub struct RuntimeError {
    reason: String,
    line: Option<usize>,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.reason)?;
        if let Some(line) = self.line {
            writeln!(f, "[line {}] in script", line)?;
        }

        Ok(())
    }
}

impl Error for RuntimeError {}

impl RuntimeError {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
            line: None,
        }
    }

    pub fn with_line(&mut self, line: usize) -> &mut Self {
        self.line = Some(line);
        self
    }
}

impl From<StackError> for RuntimeError {
    fn from(value: StackError) -> Self {
        Self {
            reason: value.to_string(),
            line: None,
        }
    }
}

pub enum InterpretError {
    Compile,
    Runtime,
}

impl InterpretError {
    pub fn exit(&self) {
        match self {
            Self::Compile => exit(65),
            Self::Runtime => exit(70),
        }
    }
}

impl From<ParserError> for InterpretError {
    fn from(_value: ParserError) -> Self {
        InterpretError::Compile
    }
}
