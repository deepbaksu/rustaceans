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

#[derive(Debug)]
pub struct UnitParseError {
    pub reason: String,
}

impl UnitParseError {
    pub(crate) fn new(reason: String) -> UnitParseError {
        Self { reason }
    }
}

impl From<chrono::ParseError> for UnitParseError {
    fn from(chrono_error: chrono::ParseError) -> Self {
        Self {
            reason: chrono_error.to_string(),
        }
    }
}

impl Unit {
    pub fn parse(text: &str) -> Result<Self, UnitParseError> {
        if YMD_REGEX.is_match(text) {
            let mut split = text.split(' ');
            let naive_datetime = split
                .next()
                .ok_or_else(|| {
                    UnitParseError::new(
                        "%Y-%m-%d TIMEZONE is expected but received none".to_owned(),
                    )
                })
                .and_then(|text| {
                    chrono::NaiveDate::parse_from_str(text, "%Y-%m-%d")
                        .map(|d| d.and_hms(0, 0, 0))
                        .map_err(|err| {
                            UnitParseError::new(format!(
                                "failed to parse into NativeDateTime. see {}",
                                err.to_string()
                            ))
                        })
                })?;
            let timezone = split
                .next()
                .ok_or_else(|| {
                    UnitParseError::new(
                        "%Y-%m-%d TIMEZONE is expected but TIMEZONE was not found".to_string(),
                    )
                })
                .and_then(get_timezone)?;

            return Ok(Unit::DateTimeWithTimeZone(DateTimeWithTimeZone {
                datetime: naive_datetime,
                timezone,
            }));
        } else if let Ok(tz) = get_timezone(text) {
            return Ok(Unit::TimeZoneOnly(tz));
        }

        Err(UnitParseError::new(
            "unable to match any patterns".to_owned(),
        ))
    }
}

#[derive(Debug)]
pub struct DateTimeWithTimeZone {
    pub datetime: chrono::NaiveDateTime,
    pub timezone: chrono_tz::Tz,
}

fn get_timezone(text: &str) -> Result<chrono_tz::Tz, UnitParseError> {
    match text {
        "KST" => Ok(chrono_tz::Asia::Seoul),
        "PST" => Ok(chrono_tz::US::Pacific),
        "PDT" => Ok(chrono_tz::US::Pacific),
        _ => Err(UnitParseError::new(format!(
            "{} is not a recognized timezone",
            text
        ))),
    }
}
