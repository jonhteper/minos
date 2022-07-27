use crate::errors::{ErrorKind, MinosError};
use crate::group::GroupId;
use crate::resources::{Owner, ResourceType};
use crate::user::UserAttributes;
use crate::utils::datetime_now;
use crate::Status;
use chrono::{Duration, NaiveDateTime};

#[derive(Debug, PartialEq, Clone, PartialOrd)]
/// Users permissions, defines what a user is allowed to do.
pub enum Permission {
    /// The user can create the source
    Create,
    /// The user can read the source
    Read,
    /// The user can edit the source, but can't delete the source
    Update,
    /// The user can delete the source
    Delete,

    /// The user can perform a specific action
    #[cfg(feature = "custom_permission")]
    Custom(String),
}

#[cfg(not(feature = "custom_permission"))]
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

#[cfg(not(feature = "custom_permission"))]
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

#[cfg(feature = "custom_permission")]
impl From<&str> for Permission {
    fn from(str: &str) -> Self {
        if str == Permission::Create.to_string() {
            Permission::Create
        } else if str == Permission::Update.to_string() {
            Permission::Update
        } else if str == Permission::Delete.to_string() {
            Permission::Delete
        } else if str == Permission::Read.to_string() {
            Permission::Read
        } else {
            Permission::Custom(str.to_string())
        }
    }
}

impl Permission {
    #[cfg(not(feature = "custom_permission"))]
    pub fn as_u8(&self) -> u8 {
        self.clone() as u8
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
    pub(crate) resource_type: String,
    pub(crate) expiration: NaiveDateTime,
}

impl Authorization {
    pub fn permissions(&self) -> &Vec<Permission> {
        &self.permissions
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }

    pub fn resource_type(&self) -> &str {
        &self.resource_type
    }
    pub fn expiration(&self) -> NaiveDateTime {
        self.expiration
    }

    fn basic_check(&self, resource_id: &str, user: &UserAttributes) -> Result<(), MinosError> {
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

        Ok(())
    }

    pub fn check(
        &self,
        resource_id: &str,
        user: &UserAttributes,
        required_permission: &Permission,
    ) -> Result<(), MinosError> {
        let _ = self.basic_check(resource_id, user)?;

        if !&self.permissions.contains(&required_permission) {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                &required_permission.required_msg(),
            ));
        }

        Ok(())
    }

    pub fn multi_permissions_check(
        &self,
        resource_id: &str,
        user: &UserAttributes,
        required_permissions: &Vec<Permission>,
    ) -> Result<(), MinosError> {
        let _ = self.basic_check(resource_id, user)?;

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
    /// authorization duration, in seconds (recommended max duration: 65535 ~ 1092 min ~ 18 hours)
    pub(crate) duration: u16,

    /// Use only for objects with real owner. If you want set only Permission::Create,
    /// use other authorization policy.
    pub(crate) by_owner: bool,

    /// Restricts the authorization to only users with specific groups
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
    resource_type: &'b ResourceType,
}

impl<'b> AuthorizationBuilder<'b> {
    pub fn new(resource_type: &'b ResourceType) -> Self {
        Self { resource_type }
    }

    fn check_groups(&self, user: &UserAttributes, policy: &Policy) -> Result<(), MinosError> {
        if let Some(possible_ids) = &policy.groups_ids {
            for id in possible_ids {
                if user.groups.contains(&id) {
                    return Ok(());
                }
            }

            return Err(MinosError::new(
                ErrorKind::Authorization,
                "The user is not in the correct group",
            ));
        }

        Ok(())
    }

    fn same_group_check(group_id: GroupId, user: &UserAttributes) -> Result<(), MinosError> {
        if !&user.groups.contains(&group_id) {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                "The user is not in the owning group",
            ));
        }

        Ok(())
    }

    fn same_user_check(user_id: &str, user: &UserAttributes) -> Result<(), MinosError> {
        if user_id != &user.id {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                "The user is not the owner",
            ));
        }
        Ok(())
    }

    fn by_owner_check(&self, user: &UserAttributes) -> Result<(), MinosError> {
        match &self.resource_type.owner {
            None => {
                return Err(MinosError::new(
                    ErrorKind::IncompatibleAuthPolicy,
                    "The resource haven't an owner",
                ));
            }
            Some(owner) => match owner {
                Owner::User(id) => Self::same_user_check(id, &user)?,
                Owner::Group(id) => Self::same_group_check(GroupId::from(id.as_str()), &user)?,
            },
        }

        Ok(())
    }

    /// Create an Authorization based in Policy, resource id, and User. This function unlike,
    /// [`build`], checks if the policy is malformed.
    ///
    /// # Errors
    /// This function will return an error in three cases:
    /// * [`InactiveUser`]: The user is not active.
    /// * [`IncompatibleAuthPolicy`]: The policy not corresponds to resource type or the attribute
    ///   `by_owner` is true, but the resource not have an owner.
    /// * [`Authorization`]: The user not have any permissions available.
    ///
    /// [`build`]: AuthorizationBuilder::build
    /// [`InactiveUser`]: ErrorKind::InactiveUser
    /// [`IncompatibleAuthPolicy`]: ErrorKind::IncompatibleAuthPolicy
    /// [`Authorization`]: ErrorKind::Authorization
    pub fn build_by_policy(
        &self,
        policy: &Policy,
        resource_id: &str,
        user: &UserAttributes,
    ) -> Result<Authorization, MinosError> {
        if user.status != Status::Active {
            return Err(MinosError::new(
                ErrorKind::InactiveUser,
                "The user is not active",
            ));
        }

        if !&self.resource_type.policies.contains(&policy) {
            return Err(MinosError::new(
                ErrorKind::IncompatibleAuthPolicy,
                "The policy not corresponds to resource type",
            ));
        }

        if policy.by_owner {
            let _ = self.by_owner_check(&user)?;
        } else {
            let _ = self.check_groups(&user, &policy)?;
        }

        Ok(Authorization {
            permissions: policy.permissions.clone(),
            user_id: user.id.clone(),
            resource_id: resource_id.to_string(),
            resource_type: self.resource_type.label.clone(),
            expiration: datetime_now() + Duration::seconds(policy.duration.clone() as i64),
        })
    }

    /// Create an Authorization based in resource id and User. Check all policies and assign all
    ///  permissions available to the user, but assign the shortest duration found.
    ///
    /// # Errors
    /// This function will return an error only in two cases:
    /// * [`InactiveUser`]: The user is not active.
    /// * [`Authorization`]: The user not have any permissions available.
    ///
    /// [`InactiveUser`]: ErrorKind::InactiveUser
    /// [`Authorization`]: ErrorKind::Authorization
    pub fn build(
        &self,
        resource_id: &str,
        user: &UserAttributes,
    ) -> Result<Authorization, MinosError> {
        if user.status != Status::Active {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                "The user is not active",
            ));
        }

        let mut permissions = vec![];
        let mut durations = vec![];
        for policy in &self.resource_type.policies {
            match self.build_by_policy(&policy, &resource_id, &user) {
                Ok(mut auth) => {
                    permissions.append(&mut auth.permissions);
                    durations.push(&policy.duration);
                }
                Err(_) => continue,
            }
        }

        durations.sort();
        let seconds = **durations
            .get(0)
            .ok_or(MinosError::new(ErrorKind::Authorization, "Not authorized"))?;

        Ok(Authorization {
            permissions: permissions,
            user_id: user.id.clone(),
            resource_id: resource_id.to_string(),
            resource_type: self.resource_type.label.clone(),
            expiration: datetime_now() + Duration::seconds(seconds as i64),
        })
    }
}
