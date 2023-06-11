use derived::Ctor;
use getset::Getters;

use super::policy::Policy;

pub type ResourceId = String;
pub type ResourceName = String;

#[derive(Debug, Clone, Ctor, Getters)]
#[getset(get = "pub")]
pub struct Resource {
    name: ResourceName,
    id: Option<ResourceId>,
    policies: Vec<Policy>,
}