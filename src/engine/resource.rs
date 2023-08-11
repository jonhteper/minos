use std::sync::Arc;

use getset::Getters;

use crate::{
    language::requirements::Value,
    parser::tokens::{Identifier, ResourceAttribute},
};

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct Resource {
    pub id: Option<String>,
    pub type_: String,
    pub owner: Option<String>,
    pub status: Option<String>,
}

pub trait AsResource {
    fn as_resource(&self) -> Resource;
}

pub trait IntoResource {
    fn into_resource(self) -> Resource;
}

pub trait TryIntoResource {
    type Error;
    fn try_into_resource(self) -> Result<Resource, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub(crate) struct ResourceRepr {
    pub id: Option<Arc<str>>,
    pub type_: Arc<str>,
    pub owner: Option<Arc<str>>,
    pub status: Option<Arc<str>>,
}

impl ResourceRepr {
    pub(crate) fn get_attribute(&self, attr: ResourceAttribute) -> Option<Value> {
        match attr {
            ResourceAttribute::Id => self.id.as_ref().map(|id| Value::String(id.clone())),
            ResourceAttribute::Type => Some(Value::Identifier(Identifier(self.type_.clone()))),
            ResourceAttribute::Owner => self.owner.as_ref().map(|owner| Value::String(owner.clone())),
            ResourceAttribute::Status => self
                .status
                .as_ref()
                .map(|status| Value::Identifier(Identifier(status.clone()))),
        }
    }
}

impl From<&Resource> for ResourceRepr {
    fn from(resource: &Resource) -> Self {
        Self {
            id: resource.id.as_ref().map(|id| Arc::from(id.as_str())),
            type_: Arc::from(resource.type_.as_str()),
            owner: resource.owner.as_ref().map(|owner| Arc::from(owner.as_str())),
            status: resource.status.as_ref().map(|status| Arc::from(status.as_str())),
        }
    }
}
