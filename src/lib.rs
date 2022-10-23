//! Authorization library
//!
use crate::errors::{ErrorKind, MinosError};
use std::fmt::{Display, Formatter};

pub mod agent;
pub mod authorization;
pub mod authorization_builder;
pub mod errors;
pub mod prelude;
pub mod resources;
pub mod utils;

#[cfg(feature = "jwt")]
pub mod jwt;

#[cfg(feature = "toml_storage")]
pub mod toml;

#[cfg(test)]
mod test;

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct NonEmptyString(String);

impl TryFrom<&str> for NonEmptyString {
    type Error = MinosError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        if str.trim().is_empty() {
            return Err(MinosError::new(
                ErrorKind::EmptyId,
                "The identifier can't be an empty string",
            ));
        }

        Ok(Self(str.to_string()))
    }
}

impl Display for NonEmptyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NonEmptyString {
    pub fn from_str(str: &str) -> Option<Self> {
        match str.trim().is_empty() {
            true => None,
            false => Some(Self(str.to_string())),
        }
    }
}
