use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::objects::LoxObject;

#[derive(Clone)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    Object(Rc<LoxObject>),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(bool) => f.pad(&format!("{:?}", bool)),
            Self::Nil => f.pad("nil"),
            Self::Number(number) => f.pad(&format!("{}", number)),
            Self::Object(obj) => f.pad(&format!("{:?}", obj)),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(bool) => f.pad(&format!("{:?}", bool)),
            Self::Nil => f.pad("nil"),
            Self::Number(number) => f.pad(&format!("{}", number)),
            Self::Object(obj) => f.pad(&obj.to_string()),
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

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(this), Self::Boolean(other)) => this == other,
            (Self::Nil, Self::Nil) => true,
            (Self::Number(this), Self::Number(other)) => this == other,
            (Self::Object(this), Self::Object(other)) => this == other,
            _ => false,
        }
    }
}

impl Value {
    pub fn string(string: String) -> Self {
        Self::Object(LoxObject::string(string).into())
    }
}
