use crate::core::actor::Actor;
use crate::errors::MinosError;
use chrono::Utc;
use non_empty_string::NonEmptyString;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU64;

const OWNER_POLICY_MODE_STR: &str = "owner";
const SINGLE_GROUP_MODE_STR: &str = "single group";
const MULTI_GROUP_MODE_STR: &str = "multi group";
const OWNER_SINGLE_GROUP_MODE_STR: &str = "owner and single group";
const OWNER_MULTI_GROUP_MODE_STR: &str = "owner and multi group";

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd)]
/// Defines what an actor is allowed to do.
pub enum Permission {
    /// The actor can create the source
    Create,
    /// The actor can read the source
    Read,
    /// The actor can edit the source, but can't delete the source
    Update,
    /// The actor can delete the source
    Delete,

    /// The actor can perform a specific action
    Custom(String),
}

impl Display for Permission {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(permission) => Display::fmt(&permission, f),
            _ => write!(f, "{self:?}"),
        }
    }
}

impl From<&str> for Permission {
    fn from(str: &str) -> Self {
        match str {
            "create" => Self::Create,
            "read" => Self::Read,
            "update" => Self::Update,
            "delete" => Self::Delete,
            _ => Self::Custom(str.to_string()),
        }
    }
}

impl Permission {
    /// Return simple explanation for permission required
    ///
    ///# Examples
    ///```
    ///     use minos::errors::MinosError;
    ///     use minos::prelude::Permission;
    ///
    ///     fn check_permission(permission: Permission) -> Result<(), MinosError> {
    ///         if permission != Permission::Update {
    ///             return Err(MinosError::MissingPermission(Permission::Update));
    ///         }
    ///
    ///         Ok(())
    ///     }
    /// ```
    ///```
    ///     use minos::prelude::Permission;
    ///     assert_eq!(Permission::Update.required_msg(), "update permission is required.");
    ///     assert_eq!(Permission::Custom("Purge".to_string()).required_msg(), "purgue permission is required");
    /// ```
    pub fn required_msg(&self) -> String {
        format!("{self} permission is required.")
    }

    /// Returns a vector with Create, Read, Update, and Delete permissions
    /// # Example
    /// ```
    ///     use minos::prelude::Permission;
    ///     use minos::prelude::Permission::{Create, Read, Update, Delete};
    ///
    ///     assert_eq!(vec![Create, Read, Update, Delete], Permission::crud())
    /// ```
    pub fn crud() -> Vec<Permission> {
        vec![
            Permission::Create,
            Permission::Read,
            Permission::Update,
            Permission::Delete,
        ]
    }

    /// Like crud, but within Create
    pub fn rud() -> Vec<Permission> {
        vec![Permission::Read, Permission::Update, Permission::Delete]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd)]
pub struct Authorization {
    pub(crate) permissions: Vec<Permission>,
    pub(crate) agent_id: NonEmptyString,
    pub(crate) resource_id: NonEmptyString,
    pub(crate) resource_type: Option<NonEmptyString>,
    pub(crate) expiration: u64,
}

impl Authorization {
    pub fn permissions(&self) -> Vec<Permission> {
        self.permissions.clone()
    }

    pub fn agent_id(&self) -> String {
        self.agent_id.to_string()
    }

    pub fn resource_id(&self) -> String {
        self.resource_id.to_string()
    }

    pub fn resource_type(&self) -> Option<NonEmptyString> {
        self.resource_type.clone()
    }
    pub fn expiration(&self) -> u64 {
        self.expiration
    }

    fn basic_check<A: Actor>(&self, resource_id: &str, actor: &A) -> Result<(), MinosError> {
        if self.resource_id.to_string() != resource_id {
            return Err(MinosError::InvalidResource);
        }

        if self.expiration <= Utc::now().timestamp() as u64 {
            return Err(MinosError::ExpiredAuthorization);
        }

        if actor.id() != self.agent_id {
            return Err(MinosError::InvalidActor);
        }

        Ok(())
    }

    pub fn search_permission(&self, permission: Permission) -> Result<(), MinosError> {
        if !&self.permissions.contains(&permission) {
            return Err(MinosError::MissingPermission(permission));
        }

        Ok(())
    }

    pub fn check<A: Actor>(
        &self,
        resource_id: &str,
        actor: &A,
        required_permission: Permission,
    ) -> Result<(), MinosError> {
        self.basic_check(resource_id, actor)?;
        self.search_permission(required_permission)
    }

    pub fn multi_permissions_check<A: Actor>(
        &self,
        resource_id: &str,
        actor: &A,
        required_permissions: &Vec<Permission>,
    ) -> Result<(), MinosError> {
        self.basic_check(resource_id, actor)?;

        for permission in required_permissions {
            if !&self.permissions.contains(permission) {
                return Err(MinosError::MissingPermission(permission.clone()));
            }
        }

        Ok(())
    }
}

/// Defines the algorithm used in authorization process
#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Copy)]
pub enum AuthorizationMode {
    /// The authorization is granted only if the [`Actor`] is
    /// the owner of the [`Resource`]
    ///
    /// [`Resource`]: crate::resources::Resource
    Owner,

    /// The authorization is granted only if the [`Actor`] belongs
    /// to one of the listed groups
    SingleGroup,

    /// The authorization is granted only if the [`Actor`] belongs
    /// to all of the listed groups
    MultiGroup,

    /// The authorization is granted only if the [`Actor`] is
    /// the owner of the [`Resource`] and belongs to one of the
    /// listed groups
    ///
    /// [`Resource`]: crate::resources::Resource
    OwnerSingleGroup,

    /// The authorization is granted only if the [`Actor`] is
    /// the owner of the [`Resource`] and belongs to all of the
    /// listed groups.
    ///
    /// [`Resource`]: crate::resources::Resource
    OwnerMultiGroup,
}

impl ToString for AuthorizationMode {
    fn to_string(&self) -> String {
        match self {
            Self::Owner => OWNER_POLICY_MODE_STR.to_string(),
            Self::SingleGroup => SINGLE_GROUP_MODE_STR.to_string(),
            Self::MultiGroup => MULTI_GROUP_MODE_STR.to_string(),
            Self::OwnerSingleGroup => OWNER_SINGLE_GROUP_MODE_STR.to_string(),
            Self::OwnerMultiGroup => OWNER_MULTI_GROUP_MODE_STR.to_string(),
        }
    }
}

impl TryFrom<&str> for AuthorizationMode {
    type Error = MinosError;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            OWNER_POLICY_MODE_STR => Ok(Self::Owner),
            SINGLE_GROUP_MODE_STR => Ok(Self::SingleGroup),
            MULTI_GROUP_MODE_STR => Ok(Self::MultiGroup),
            OWNER_SINGLE_GROUP_MODE_STR => Ok(Self::OwnerSingleGroup),
            OWNER_MULTI_GROUP_MODE_STR => Ok(Self::OwnerMultiGroup),
            _ => Err(MinosError::InvalidPolicyMode),
        }
    }
}

/// Defines the access and modification rules for a resource. It has two types of
/// authorization policies: by owner and by roles; the use of the first excludes
/// the other and vice versa.
///
/// Care must be taken to use the authorization policies correctly, because when building the
/// Authorization with the AuthorizationBuilder, it will return an error.
///
#[derive(PartialEq, Eq, Debug, Clone, PartialOrd)]
pub struct Policy {
    /// Unique identifier, to prevent collisions.
    pub(crate) id: Option<NonEmptyString>,

    pub(crate) resource_type: Option<NonEmptyString>,

    /// authorization duration, in seconds
    pub(crate) duration: NonZeroU64,

    /// defines the algorithm used in authorization process
    pub(crate) auth_mode: AuthorizationMode,

    /// listed groups
    pub(crate) groups_ids: Option<Vec<NonEmptyString>>,

    /// permissions granted
    pub(crate) permissions: Vec<Permission>,
}

impl Policy {
    pub fn new(
        id: Option<NonEmptyString>,
        resource_type: Option<NonEmptyString>,
        duration: NonZeroU64,
        auth_mode: AuthorizationMode,
        groups_ids: Option<Vec<NonEmptyString>>,
        permissions: Vec<Permission>,
    ) -> Self {
        Self {
            id,
            resource_type,
            duration,
            auth_mode,
            groups_ids,
            permissions,
        }
    }

    pub fn id(&self) -> Option<&NonEmptyString> {
        self.id.as_ref()
    }

    pub fn duration(&self) -> NonZeroU64 {
        self.duration
    }

    pub fn mode(&self) -> AuthorizationMode {
        self.auth_mode
    }

    pub fn groups_ids(&self) -> Option<&[NonEmptyString]> {
        self.groups_ids.as_deref()
    }

    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }
}
