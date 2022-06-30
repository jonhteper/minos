use crate::errors::{ErrorKind, MinosError};
use crate::user::UserAttributes;
use crate::utils::datetime_now;
use chrono::{Duration, NaiveDateTime};

use crate::group::GroupId;
use crate::Status;

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
/// Users permissions, defines what a user is allowed to do.
pub enum Permission {
    /// The user can create an object
    Create,
    /// The user can read the source
    Read,
    /// The user can edit the source, but can't delete the source
    Update,
    /// The user can delete the source
    Delete,
}

impl From<u8> for Permission {
    fn from(n: u8) -> Self {
        match n {
            3 => Permission::Delete,
            2 => Permission::Update,
            1 => Permission::Create,
            _ => Permission::Read,
        }
    }
}

impl ToString for Permission {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

impl From<&str> for Permission {
    fn from(str: &str) -> Self {
        if str == Permission::Create.to_string() {
            Permission::Create
        } else if str == Permission::Update.to_string() {
            Permission::Update
        } else if str == Permission::Delete.to_string() {
            Permission::Delete
        } else {
            Permission::Read
        }
    }
}

impl Permission {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

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

/// Users authorizations
#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Authorization {
    pub(crate) permissions: Vec<Permission>,
    pub(crate) user_id: String,
    pub(crate) resource_id: String,
    pub(crate) resource_type: ResourceType,
    pub(crate) expiration: NaiveDateTime,
}

impl Authorization {
    // TODO: check if constructor is necessary

    pub fn permissions(&self) -> &Vec<Permission> {
        &self.permissions
    }
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }
    pub fn resource_type(&self) -> &ResourceType {
        &self.resource_type
    }
    pub fn expiration(&self) -> NaiveDateTime {
        self.expiration
    }

    pub fn check(
        &self,
        resource_id: &str,
        user: &UserAttributes,
        required_permission: &Permission,
    ) -> Result<(), MinosError> {
        if &self.resource_id != resource_id {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                "Authorization created for another resource",
            ));
        }

        if self.expiration <= datetime_now() {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                "The Authorization is expired",
            ));
        }

        if &user.id != &self.user_id {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                &format!("This Authorization is not for the user {}", &user.id),
            ));
        }

        if !&self.permissions.contains(&required_permission) {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                &required_permission.required_msg(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Owner {
    User(String),
    Group(String),
}

pub trait Resource {
    fn id(&self) -> &str;
    fn resource_type(&self) -> ResourceType;
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub struct ResourceType {
    pub(crate) label: String,
    pub(crate) owner: Option<Owner>,
    pub(crate) policies: Vec<Policy>,
}

/// Defines the access and modification rules for a resource. It has two types of
/// authorization policies: by owner and by roles; the use of the first excludes
/// the other and vice versa.
///
/// Care must be taken to use the authorization policies correctly, because when building the
/// Authorization with the AuthorizationBuilder, it will return an error.
///
#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub struct Policy {
    /// authorization duration, in seconds (max 65535 ~ 1092 min ~ 18 hours)
    pub(crate) duration: u16,

    /// Use only for objects with real owner. If you want set only Permission::Create,
    /// use other authorization policy.
    pub(crate) by_owner: bool,

    /// Restricts the authorization to only users with specific roles
    pub(crate) groups_ids: Option<Vec<GroupId>>,

    /// permissions granted
    pub(crate) permissions: Vec<Permission>,
}

impl Policy {
    pub fn new(
        duration: u16,
        by_owner: bool,
        groups_ids: Option<Vec<GroupId>>,
        permissions: Vec<Permission>,
    ) -> Self {
        Self {
            duration,
            by_owner,
            groups_ids,
            permissions,
        }
    }
    pub fn duration(&self) -> u16 {
        self.duration
    }
    pub fn by_owner(&self) -> bool {
        self.by_owner
    }
    pub fn groups_ids(&self) -> &Option<Vec<GroupId>> {
        &self.groups_ids
    }
    pub fn permissions(&self) -> &Vec<Permission> {
        &self.permissions
    }
}

pub struct AuthorizationBuilder<'b> {
    policy: &'b Policy,
}

impl<'b> AuthorizationBuilder<'b> {
    pub fn new(policy: &'b Policy) -> Self {
        Self { policy }
    }

    fn check_groups(&self, user: &UserAttributes) -> Result<(), MinosError> {
        if let Some(ids) = &self.policy.groups_ids {
            let groups = &user.groups;
            let groups_ids: Vec<&str> = groups.into_iter().map(|g| g.as_str()).collect();

            for id in ids {
                if !groups_ids.contains(&id.as_str()) {
                    return Err(MinosError::new(
                        ErrorKind::Authorization,
                        "The user is not in the correct group",
                    ));
                }
            }
        }

        Ok(())
    }

    fn same_group_check(&self, group_id: GroupId, user: &UserAttributes) -> Result<(), MinosError> {
        if !&user.groups.contains(&group_id) {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                "The user is not in the owning group",
            ));
        }

        Ok(())
    }

    fn same_user_check(&self, user_id: &str, user: &UserAttributes) -> Result<(), MinosError> {
        if user_id != &user.id {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                "The user is not the owner",
            ));
        }
        Ok(())
    }

    /// Create a Authorization based in Policy and User
    pub fn build(
        &self,
        resource_id: &str,
        resource_type: &ResourceType,
        user: &UserAttributes,
    ) -> Result<Authorization, MinosError> {
        if user.status != Status::Active {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                "The user is not active",
            ));
        }

        if self.policy.by_owner {
            match &resource_type.owner {
                None => {
                    return Err(MinosError::new(
                        ErrorKind::IncompatibleAuthPolicy,
                        "The resource haven't an owner",
                    ));
                }
                Some(owner) => match owner {
                    Owner::User(id) => self.same_user_check(id, &user)?,
                    Owner::Group(id) => self.same_group_check(GroupId::from(id.as_str()), &user)?,
                },
            }
        } else {
            let _ = self.check_groups(&user)?;
        }

        Ok(Authorization {
            permissions: self.policy.permissions.clone(),
            user_id: user.id.clone(),
            resource_id: resource_id.to_string(),
            resource_type: resource_type.clone(),
            expiration: datetime_now() + Duration::seconds(60 * 5),
        })
    }
}
