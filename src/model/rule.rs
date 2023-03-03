use serde_json::map::Map;
use serde_json::value::Value as Value;
use crate::model::permission::Permission;
use crate::model::assertion::Assertion;
use crate::errors::MinosError;

pub trait ToRule {
    fn to_rule(&self, object: &Map<String, Value>) -> Result<Rule, MinosError>;
}

pub struct Rule {
    permissions: Vec<Permission>,
    actor_attributes: Map<String, Value>,
    resource_attributes: Map<String, Value>,
    environment_attributes: Map<String, Value>,
    assertions: Vec<Assertion>,
}

impl Rule {

}