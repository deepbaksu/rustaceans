use crate::parser::parse;

mod common;
mod convert;
mod parser;
mod unit;

/// Entry function of this library.
///
/// Example
///
/// ```
/// assert_eq!(universal_converter::convert("2021-01-02 KST => PST").unwrap(),  "2021-01-02 00:00:00 KST => 2021-01-01 07:00:00 PST");
/// assert_eq!(universal_converter::convert("2021-01-02 00:00:00 KST => PST").unwrap(),  "2021-01-02 00:00:00 KST => 2021-01-01 07:00:00 PST");
/// ```
pub fn convert(text: &str) -> Result<String, String> {
    let parsed = parse(text).map_err(|err| err.to_string())?;
    convert::convert(parsed).map_err(|err| err.into())
}
