use std::borrow::Cow;

use derived::Ctor;
use getset::{Getters, MutGetters};

#[derive(Debug, Clone, PartialEq, Eq, Ctor, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct Resource<'a> {
    resource_type: Cow<'a, str>,
    id: Option<Cow<'a, str>>,
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
