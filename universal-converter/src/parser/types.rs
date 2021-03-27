use crate::unit::types::Unit;

#[derive(Debug)]
pub struct ParseResult {
    pub from: Unit,
    pub to: Unit,
}
