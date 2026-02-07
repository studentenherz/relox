use std::error::Error;
use std::fmt::Display;

use crate::scanner::ScannerError;
use crate::stack::StackError;

#[derive(Debug)]
pub enum ErrorKind {
    Compile,
    Runtime,
}

#[derive(Debug)]
pub struct LoxError {
    kind: ErrorKind,
    reason: String,
}

pub type LoxResult<T> = Result<T, LoxError>;

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for LoxError {}

impl From<std::io::Error> for LoxError {
    fn from(value: std::io::Error) -> Self {
        // let kind = value.kind();
        // let reason  = match kind {
        //     std::io::ErrorKind::NotFound => format(args)
        // }

        Self {
            kind: ErrorKind::Compile,
            reason: value.to_string(),
        }
    }
}

impl From<StackError> for LoxError {
    fn from(value: StackError) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            reason: value.to_string(),
        }
    }
}

impl From<ScannerError> for LoxError {
    fn from(value: ScannerError) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            reason: value.to_string(),
        }
    }
}

impl LoxError {
    pub fn compile(reason: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::Compile,
            reason: reason.into(),
        }
    }

    pub fn runtime(reason: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::Runtime,
            reason: reason.into(),
        }
    }

    pub fn exit(&self) {
        match self.kind {
            ErrorKind::Compile => std::process::exit(65),
            ErrorKind::Runtime => std::process::exit(70),
        }
    }
}
