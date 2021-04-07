pub(crate) mod types;

#[cfg(test)]
mod tests {
    use crate::common::UniversalError;
    use crate::unit::types::Unit;

    #[test]
    pub fn test_parse_datetime() -> Result<(), UniversalError> {
        let unit = Unit::parse("2021-01-01 KST")?;
        match unit {
            Unit::DateTimeWithTimeZone(dt) => {
                assert_eq!(dt.datetime.format("%Y-%m-%d").to_string(), "2021-01-01");
                assert_eq!(dt.timezone, chrono_tz::Asia::Seoul);
            }
            _ => assert!(
                false,
                "expected to parse as DateTimeWithTimeZone but did not parse any"
            ),
        }

        Ok(())
    }

    #[test]
    pub fn test_parse_timezone_only() -> Result<(), UniversalError> {
        let unit = Unit::parse("KST")?;
        match unit {
            Unit::TimeZoneOnly(tz) => assert_eq!(tz, chrono_tz::Asia::Seoul),
            _ => assert!(false, "expected to parse timezone but failed to parse"),
        }
        Ok(())
    }
}
