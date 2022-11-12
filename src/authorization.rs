use crate::actor::Actor;
use crate::errors::{ErrorKind, MinosError};
use crate::NonEmptyString;
use chrono::Utc;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
/// Agents permissions, defines what a user is allowed to do.
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

impl ToString for Permission {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
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
    ///     use minos::errors::{ErrorKind, MinosError};
    ///     use minos::authorization::Permission;
    ///
    ///     fn check_permission(permission: Permission) -> Result<(), MinosError> {
    ///         if permission != Permission::Update {
    ///             return Err(MinosError::new(
    ///                     ErrorKind::Authorization,
    ///                     &Permission::Update.required_msg(),
    ///             ));
    ///         }
    ///
    ///         Ok(())
    ///     }
    /// ```
    ///```
    ///     use minos::authorization::Permission;
    ///     assert_eq!(Permission::Update.required_msg(), "Update permission is required.");
    /// ```
    pub fn required_msg(&self) -> String {
        format!("{:?} permission is required.", self)
    }

    /// Returns a vector with Create, Read, Update, and Delete permissions
    /// # Example
    /// ```
    ///     use minos::authorization::Permission;
    ///     use minos::authorization::Permission::{Create, Read, Update, Delete};
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

#[derive(Debug, PartialEq, Clone, PartialOrd)]
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
            return Err(MinosError::new(
                ErrorKind::Authorization,
                "Authorization created for another resource",
            ));
        }

        if self.expiration <= Utc::now().timestamp() as u64 {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                "The Authorization is expired",
            ));
        }

        if actor.id() != self.agent_id {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                &format!("This Authorization is not for the user {}", actor.id()),
            ));
        }

        Ok(())
    }

    pub fn search_permission(&self, permission: Permission) -> Result<(), MinosError> {
        if !&self.permissions.contains(&permission) {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                &permission.required_msg(),
            ));
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
                return Err(MinosError::new(
                    ErrorKind::Authorization,
                    &permission.required_msg(),
                ));
            }
        }

        Ok(())
    }
}

/// Defines the access and modification rules for a resource. It has two types of
/// authorization policies: by owner and by roles; the use of the first excludes
/// the other and vice versa.
///
/// Care must be taken to use the authorization policies correctly, because when building the
/// Authorization with the AuthorizationBuilder, it will return an error.
///
#[derive(PartialEq, Debug, Clone, PartialOrd, Default)]
pub struct Policy {
    /// authorization duration, in seconds
    pub(crate) duration: u64,

    /// Use only for objects with real owner. If you want set only Permission::Create,
    /// use other authorization policy.
    pub(crate) by_owner: bool,

    /// Restricts the authorization to only agents in specific groups
    pub(crate) groups_ids: Option<Vec<NonEmptyString>>,

    /// permissions granted
    pub(crate) permissions: Vec<Permission>,
}

impl Policy {
    pub fn new(
        duration: u64,
        by_owner: bool,
        groups_ids: Option<Vec<NonEmptyString>>,
        permissions: Vec<Permission>,
    ) -> Self {
        Self {
            duration,
            by_owner,
            groups_ids,
            permissions,
        }
    }
    pub fn duration(&self) -> u64 {
        self.duration
    }
    pub fn by_owner(&self) -> bool {
        self.by_owner
    }
    pub fn groups_ids(&self) -> &Option<Vec<NonEmptyString>> {
        &self.groups_ids
    }
    pub fn permissions(&self) -> &Vec<Permission> {
        &self.permissions
    }
}
