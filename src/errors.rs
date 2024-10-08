use std::{io, sync::Arc};

use crate::parser::{v0_16, v0_16_m};
use parse_display::ParseError;
use thiserror::Error as ThisError;

pub type MinosResult<T> = Result<T, Error>;

#[non_exhaustive]
#[derive(Debug, Clone, ThisError, PartialEq, Eq)]
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
    InvalidToken { expected: &'static str, found: String },

    #[error("expected Token, found nothing")]
    MissingToken,

    #[error("sintaxis not supported")]
    SyntaxNotSupported,

    #[error("permission '{0}' not found")]
    PermissionNotFound(String),

    #[error("macro '{0}' not found")]
    MacroNotExist(String),

    // 3-party errors
    #[error("io err: {0}")]
    Io(String),

    #[error(transparent)]
    RuleV0_16(Box<pest::error::Error<v0_16::Rule>>),

    #[error(transparent)]
    RuleV0_16M(Box<pest::error::Error<v0_16_m::Rule>>),

    #[error(transparent)]
    ParseError(Arc<ParseError>),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<pest::error::Error<v0_16::Rule>> for Error {
    fn from(err: pest::error::Error<v0_16::Rule>) -> Self {
        Self::RuleV0_16(Box::new(err))
    }
}

impl From<pest::error::Error<v0_16_m::Rule>> for Error {
    fn from(err: pest::error::Error<v0_16_m::Rule>) -> Self {
        Self::RuleV0_16M(Box::new(err))
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::ParseError(Arc::new(err))
    }
}
