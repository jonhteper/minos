use crate::authorization::{Permission, Policy};
use crate::errors::{ErrorKind, MinosError};
use crate::group::GroupId;
use crate::resources::{Owner, ResourceType};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[cfg(feature = "toml_storage")]
pub struct TomlFile {
    file: File,
}

impl TomlFile {
    pub(crate) fn try_to_string(&mut self) -> Result<String, MinosError> {
        let mut content = String::new();
        let _ = &self.file.read_to_string(&mut content)?;

        Ok(content)
    }
}

#[cfg(feature = "toml_storage")]
impl TryFrom<PathBuf> for TomlFile {
    type Error = MinosError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let extension = path
            .extension()
            .ok_or(MinosError::new(
                ErrorKind::BadExtension,
                "The file not have extension",
            ))?
            .to_str();

        if extension != Some("toml") {
            return Err(MinosError::new(
                ErrorKind::BadExtension,
                "The file not have the correct extension",
            ));
        }

        Ok(Self {
            file: File::open(&path)?,
        })
    }
}

#[cfg(feature = "toml_storage")]
#[derive(PartialEq, Debug, Clone, PartialOrd, Serialize, Deserialize)]
struct StoredOwner {
    user: Option<bool>,
    group: Option<bool>,
    id: Option<String>,
}

impl StoredOwner {
    /// Warning: if id attribute not exists, the default value will be an empty String
    fn maybe_owner(&self) -> Option<Owner> {
        let id = match &self.id {
            None => "".to_string(),
            Some(id) => id.clone(),
        };

        if let Some(user) = self.user {
            if user {
                return Some(Owner::User(id));
            }
        }

        if let Some(group) = self.group {
            if group {
                return Some(Owner::Group(id));
            }
        }

        None
    }
}

#[cfg(feature = "toml_storage")]
#[derive(PartialEq, Debug, Clone, PartialOrd, Serialize, Deserialize)]
struct StoredPolicy {
    duration: Option<u16>,
    by_owner: Option<bool>,
    groups_ids: Option<Vec<String>>,
    permissions: Option<Vec<String>>,
}

impl StoredPolicy {
    fn vec_string_as_vec_group_id(vec: &Vec<String>) -> Vec<GroupId> {
        vec.clone().into_iter().map(|g| GroupId::from(g)).collect()
    }

    fn vec_string_as_vec_permissions(vec: &Vec<String>) -> Vec<Permission> {
        vec.clone()
            .into_iter()
            .map(|p| Permission::from(p.as_str()))
            .collect()
    }

    fn as_policy(&self) -> Policy {
        let mut policy = Policy::default();
        if let Some(duration) = &self.duration {
            policy.duration = duration.clone();
        }

        if let Some(by_owner) = &self.by_owner {
            policy.by_owner = by_owner.clone();
        }

        if let Some(groups_ids) = &self.groups_ids {
            policy.groups_ids = Some(StoredPolicy::vec_string_as_vec_group_id(groups_ids));
        }

        if let Some(permission) = &self.permissions {
            policy.permissions = StoredPolicy::vec_string_as_vec_permissions(permission);
        }

        policy
    }
}

#[cfg(feature = "toml_storage")]
#[derive(PartialEq, Debug, Clone, PartialOrd, Serialize, Deserialize)]
struct StoredResourceType {
    label: Option<String>,
    owner: Option<StoredOwner>,
    policies: Option<Vec<StoredPolicy>>,
}

impl StoredResourceType {
    fn string_policies_as_policies(&self) -> Vec<Policy> {
        if self.policies.is_none() {
            return vec![];
        }

        self.policies
            .clone()
            .unwrap()
            .into_iter()
            .map(|s| s.as_policy())
            .collect()
    }
}

#[cfg(feature = "toml_storage")]
impl From<StoredResourceType> for ResourceType {
    fn from(stored: StoredResourceType) -> Self {
        let mut resource_type = ResourceType::default();
        resource_type.label = match &stored.label {
            None => "".to_string(),
            Some(label) => label.clone(),
        };

        resource_type.owner = match &stored.owner {
            None => None,
            Some(owner) => owner.maybe_owner().clone(),
        };

        resource_type.policies = stored.string_policies_as_policies();

        resource_type
    }
}

#[cfg(feature = "toml_storage")]
impl TryFrom<TomlFile> for ResourceType {
    type Error = MinosError;

    fn try_from(toml_file: TomlFile) -> Result<Self, Self::Error> {
        let mut file = toml_file;
        let decoded: StoredResourceType = toml::from_str(&mut file.try_to_string()?)?;

        Ok(ResourceType::from(decoded))
    }
}
