use crate::authorization::Policy;

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
    fn id(&self) -> String;
    fn resource_type(&self) -> ResourceType;
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
}
