use std::sync::Arc;

use derived::Ctor;
use getset::{Getters, MutGetters};

use crate::{
    language::requirements::Value,
    parser::tokens::{Identifier, ResourceAttribute},
};

#[derive(Debug, Clone, PartialEq, Eq, Ctor, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct Resource {
    resource_id: Option<Arc<str>>,
    resource_type: Arc<str>,
    resource_owner: Option<Arc<str>>,
}

impl Resource {
    pub(crate) fn get_attribute(&self, attr: ResourceAttribute) -> Option<Value> {
        match attr {
            ResourceAttribute::Id => self
                .resource_id
                .as_ref()
                .map(|id| Value::String(id.clone())),
            ResourceAttribute::Type => {
                Some(Value::Identifier(Identifier(self.resource_type.clone())))
            }
            ResourceAttribute::Owner => self
                .resource_owner
                .as_ref()
                .map(|owner| Value::String(owner.clone())),
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
