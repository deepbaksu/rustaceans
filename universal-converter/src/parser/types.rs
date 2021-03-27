use crate::unit::types;
use crate::unit::types::{Unit, UnitParseError};

#[derive(Debug)]
pub struct ParseResult {
    pub from: Unit,
    pub to: Unit,
}

#[derive(Debug)]
pub struct ParseError {
    pub reason: String,
}

impl ParseError {
    pub(crate) fn new(reason: String) -> ParseError {
        Self { reason }
    }
}

impl From<types::UnitParseError> for ParseError {
    fn from(err: UnitParseError) -> Self {
        Self { reason: err.reason }
    }
}
