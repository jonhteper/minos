use std::borrow::Cow;

use derived::Ctor;
use getset::{Getters, MutGetters};

use crate::{
    language::requirements::Value,
    parser::tokens::{Identifier, ResourceAttribute},
};

#[derive(Debug, Clone, PartialEq, Eq, Ctor, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct Resource<'a> {
    resource_id: Option<Cow<'a, str>>,
    resource_type: Cow<'a, str>,
    resource_owner: Option<Cow<'a, str>>,
}

impl<'a> Resource<'a> {
    pub(crate) fn get_attribute(&self, attr: ResourceAttribute) -> Option<Value> {
        match attr {
            ResourceAttribute::Id => self.resource_id.map(|id| Value::String(&id)),
            ResourceAttribute::Type => Some(Value::Identifier(Identifier(&self.resource_type))),
            ResourceAttribute::Owner => self.resource_owner.map(|owner| Value::String(&owner)),
        }
    }
}

pub trait AsResource {
    fn as_resource(&self) -> Resource;
}

pub trait IntoResource {
    fn into_resource<'a>(self) -> Resource<'a>;
}

pub trait TryIntoResource {
    type Error;
    fn try_into_resource<'a>(self) -> Result<Resource<'a>, Self::Error>;
}
