use derived::Ctor;
use getset::Getters;

use crate::{
    authorization::Actor,
    errors::{Error, MinosResult},
};

use super::{
    lang::Token,
    requirements::{self, Requirement},
};

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

impl TryFrom<&Token<'_>> for Rule {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_rule().ok_or(Error::InvalidToken {
            expected: Token::Rule(vec![]).to_string(),
            found: token.to_string(),
        })?;
        let requirements: MinosResult<Vec<Requirement>> = inner_tokens
            .iter()
            .map(|r| Requirement::try_from(r))
            .collect();

        Ok(Rule {
            requirements: requirements?,
        })
    }
}
