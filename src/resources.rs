use crate::authorization::Policy;

#[derive(Debug, PartialEq, Clone, PartialOrd, Copy)]
pub enum OwnerType {
    User,
    Group,
    None,
}

impl Default for OwnerType {
    fn default() -> Self {
        Self::None
    }
}

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

impl ToString for Owner {
    fn to_string(&self) -> String {
        match self {
            Owner::User(id) => id.clone(),
            Owner::Group(id) => id.clone(),
        }
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
    pub(crate) owner_type: OwnerType,
    pub(crate) policies: Vec<Policy>,
}

impl ResourceType {
    pub fn new(label: String, owner_type: OwnerType, policies: Vec<Policy>) -> Self {
        Self {
            label,
            owner_type,
            policies,
        }
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn owner_type(&self) -> OwnerType {
        self.owner_type
    }
    pub fn policies(&self) -> &Vec<Policy> {
        &self.policies
    }

    #[cfg(feature = "unsafe_setters")]
    pub fn set_owner_type(&mut self, owner_type: OwnerType) {
        self.owner_type = owner_type
    }

    #[cfg(feature = "unsafe_setters")]
    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    #[cfg(feature = "unsafe_setters")]
    pub fn set_policies(&mut self, policies: Vec<Policy>) {
        self.policies = policies;
    }
}
