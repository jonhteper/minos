use chrono::format::ParseError;
use heimdall_errors::{implement_error, implement_error_with_kind};
use std::fmt::{Display, Formatter, Result};
use std::io;

use crate::errors::ErrorKind::{EmptyString, Other};
#[cfg(feature = "jwt")]
use jsonwebtoken;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    Io(io::ErrorKind),
    Chrono,

    #[cfg(feature = "jwt")]
    JWT(jsonwebtoken::errors::ErrorKind),

    #[cfg(feature = "toml_storage")]
    BadExtension,

    #[cfg(feature = "toml_storage")]
    Toml,

    /// Authorization rules collision
    IncompatibleAuthPolicy,
    EmptyString,
    InvalidPermission,
    /// It is recommended to use this error when using a custom-made implementation of [`Resource::authorize`].
    ///
    /// [`Resource::authorize`]: crate::resources::Resource::authorize
    Authorization,
    ParsePolicyMode,

    #[cfg(feature = "manifest")]
    MissingResourceType,
    Other,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MinosError {
    kind: ErrorKind,
    message: String,
}

impl MinosError {
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        MinosError {
            kind,
            message: message.to_string(),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind.clone()
    }
}

impl Display for MinosError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "kind: {} message: {}", self.kind, self.message)
    }
}

impl From<&MinosError> for MinosError {
    fn from(error: &MinosError) -> Self {
        MinosError {
            kind: error.kind.clone(),
            message: error.to_string(),
        }
    }
}

implement_error_with_kind!(MinosError, std::io::Error, ErrorKind::Io);
implement_error!(MinosError, ParseError, ErrorKind::Chrono);

impl From<String> for MinosError {
    fn from(error: String) -> Self {
        if error.is_empty() {
            return MinosError {
                kind: EmptyString,
                message: error,
            };
        }

        MinosError {
            kind: Other,
            message: error,
        }
    }
}

impl From<&str> for MinosError {
    fn from(error: &str) -> Self {
        if error.is_empty() {
            return MinosError {
                kind: EmptyString,
                message: error.to_string(),
            };
        }

        MinosError {
            kind: Other,
            message: error.to_string(),
        }
    }
}

#[cfg(feature = "jwt")]
mod jwt_feature {
    use super::{ErrorKind, MinosError};
    use heimdall_errors::implement_error_with_kind;
    use jsonwebtoken;

    implement_error_with_kind!(MinosError, jsonwebtoken::errors::Error, ErrorKind::JWT);
}

#[cfg(feature = "toml_storage")]
mod toml_feature {
    use super::{ErrorKind, MinosError};
    use heimdall_errors::implement_error;
    use toml;
    use toml::de::Error;

    implement_error!(MinosError, Error, ErrorKind::Toml);
    implement_error!(MinosError, toml::ser::Error, ErrorKind::Toml);
}
