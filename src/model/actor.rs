use crate::errors::MinosError;
use crate::model::attribute::Attribute;

pub struct Actor<'a> {
    id: &'a str,
    groups: Vec<String>,
    attributes: Vec<Attribute<'a>>,
}

pub trait ToActor {
    fn to_actor(&self) -> Result<Actor<'_>, MinosError>;
}
