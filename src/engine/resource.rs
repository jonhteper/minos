use std::{borrow::Cow, sync::Arc};

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
            ResourceAttribute::Id => self
                .resource_id
                .as_ref()
                .map(|id| Value::String(Arc::from(id.as_ref()))),
            ResourceAttribute::Type => Some(Value::Identifier(Identifier(
                self.resource_type.as_ref().into(),
            ))),
            ResourceAttribute::Owner => self
                .resource_owner
                .as_ref()
                .map(|owner| Value::String(Arc::from(owner.as_ref()))),
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
