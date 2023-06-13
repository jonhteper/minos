use std::io;

use parse_display::{Display, ParseError};
use thiserror::Error as ThisError;

use crate::minos::{lang::Token, parser::v0_14::Rule};

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

    // 3-party errors
    #[error("io err: {0}")]
    Io(String),

    #[error(transparent)]
    Pest(Box<pest::error::Error<Rule>>),

    #[error(transparent)]
    ParseError(#[from] ParseError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(err: pest::error::Error<Rule>) -> Self {
        Self::Pest(Box::new(err))
    }
}