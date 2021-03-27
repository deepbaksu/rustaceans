use crate::common::UniversalError;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref YMD_REGEX: Regex = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
}

/// Unit Conversion 기본 단위 입니다.
/// Unit -> Unit 으로 변경할 때 사용됩니다.
#[derive(Debug)]
pub enum Unit {
    DateTimeWithTimeZone(DateTimeWithTimeZone),
    TimeZoneOnly(chrono_tz::Tz),
}

impl Unit {
    pub fn parse(text: &str) -> Result<Self, UniversalError> {
        if YMD_REGEX.is_match(text) {
            let split: Vec<&str> = text.split(' ').collect();
            if split.len() == 2 {
                let datetime_portion = split[0];
                let datetime_format = "%Y-%m-%d";
                let timezone = get_timezone(split[1])?;
                let is_datetime = false;

                return DateTimeWithTimeZone::parse(
                    &datetime_portion,
                    &datetime_format,
                    is_datetime,
                    timezone,
                )
                .map(Unit::DateTimeWithTimeZone);
            } else if split.len() == 3 {
                let datetime_portion = format!("{} {}", split[0], split[1]);
                let datetime_format = "%Y-%m-%d %H:%M:%S";
                let timezone = get_timezone(split[2])?;
                let is_datetime = true;

                return DateTimeWithTimeZone::parse(
                    &datetime_portion,
                    &datetime_format,
                    is_datetime,
                    timezone,
                )
                .map(Unit::DateTimeWithTimeZone);
            } else {
                return Err(UniversalError::ParseError(format!(
                    "%Y-%m-%d (timezone) is expected but received {}",
                    text
                )));
            }
        } else if let Ok(tz) = get_timezone(text) {
            return Ok(Unit::TimeZoneOnly(tz));
        }

        Err(UniversalError::ParseError(
            "unable to match any patterns".to_owned(),
        ))
    }
}

#[derive(Debug)]
pub struct DateTimeWithTimeZone {
    pub datetime: chrono::NaiveDateTime,
    pub timezone: chrono_tz::Tz,
}

impl DateTimeWithTimeZone {
    fn parse(
        text: &str,
        format_str: &str,
        has_time: bool,
        timezone: chrono_tz::Tz,
    ) -> Result<Self, UniversalError> {
        let naive_datetime = match has_time {
            true => chrono::NaiveDateTime::parse_from_str(text, format_str),
            false => {
                chrono::NaiveDate::parse_from_str(text, format_str).map(|d| d.and_hms(0, 0, 0))
            }
        }
        .map_err(|err| {
            UniversalError::ParseError(format!(
                "failed to parse into NativeDateTime. see {}",
                err.to_string()
            ))
        })?;

        Ok(Self {
            datetime: naive_datetime,
            timezone,
        })
    }
}

fn get_timezone(text: &str) -> Result<chrono_tz::Tz, UniversalError> {
    match text {
        "KST" => Ok(chrono_tz::Asia::Seoul),
        "PST" => Ok(chrono_tz::US::Pacific),
        "PDT" => Ok(chrono_tz::US::Pacific),
        _ => Err(UniversalError::ParseError(format!(
            "{} is not a recognized timezone",
            text
        ))),
    }
}
