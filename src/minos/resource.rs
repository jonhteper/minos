use derived::Ctor;
use getset::Getters;

use crate::{
    errors::{Error, MinosResult},
    minos::lang::Indentifier,
};

use super::{lang::Token, policy::Policy};

pub type ResourceId = String;
pub type ResourceName = String;

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct Resource {
    name: ResourceName,
    id: Option<ResourceId>,
    policies: Vec<Policy>,
}

impl TryFrom<&Token<'_>> for Resource {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_resource().ok_or(Error::InvalidToken {
            expected: Token::Resource(vec![]).to_string(),
            found: token.to_string(),
        })?;

        let Indentifier(name) = inner_tokens[0].inner_identifier().unwrap();
        let id = inner_tokens[1].inner_string().map(|s| s.to_string());

        let policies: MinosResult<Vec<Policy>> = match id.is_some() {
            true => inner_tokens.iter().skip(2).map(Policy::try_from).collect(),
            false => inner_tokens.iter().skip(1).map(Policy::try_from).collect(),
        };

        Ok(Resource {
            name: name.to_string(),
            id,
            policies: policies?,
        })
    }
}
