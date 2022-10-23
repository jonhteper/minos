use chrono::format::ParseError;
use heimdall_errors::{implement_error, implement_error_with_kind};
use std::fmt::{Display, Formatter, Result};
use std::io;

#[cfg(feature = "jwt")]
use jsonwebtoken;

#[derive(Debug, PartialEq, Clone)]
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
    EmptyId,
    InvalidPermission,
    /// It is recommended to use this error when using a custom-made implementation of [`Resource::authorize`].
    ///
    /// [`Resource::authorize`]: crate::resources::Resource::authorize
    Authorization,
}

impl ErrorKind {
    pub fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[derive(Debug, PartialEq, Clone)]
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
        write!(
            f,
            "kind: {} message: {}",
            self.kind.to_string(),
            self.message
        )
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
