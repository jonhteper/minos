//! This module allows you save an read [`Resource`] policies in toml files
use crate::agent::Agent;
use crate::authorization::{Authorization, Permission, Policy};
use crate::authorization_builder::AuthorizationBuilder;
use crate::errors::{ErrorKind, MinosError};
use crate::resources::Resource;
use crate::NonEmptyString;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
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
        let stored_rs = StoredResource::try_from_resource(resource)?;
        let content = toml::to_string(&stored_rs)?;
        let mut file = File::create(path)?;
        let _ = file.write_all(content.as_bytes())?;

        Ok(Self { file })
    }
}

impl TryFrom<PathBuf> for TomlFile {
    type Error = MinosError;

    /// Saves the file
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let extension = path
            .extension()
            .ok_or(MinosError::new(
                ErrorKind::BadExtension,
                "The file not have extension",
            ))?
            .to_str();

        if extension != Some("toml") && extension != Some("resource") {
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

#[derive(PartialEq, Debug, Clone, PartialOrd, Serialize, Deserialize)]
struct StoredPolicy {
    duration: u64,
    by_owner: bool,
    groups_ids: Option<Vec<String>>,
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
        if let Some(groups_ids) = &self.groups_ids {
            groups = Some(StoredPolicy::vec_string_as_vec_group_id(groups_ids.clone()));
        }

        Ok(Policy {
            duration: self.duration,
            by_owner: self.by_owner,
            groups_ids: groups,
            permissions: StoredPolicy::vec_string_as_vec_permissions(self.permissions.clone()),
        })
    }
}

impl From<Policy> for StoredPolicy {
    fn from(policy: Policy) -> Self {
        let groups_ids = match policy.groups_ids {
            None => None,
            Some(ids) => Some(ids.into_iter().map(|id| id.to_string()).collect()),
        };
        let permissions = policy
            .permissions
            .into_iter()
            .map(|p| p.to_string())
            .collect();
        Self {
            duration: policy.duration,
            by_owner: policy.by_owner,
            groups_ids,
            permissions,
        }
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Serialize, Deserialize)]
pub struct StoredResource {
    label: String,
    policies: Vec<StoredPolicy>,
}

impl StoredResource {
    pub fn try_from_resource<R: Resource>(resource: &R) -> Result<Self, MinosError> {
        let label = resource
            .resource_type()
            .ok_or(MinosError::new(
                ErrorKind::Toml,
                "The resource needs an explicit resource type",
            ))?
            .to_string();

        let policies = resource
            .policies()
            .into_iter()
            .map(|p| StoredPolicy::from(p))
            .collect();

        Ok(Self { label, policies })
    }

    pub fn resource_type(&self) -> Option<NonEmptyString> {
        NonEmptyString::from_str(&self.label)
    }

    pub fn policies(&self) -> Vec<Policy> {
        self.policies
            .clone()
            .into_iter()
            .flat_map(|p| p.as_policy().ok())
            .collect()
    }
}

impl TryFrom<TomlFile> for StoredResource {
    type Error = MinosError;

    fn try_from(toml_file: TomlFile) -> Result<Self, Self::Error> {
        let mut file = toml_file;
        let decoded: StoredResource = toml::from_str(&mut file.try_to_string()?)?;

        Ok(decoded)
    }
}
