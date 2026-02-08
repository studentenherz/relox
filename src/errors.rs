use std::error::Error;
use std::fmt::Display;
use std::process::exit;

use crate::span::Span;
use crate::stack::StackError;

#[derive(Debug)]
pub struct CompileError<'a> {
    reason: &'static str,
    span: Span<'a>,
}

impl<'a> Display for CompileError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error at ", self.span.line)?;
        if self.span.slice.is_empty() {
            write!(f, "end")?;
        } else {
            write!(f, "'{}'", self.span.slice)?;
        }

        write!(f, ": {}", self.reason)
    }
}

impl<'a> Error for CompileError<'a> {}

impl<'a> CompileError<'a> {
    pub fn new(span: Span<'a>, reason: &'static str) -> Self {
        Self { reason, span }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    reason: String,
    line: Option<usize>,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(line) = self.line {
            write!(f, "[line {}] ", line)?;
        }
        write!(f, "Error: {}", self.reason)
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

    pub fn with_line(self, line: usize) -> Self {
        Self {
            line: Some(line),
            ..self
        }
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
