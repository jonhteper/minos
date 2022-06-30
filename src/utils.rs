use crate::errors::MinosError;
use chrono::{NaiveDateTime, Utc};

pub const DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S";

pub fn datetime_now() -> NaiveDateTime {
    Utc::now().naive_local()
}

pub fn formatted_datetime_now() -> Result<NaiveDateTime, MinosError> {
    Ok(string_as_datetime(
        &Utc::now().naive_local().format(DATETIME_FMT).to_string(),
    )?)
}

pub fn string_as_datetime(date: &str) -> Result<NaiveDateTime, MinosError> {
    Ok(NaiveDateTime::parse_from_str(&date, DATETIME_FMT)?)
}
