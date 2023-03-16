use crate::errors::MinosError;
use serde_json::{Map, Value};

pub struct Actor<'a> {
    id: &'a str,
    groups: Vec<String>,
    attributes: Map<String, Value>,
}

pub trait ToActor {
    fn to_actor(&self) -> Result<Actor<'_>, MinosError>;
}
