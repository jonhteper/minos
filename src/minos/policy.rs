use derived::Ctor;
use getset::Getters;

use crate::{authorization::Actor, errors::Error};

use super::rule::Rule;


pub type Permission = String;

#[derive(Debug, Clone, Ctor, Getters)]
#[getset(get = "pub")]
pub struct Policy {
    allow: Vec<Permission>,
    rules: Vec<Rule>,
}

impl Policy {
    /// Returns the [Permission] list if the actor satisfies at least one of the rules.
    /// This function can fail if the rules are bad created.
    pub fn apply(&self, actor: &impl Actor) -> Result<Option<&Vec<Permission>>, Error> {
        for rule in &self.rules {
            if rule.apply(actor)? {
                return Ok(Some(&self.allow));
            }
        }

        Ok(None)
    }
}

