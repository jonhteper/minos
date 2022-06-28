use crate::errors::{ErrorKind, MinosError};
use chrono::{NaiveDateTime, Utc};
use std::path::PathBuf;

pub const DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S";

pub fn datetime_now() -> NaiveDateTime {
    Utc::now().naive_local()
}

pub fn string_datetime_now() -> String {
    datetime_now().format(DATETIME_FMT).to_string()
}

pub fn string_as_datetime(date: String) -> Result<NaiveDateTime, MinosError> {
    let datetime = NaiveDateTime::parse_from_str(&date, DATETIME_FMT)?;
    Ok(datetime)
}
