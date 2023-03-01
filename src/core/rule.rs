use serde_json::map::Map;
use serde_json::value::Value;
use crate::core::permission::Permission;
use crate::core::assertion::Assertion;
use crate::errors::MinosError;

pub struct Rule {
    permissions: Vec<Permission>,
    actor_attributes: Map<String, Value>,
    resource_attributes: Map<String, Value>,
    environment_attributes: Map<String, Value>,
    assertions: Vec<Assertion>,
}

impl TryFrom<String> for Rule {
    type Error = MinosError;
    fn try_from(str: String) -> Result<Self, Self::Error> {
        todo!()
    }
}