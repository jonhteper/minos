//! This module allows you save an read [`Resource`] policies in toml files
use crate::core::authorization::{AuthorizationMode, Permission, Policy};
use crate::core::resources::Resource;
use crate::errors::MinosError;
use crate::resource_manifest::ResourceManifest;
use non_empty_string::NonEmptyString;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::num::NonZeroU64;
use std::path::PathBuf;

pub struct TomlFile {
    file: File,
}

impl TomlFile {
    pub(crate) fn try_to_string(&mut self) -> Result<String, MinosError> {
        let mut content = String::new();
        let _ = &self.file.read_to_string(&mut content)?;

        Ok(content)
    }

    /// Create the toml file in the path
    pub fn create<R: Resource>(resource: &R, path: &PathBuf) -> Result<Self, MinosError> {
        let stored_manifest = StoredManifest::from(&ResourceManifest::try_from_resource(resource)?);
        let content = toml::to_string(&stored_manifest)?;
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;

        Ok(Self { file })
    }
}

impl TryFrom<&PathBuf> for TomlFile {
    type Error = MinosError;

    /// Saves the file
    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let extension = path.extension().ok_or(MinosError::NoExtension)?.to_str();

        let valid_extensions = [
            Some("toml"),
            Some("resource"),
            Some("manifest"),
            Some("minos"),
        ];

        if !valid_extensions.contains(&extension) {
            return Err(MinosError::BadExtension);
        }

        Ok(Self {
            file: File::open(path)?,
        })
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Serialize, Deserialize)]
struct StoredPolicy {
    id: Option<String>,
    resource_type: Option<String>,
    duration: u64,
    auth_mode: String,
    groups: Option<Vec<String>>,
    permissions: Vec<String>,
}

impl StoredPolicy {
    fn vec_string_as_vec_group_id(vec: Vec<String>) -> Vec<NonEmptyString> {
        vec.into_iter()
            .filter_map(|g| NonEmptyString::try_from(g.as_str()).ok())
            .collect()
    }

    fn vec_string_as_vec_permissions(vec: Vec<String>) -> Vec<Permission> {
        vec.into_iter()
            .map(|p| Permission::from(p.as_str()))
            .collect()
    }

    fn as_policy(&self) -> Result<Policy, MinosError> {
        let mut groups = None;
        if let Some(groups_ids) = &self.groups {
            groups = Some(StoredPolicy::vec_string_as_vec_group_id(groups_ids.clone()));
        }
        let duration = NonZeroU64::new(self.duration).ok_or(MinosError::ZeroValueDuration)?;
        let id = self
            .id
            .as_ref()
            .and_then(|str| NonEmptyString::try_from(str.as_str()).ok());
        let resource_type = self
            .resource_type
            .as_ref()
            .and_then(|str| NonEmptyString::try_from(str.as_str()).ok());

        Ok(Policy {
            id,
            resource_type,
            duration,
            auth_mode: AuthorizationMode::try_from(self.auth_mode.as_str())?,
            groups,
            permissions: StoredPolicy::vec_string_as_vec_permissions(self.permissions.clone()),
        })
    }
}

impl From<Policy> for StoredPolicy {
    fn from(policy: Policy) -> Self {
        let groups_ids = policy
            .groups
            .map(|ids| ids.into_iter().map(|id| id.to_string()).collect());

        let permissions = policy
            .permissions
            .into_iter()
            .map(|p| p.to_string())
            .collect();
        let id = policy.id.as_ref().map(|id| id.to_string());
        let resource_type = policy.resource_type.map(|str| str.to_string());

        Self {
            id,
            resource_type,
            duration: policy.duration.get(),
            auth_mode: policy.auth_mode.to_string(),
            groups: groups_ids,
            permissions,
        }
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Serialize, Deserialize)]
pub struct StoredManifest {
    resource_type: String,
    owner: bool,
    policies: Vec<StoredPolicy>,
}

impl From<&ResourceManifest> for StoredManifest {
    fn from(manifest: &ResourceManifest) -> Self {
        let policies = manifest
            .policies()
            .into_iter()
            .map(StoredPolicy::from)
            .collect();

        Self {
            resource_type: manifest.resource_type().to_string(),
            owner: manifest.owner(),
            policies,
        }
    }
}

impl StoredManifest {
    pub fn resource_type(&self) -> Option<NonEmptyString> {
        NonEmptyString::try_from(self.resource_type.as_str()).ok()
    }

    pub fn owner(&self) -> bool {
        self.owner
    }

    pub fn policies(&self) -> Vec<Policy> {
        self.policies
            .clone()
            .into_iter()
            .flat_map(|p| p.as_policy().ok())
            .collect()
    }
}

impl TryFrom<TomlFile> for StoredManifest {
    type Error = MinosError;

    fn try_from(toml_file: TomlFile) -> Result<Self, Self::Error> {
        let mut file = toml_file;
        let decoded: StoredManifest = toml::from_str(&file.try_to_string()?)?;

        Ok(decoded)
    }
}
