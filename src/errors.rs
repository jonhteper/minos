use crate::model::permission::Permission;
use chrono::format::ParseError;
#[cfg(feature = "jwt")]
use jsonwebtoken;
use std::fmt::{Display, Formatter, Result};
use std::io;
use std::io::Error;
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub struct IoErrorRep {
    kind: io::ErrorKind,
}

impl Display for IoErrorRep {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.kind, f)
    }
}

impl From<io::Error> for IoErrorRep {
    fn from(error: Error) -> Self {
        Self { kind: error.kind() }
    }
}

/// High level list of common errors, use for easy and non
/// exhaustive errors match.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// The [Actor] don't have authorization.
    Unauthorized,

    /// The [Resource]'s [Policy] is not format correctly.
    PolicyFormatError,

    /// The [Manifest] is bad configured.
    ///
    /// [Manifest]: crate::resource_manifest::ResourceManifest
    #[cfg(feature = "manifest")]
    ManifestFormatError,

    /// No Minos error, may be an std library error or
    /// 3rd party library error.
    UnknownError,
}

#[non_exhaustive]
#[derive(Clone, Debug, Error, PartialEq)]
pub enum MinosError {
    // Unauthorized errors
    /// Indicates that the [Actor] does not have any [Permission]
    /// to manipulate the resource.
    ///
    /// [Actor]: crate::model::actor::Actor
    #[error("no permissions available")]
    MissingPermissions,

    /// Indicates that the [Actor] does not have specific [Permission]
    /// to manipulate the resource.
    ///
    /// [Actor]: crate::model::actor::Actor
    #[error("{}", .0.required_msg())]
    MissingPermission(Permission),

    /// Indicates that the [Authorization] is out of date.
    ///
    /// [Authorization]:crate::model::authorization::Authorization
    #[error("expired authorization")]
    ExpiredAuthorization,

    #[error("authorization created for another actor")]
    InvalidActor,

    #[error("authorization created for another resource")]
    InvalidResource,

    #[error("the actor is not the owner")]
    InvalidOwner,

    #[error("the actor is not in all required groups")]
    MissingGroup,

    // Policy format errors
    #[error("the rule don't have any permission")]
    EmptyPermissions,

    #[error("the permissions don't have the correct format: {}", .0)]
    PermissionsFormat(String),

    #[error("the name of permission is not a string: {}", .0)]
    PermissionNameFormat(String),

    #[error("the duration of permission is not a valid time: {}", .0)]
    PermissionDurationFormat(String),

    #[error("invalid char in assertion: {}", .0)]
    InvalidAssertionChar(String),

    #[error("invalid assertion syntax: {}", .0)]
    InvalidAssertionSyntax(String),

    #[error("invalid key syntax: {}", .0)]
    InvalidKeySyntax(String),

    #[error("the resource haven't an owner")]
    ResourceWithoutOwner,

    #[error("the policy haven't groups defined")]
    EmptyGroupsPolicy,

    /// Indicate that the attribute Policy::resource_type not match
    /// with the return value of function Resource::resource_type
    #[error("the policy not corresponds to resource type")]
    InvalidResourceTypePolicy,

    #[error("the policy mode is invalid")]
    InvalidPolicyMode,

    #[error("duration can't be equals to zero")]
    ZeroValueDuration,

    // Manifest format Errors
    #[cfg(feature = "manifest")]
    #[error("the resource requires an explicit resource type for use in the manifest")]
    MissingResourceType,

    #[cfg(feature = "toml_storage")]
    #[error("the file not have a correct extension")]
    BadExtension,

    #[cfg(feature = "toml_storage")]
    #[error("the file not have an extension")]
    NoExtension,

    // 3rd party errors
    #[error("input error: empty string")]
    EmptyString,

    #[error(transparent)]
    Io(IoErrorRep),

    #[error(transparent)]
    ChronoParse(#[from] ParseError),

    #[error(transparent)]
    Regex(#[from] regex::Error),

    #[cfg(feature = "jwt")]
    #[error(transparent)]
    JWT(#[from] jsonwebtoken::errors::Error),

    #[cfg(feature = "toml_storage")]
    #[error(transparent)]
    TomlSerialize(#[from] toml::ser::Error),

    #[cfg(feature = "toml_storage")]
    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),

    /// TODO: Remove in production
    #[error("unimplemented code")]
    __UnImplemented,
}

impl From<io::Error> for MinosError {
    fn from(error: Error) -> Self {
        Self::Io(IoErrorRep::from(error))
    }
}

impl MinosError {
    pub fn kind(&self) -> ErrorKind {
        match self {
            MinosError::MissingPermissions => ErrorKind::Unauthorized,
            MinosError::MissingPermission(_) => ErrorKind::Unauthorized,
            MinosError::ExpiredAuthorization => ErrorKind::Unauthorized,
            MinosError::InvalidActor => ErrorKind::Unauthorized,
            MinosError::InvalidResource => ErrorKind::Unauthorized,
            MinosError::InvalidOwner => ErrorKind::Unauthorized,
            MinosError::MissingGroup => ErrorKind::Unauthorized,

            MinosError::ResourceWithoutOwner => ErrorKind::PolicyFormatError,
            MinosError::EmptyGroupsPolicy => ErrorKind::PolicyFormatError,
            MinosError::InvalidResourceTypePolicy => ErrorKind::PolicyFormatError,
            MinosError::InvalidPolicyMode => ErrorKind::PolicyFormatError,
            MinosError::ZeroValueDuration => ErrorKind::PolicyFormatError,

            #[cfg(feature = "manifest")]
            MinosError::MissingResourceType => ErrorKind::ManifestFormatError,
            #[cfg(feature = "toml_storage")]
            MinosError::BadExtension => ErrorKind::ManifestFormatError,
            #[cfg(feature = "toml_storage")]
            MinosError::NoExtension => ErrorKind::ManifestFormatError,

            _ => ErrorKind::UnknownError,
        }
    }
}
