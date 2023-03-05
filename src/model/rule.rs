use crate::errors::MinosError;
use crate::model::assertion::{Assertion, ToAssertions};
use crate::model::attribute::Attribute;
use crate::model::permission::{Permission, ToPermissions};
use derive_builder::Builder;
use serde_json::map::Map;
use serde_json::value::Value;

pub trait ToRule: ToPermissions + ToAssertions {
    fn to_rule(&self) -> Result<Rule, MinosError>;
}

#[derive(Debug, Default, Clone)]
pub struct ActorRuleAttributes<'at> {
    groups: Vec<String>,
    attributes: Vec<Attribute<'at>>,
}

#[derive(Debug, Default, Clone)]
pub struct ResourceRuleAttributes<'at> {
    owner_check: bool,
    attributes: Vec<Attribute<'at>>,
}

#[derive(Default, Clone, Debug, Builder)]
pub struct Rule<'a> {
    permissions: Vec<Permission>,
    actor_attributes: ActorRuleAttributes<'a>,
    resource_attributes: ResourceRuleAttributes<'a>,
    environment_attributes: Vec<Attribute<'a>>,
    assertions: Vec<Assertion>,
}
