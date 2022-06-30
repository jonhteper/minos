use crate::authorization::Policy;

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
