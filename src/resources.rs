use crate::authorization::Policy;
use crate::errors::{ErrorKind, MinosError};

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Owner {
    User(String),
    Group(String),
}

impl Default for Owner {
    fn default() -> Self {
        Self::User("".to_string())
    }
}

pub trait Resource {
    type Error;
    fn id(&self) -> String;
    fn owner(&self) -> Result<Option<Owner>, Self::Error>;
    fn resource_type(&self) -> Result<ResourceType, Self::Error>;
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Default)]
pub struct ResourceType {
    pub(crate) label: String,
    pub(crate) owner: Option<Owner>,
    pub(crate) policies: Vec<Policy>,
}

impl ResourceType {
    pub fn new(label: String, owner: Option<Owner>, policies: Vec<Policy>) -> Self {
        Self {
            label,
            owner,
            policies,
        }
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn owner(&self) -> &Option<Owner> {
        &self.owner
    }
    pub fn policies(&self) -> &Vec<Policy> {
        &self.policies
    }

    #[cfg(feature = "unsafe_setters")]
    pub fn set_owner(&mut self, owner: Owner) {
        self.owner = Some(owner)
    }

    #[cfg(feature = "unsafe_setters")]
    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    #[cfg(feature = "unsafe_setters")]
    pub fn set_policies(&mut self, policies: Vec<Policy>) {
        self.policies = policies;
    }

    /// Modify the owner id securely. Use this method if the ResourceType are built.
    ///
    /// **Warning**: You can't really change the owner, only the id. If the owner is an [`Owner::User`] can't
    /// change to [`Owner::Group`]
    /// # Errors
    /// * The resource type not have an owner
    /// * The owner id is not empty, and the param `overwrite` is not true
    pub fn safe_set_owner(&mut self, owner_id: &str, overwrite: bool) -> Result<(), MinosError> {
        if self.owner.is_none() {
            return Err(MinosError::new(
                ErrorKind::ResourceType,
                "The resource type not have an owner",
            ));
        }

        self.owner = match self.owner.as_ref().unwrap() {
            Owner::User(id) => {
                if !id.is_empty() && !overwrite {
                    return Err(MinosError::new(
                        ErrorKind::ResourceType,
                        "The id is not empty",
                    ));
                }
                Some(Owner::User(owner_id.to_string()))
            }
            Owner::Group(id) => {
                if !id.is_empty() && !overwrite {
                    return Err(MinosError::new(
                        ErrorKind::ResourceType,
                        "The id is not empty",
                    ));
                }
                Some(Owner::Group(owner_id.to_string()))
            }
        };

        Ok(())
    }
}
