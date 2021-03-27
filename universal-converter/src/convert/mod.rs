use crate::common::UniversalError;
use crate::parser::types::ParseResult;
use crate::unit::types::Unit;
use chrono::{DateTime, TimeZone};

pub fn convert(parse_result: ParseResult) -> Result<String, UniversalError> {
    match (parse_result.from, parse_result.to) {
        (Unit::DateTimeWithTimeZone(dt_wrapper), Unit::TimeZoneOnly(tz)) => {
            let from_time: DateTime<_> = dt_wrapper
                .timezone
                .from_local_datetime(&dt_wrapper.datetime)
                .single().ok_or_else(|| UniversalError::ConversionError(format!("the left datetime cannot be converted to DateTime. It's caused when the datetime cannot be converted into timezone. Input: {:?}", dt_wrapper)))?;
            let to_time = from_time.with_timezone(&tz);

            Ok(format!("{} => {}", from_time, to_time))
        }
        _ => Err(UniversalError::ConversionError(
            "Invalid parse_result is given such that it can't be converted into String".to_owned(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::types::DateTimeWithTimeZone;

    #[test]
    pub fn test_convert_date_to_date() {
        let parse_result = ParseResult {
            from: Unit::DateTimeWithTimeZone(DateTimeWithTimeZone {
                timezone: chrono_tz::Asia::Seoul,
                datetime: chrono::NaiveDate::from_ymd(2021, 1, 2).and_hms(0, 0, 0),
            }),
            to: Unit::TimeZoneOnly(chrono_tz::US::Pacific),
        };

        assert_eq!(
            convert(parse_result).unwrap(),
            format!("2021-01-02 00:00:00 KST => 2021-01-01 07:00:00 PST")
        );
    }
}
