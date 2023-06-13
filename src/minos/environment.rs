use std::{collections::HashMap, marker::PhantomData};

use derived::Ctor;
use getset::Getters;

use crate::{errors::Error, minos::lang::Indentifier};

use super::{
    lang::Token,
    resource::{Resource, ResourceId, ResourceName},
};

pub type EnvName = String;

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct Environment{
    name: EnvName,
    resources: HashMap<ResourceName, Resource>,
    resources_identefied: HashMap<(ResourceName, ResourceId), Resource>,
}

impl TryFrom<&Token<'_>> for Environment {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_env().ok_or(Error::InvalidToken {
            expected: Token::Env(vec![]).to_string(),
            found: token.to_string(),
        })?;

        let Indentifier(name) = inner_tokens[0].inner_identifier().unwrap();
        let mut resources = HashMap::new();
        let mut resources_with_id = HashMap::new();
        for inner_token in inner_tokens.iter().skip(1) {
            let resource = Resource::try_from(inner_token)?;
            if let Some(id) = resource.id() {
                resources_with_id.insert((resource.name().clone(), id.to_string()), resource);
                continue;
            }

            resources.insert(resource.name().clone(), resource);
        }

        Ok(Environment {
            name: name.to_string(),
            resources,
            resources_identefied: resources_with_id,
        })
    }
}
