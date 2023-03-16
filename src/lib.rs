//! Authorization library
//!

#![allow(unused)] // TODO: remove in release

pub mod errors;
pub mod model;
pub mod prelude;

#[cfg(test)]
mod test;

/// [NonEmptyString] constructor, returns `Result<NonEmptyString, MinosError>`.
/// # Examples
/// ```
/// use non_empty_string::NonEmptyString;
/// use minos::errors::MinosError;
/// use minos::non_empty_string;
///
/// fn hello() -> Result<NonEmptyString, MinosError> {
///     non_empty_string!("hello")
/// }
/// ```
///
/// [NonEmptyString]: non_empty_string::NonEmptyString
#[macro_export]
macro_rules! non_empty_string {
    ($str: expr) => {
        non_empty_string::NonEmptyString::try_from($str).map_err(|_| MinosError::EmptyString)
    };
}
