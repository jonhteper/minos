use crate::authorization::Policy;
use crate::errors::{ErrorKind, MinosError};
use crate::resources::Resource;
use non_empty_string::NonEmptyString;

/// Contains auxiliar information for Resource buildind.
///
/// It's possible save this information in a file or in a database.
#[derive(PartialEq, Eq, Debug, Clone, PartialOrd)]
pub struct ResourceManifest {
    /// The most important field, because it func like id.
    resource_type: NonEmptyString,

    /// Can prevent an unnecessary request to persistent layer to
    /// find a Resource Owner.
    owner: bool,
    policies: Vec<Policy>,
}

impl ResourceManifest {
    pub fn try_from_resource<R: Resource>(resource: &R) -> Result<Self, MinosError> {
        let resource_type = resource.resource_type().ok_or_else(|| {
            MinosError::new(
                ErrorKind::MissingResourceType,
                "The resource needs an explicit resource type",
            )
        })?;
        let owner = resource.owner().is_some();

        Ok(Self {
            resource_type,
            owner,
            policies: resource.policies(),
        })
    }

    pub fn resource_type(&self) -> &NonEmptyString {
        &self.resource_type
    }

    pub fn owner(&self) -> bool {
        self.owner
    }

    pub fn policies(&self) -> Vec<Policy> {
        self.policies.clone()
    }
}
