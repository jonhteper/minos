use derived::Ctor;
use getset::{Getters, MutGetters};

#[derive(Debug, Clone, PartialEq, Eq, Ctor, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Resource {
    name: String,
    id: Option<String>,
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
