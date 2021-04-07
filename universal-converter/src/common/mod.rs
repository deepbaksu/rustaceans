use crate::common::UniversalError::{ConversionError, ParseError};
use std::fmt;

/// UniversalError is the common Error type used in this crate.
#[derive(Debug)]
pub enum UniversalError {
    ConversionError(String),
    ParseError(String),
}

impl Into<String> for UniversalError {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for UniversalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError(msg) => write!(f, "ConversionError: {}", msg),
            ParseError(msg) => write!(f, "ParseError: {}", msg),
        }
    }
}
