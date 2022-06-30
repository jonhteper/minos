use crate::errors::{ErrorKind, MinosError};
use crate::group::GroupId;
use crate::resources::{Owner, ResourceType};
use crate::user::UserAttributes;
use crate::utils::{datetime_now, string_as_datetime};
use crate::{utils, Status};
use chrono::{Duration, NaiveDateTime};
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;

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

#[derive(Debug, PartialEq, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct AuthorizationClaims {
    pub(crate) permissions: Vec<String>,
    pub(crate) userId: String,
    pub(crate) resourceId: String,
    pub(crate) resourceType: String,
    pub(crate) expiration: String,
}

impl AuthorizationClaims {
    pub fn new(
        permissions: Vec<String>,
        user_id: String,
        resource_id: String,
        resource_type: String,
        expiration: String,
    ) -> Self {
        Self {
            permissions,
            userId: user_id,
            resourceId: resource_id,
            resourceType: resource_type,
            expiration,
        }
    }
    pub fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }
    pub fn user_id(&self) -> &str {
        &self.userId
    }
    pub fn resource_id(&self) -> &str {
        &self.resourceId
    }
    pub fn resource_type(&self) -> &str {
        &self.resourceType
    }
    pub fn expiration(&self) -> &str {
        &self.expiration
    }

    fn string_permissions_to_vec_permissions(&self) -> Vec<Permission> {
        self.permissions
            .clone()
            .into_iter()
            .map(|p| Permission::from(p.as_str()))
            .collect()
    }

    pub fn as_authorization(
        &self,
        resource_type: &ResourceType,
    ) -> Result<Authorization, MinosError> {
        if &self.resourceType != &resource_type.label {
            return Err(MinosError::new(
                ErrorKind::Io,
                "The resource types not match",
            ));
        }

        Ok(Authorization {
            permissions: self.string_permissions_to_vec_permissions(),
            user_id: self.userId.clone(),
            resource_id: self.resourceId.clone(),
            resource_type: resource_type.clone(),
            expiration: string_as_datetime(&self.expiration)?,
        })
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

    fn permissions_as_vec_string(&self) -> Vec<String> {
        self.permissions
            .clone()
            .into_iter()
            .map(|p| p.to_string())
            .collect()
    }

    pub(crate) fn as_claims(&self) -> AuthorizationClaims {
        AuthorizationClaims {
            permissions: self.permissions_as_vec_string(),
            userId: self.user_id.clone(),
            resourceId: self.resource_id.clone(),
            resourceType: self.resource_type.label.clone(),
            expiration: self.expiration.format(utils::DATETIME_FMT).to_string(),
        }
    }

    pub fn token(&self, header: &Header, key: &EncodingKey) -> Result<String, MinosError> {
        let claims = &self.as_claims();
        Ok(jsonwebtoken::encode(header, &claims, key)?)
    }
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
