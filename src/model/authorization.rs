/*use crate::model::actor::Actor;
use crate::errors::MinosError;
use crate::prelude::{ActorId, Group, ResourceId, ResourceType};
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
pub struct Permission(String);

impl Display for Permission {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<&str> for Permission {
    fn from(str: &str) -> Self {
        Self(str.to_owned())
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
    ///     assert_eq!(Permission("update".to_string()).required_msg(), "update permission is required.");
    ///     assert_eq!(Permission("purge".to_string()).required_msg(), "purgue permission is required");
    /// ```
    pub fn required_msg(&self) -> String {
        format!("{self} permission is required.")
    }


}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd)]
pub struct Authorization {
    pub(crate) permissions: Vec<Permission>,
    pub(crate) actor_id: ActorId,
    pub(crate) resource_id: ResourceId,
    pub(crate) resource_type: ResourceType,
    pub(crate) expiration: u64,
}

impl Authorization {
    pub fn permissions(&self) -> Vec<Permission> {
        self.permissions.clone()
    }

    pub fn actor_id(&self) -> &ActorId {
        &self.actor_id
    }

    pub fn resource_id(&self) -> &ResourceId {
        &self.resource_id
    }

    pub fn resource_type(&self) -> &ResourceType {
        &self.resource_type
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

        if actor.id() != self.actor_id {
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

pub type PolicyId = NonEmptyString;

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
    pub(crate) id: Option<PolicyId>,

    pub(crate) resource_type: Option<ResourceType>,

    /// authorization duration, in seconds
    pub(crate) duration: NonZeroU64,

    /// defines the algorithm used in authorization process
    pub(crate) auth_mode: AuthorizationMode,

    /// listed groups
    pub(crate) groups: Option<Vec<Group>>,

    /// permissions granted
    pub(crate) permissions: Vec<Permission>,
}

impl Policy {
    pub fn new(
        id: Option<PolicyId>,
        resource_type: Option<ResourceType>,
        duration: NonZeroU64,
        auth_mode: AuthorizationMode,
        groups: Option<Vec<Group>>,
        permissions: Vec<Permission>,
    ) -> Self {
        Self {
            id,
            resource_type,
            duration,
            auth_mode,
            groups,
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

    pub fn groups(&self) -> Option<&[Group]> {
        self.groups.as_deref()
    }

    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }
}
*/
