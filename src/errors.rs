use std::io;

use parse_display::ParseError;
use thiserror::Error as ThisError;

use crate::language::policy::Permission;
use crate::parser::v0_14;
use crate::parser::v0_15;

pub type MinosResult<T> = Result<T, Error>;

#[non_exhaustive]
#[derive(Debug, ThisError, PartialEq, Eq)]
pub enum Error {
    #[error("environment '{0}' not found")]
    EnvironmentNotFound(String),

    #[error("resource '{0}' not found")]
    ResourceNotFound(String),

    #[error("the actor '{0}' is not authorized")]
    ActorNotAuthorized(String),

    #[error("unwrap invalid value, expects Str found List")]
    UnwrapInvalidStringValue,

    #[error("unwrap invalid value, expects List found Str")]
    UnwrapInvalidListValue,

    #[error("invalid token found: {found}, expected: {expected}")]
    InvalidToken { expected: String, found: String },

    #[error("sintaxis not supported")]
    SintaxisNotSupported,

    #[error("permission '{0}' not found")]
    PermissionNotFound(Permission),

    // 3-party errors
    #[error("io err: {0}")]
    Io(String),

    #[error(transparent)]
    RuleV0_14(Box<pest::error::Error<v0_14::Rule>>),

    #[error(transparent)]
    RuleV0_15(Box<pest::error::Error<v0_15::Rule>>),

    #[error(transparent)]
    ParseError(#[from] ParseError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<pest::error::Error<v0_14::Rule>> for Error {
    fn from(err: pest::error::Error<v0_14::Rule>) -> Self {
        Self::RuleV0_14(Box::new(err))
    }
}

impl From<pest::error::Error<v0_15::Rule>> for Error {
    fn from(err: pest::error::Error<v0_15::Rule>) -> Self {
        Self::RuleV0_15(Box::new(err))
    }
}
