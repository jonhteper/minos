use std::collections::HashMap;

use derived::Ctor;
use getset::Getters;

use super::resource::{ResourceName, Resource, ResourceId};

pub type EnvName = String;

#[derive(Debug, Clone, Ctor, Getters)]
#[getset(get = "pub")]
pub struct Environment {
    name: EnvName,
    resources: HashMap<(ResourceName, Option<ResourceId>), Resource>,
}