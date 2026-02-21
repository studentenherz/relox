use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(bool) => f.pad(&format!("{:?}", bool)),
            Self::Nil => f.pad("nil"),
            Self::Number(number) => f.pad(&format!("{}", number)),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(bool) => f.pad(&format!("{:?}", bool)),
            Self::Nil => f.pad("nil"),
            Self::Number(number) => f.pad(&format!("{}", number)),
        }
    }
}

impl From<&Value> for bool {
    fn from(value: &Value) -> Self {
        match value {
            Value::Nil | Value::Boolean(false) => false,
            _ => true,
        }
    }
}
