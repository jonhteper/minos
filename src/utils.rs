use crate::errors::MinosError;
use chrono::{NaiveDateTime, Utc};

pub const DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S";

/// Returns datetime with `%Y-%m-%d %H:%M:%S` format
pub fn formatted_datetime_now() -> Result<NaiveDateTime, MinosError> {
    Ok(string_as_datetime(
        &Utc::now().naive_utc().format(DATETIME_FMT).to_string(),
    )?)
}

/// Returns a datetime with `%Y-%m-%d %H:%M:%S` format, from `&str`
pub fn string_as_datetime(date: &str) -> Result<NaiveDateTime, MinosError> {
    Ok(NaiveDateTime::parse_from_str(&date, DATETIME_FMT)?)
}
