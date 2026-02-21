use std::fmt::{Debug, Display};

use crate::strings::LoxString;

pub enum LoxObject {
    String(LoxString),
}

impl Display for LoxObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(string) => f.pad(&string),
        }
    }
}

impl Debug for LoxObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(string) => f.pad(&format!("{:?}", string)),
        }
    }
}

impl PartialEq for LoxObject {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(this), Self::String(other)) => this == other,
        }
    }
}

impl LoxObject {
    pub fn string(string: String) -> Self {
        Self::String(string)
    }
}
