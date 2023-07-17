use std::sync::Arc;

use getset::{Getters, MutGetters};

use crate::{
    language::requirements::Value,
    parser::tokens::{Identifier, ResourceAttribute},
};

#[derive(Debug, Clone, PartialEq, Eq, Getters, MutGetters)]
#[get = "pub"]
pub struct Resource {
    pub id: Option<Arc<str>>,
    pub type_: Arc<str>,
    pub owner: Option<Arc<str>>,
    pub status: Option<Arc<str>>,
}

impl Resource {
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
