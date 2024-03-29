use derived::Ctor;
use getset::Getters;

use crate::{
    engine::{ActorRepr, ResourceRepr},
    errors::{Error, MinosResult},
    parser::tokens::Token,
};

use super::requirements::Requirement;

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct Rule {
    requirements: Vec<Requirement>,
}

impl Rule {
    /// Apply all requirements and return true only if actor satisfies all.
    pub(crate) fn apply(&self, actor: &ActorRepr, resource: &ResourceRepr) -> bool {
        for requirement in &self.requirements {
            if !requirement.apply(actor, resource).unwrap_or_default() {
                return false;
            }
        }

        true
    }
}

impl TryFrom<&Token> for Rule {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_rule().ok_or(Error::InvalidToken {
            expected: "Rule",
            found: token.to_string(),
        })?;
        let requirements: MinosResult<Vec<Requirement>> =
            inner_tokens.iter().map(Requirement::try_from).collect();

        Ok(Rule {
            requirements: requirements?,
        })
    }
}
