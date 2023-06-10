use parse_display::Display;
use thiserror::Error as ThisError;

use crate::minos::Operator;


#[non_exhaustive]
#[derive(Debug, ThisError)]
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

    #[error("invalid comparation, found '{0}'")]
    InvalidOperation(Operator),
}