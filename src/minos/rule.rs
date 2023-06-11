use derived::Ctor;
use getset::Getters;

use crate::{authorization::Actor, errors::Error};

use super::requirements::Requirement;


#[derive(Debug, Clone, Ctor, Getters)]
#[getset(get = "pub")]
pub struct Rule {
    requirements: Vec<Requirement>,
}

impl Rule {
    /// Apply all requirements and return true only if actor satisfies all.
    pub fn apply(&self, actor: &impl Actor) -> Result<bool, Error> {
        // for requirement in &self.requirements {
        //     if !requirement.apply(actor)? {
        //         return Ok(false);
        //     }
        // }

        // Ok(true)
        todo!()
    }
}