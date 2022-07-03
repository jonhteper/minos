use chrono::format::ParseError;
use heimdall_errors::implement_error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ErrorKind {
    Io,
    Chrono,
    JWT,
    Authorization,

    /// Auth rules collision
    IncompatibleAuthPolicy,
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
        self.kind
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
            kind: error.kind,
            message: error.to_string(),
        }
    }
}

implement_error!(MinosError, std::io::Error, ErrorKind::Io);
implement_error!(MinosError, ParseError, ErrorKind::Chrono);

#[cfg(feature = "jwt")]
mod jwt_feature {
    use heimdall_errors::implement_error;
    use jsonwebtoken;
    use super::{MinosError, ErrorKind};

    implement_error!(MinosError, jsonwebtoken::errors::Error, ErrorKind::JWT);
}
