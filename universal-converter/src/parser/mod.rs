use types::ParseResult;

use crate::common::UniversalError;
use crate::unit::types::Unit;

pub(crate) mod types;

pub fn parse(text: &str) -> Result<ParseResult, UniversalError> {
    let split: Vec<&str> = text.split(" => ").collect();

    if split.len() != 2 {
        return Err(UniversalError::ParseError(format!(
            "unable to parse {}",
            text
        )));
    }

    let first_unit = Unit::parse(split[0])?;
    let second_unit = Unit::parse(split[1])?;

    Ok(ParseResult {
        from: first_unit,
        to: second_unit,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() -> Result<(), UniversalError> {
        let parse_result = parse("2021-01-01 KST => PST")?;

        match (parse_result.from, parse_result.to) {
            (Unit::DateTimeWithTimeZone(_), Unit::TimeZoneOnly(_)) => {
                assert!(true)
            }
            _ => assert!(
                false,
                "it should have returned DateTimeWithTimeZone and TimeZoneOnly types"
            ),
        }

        Ok(())
    }
}
