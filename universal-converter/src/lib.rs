use crate::parser::parse;

mod convert;
mod parser;
mod unit;

/// Example
///
/// ```
/// assert_eq!(universal_converter::convert("2021-01-02 KST => PST").unwrap(),  "2021-01-02 00:00:00 KST => 2021-01-01 07:00:00 PST")
/// ```
pub fn convert(text: &str) -> Result<String, String> {
    let parsed = parse(text).map_err(|err| err.reason)?;
    convert::convert(parsed)
}
