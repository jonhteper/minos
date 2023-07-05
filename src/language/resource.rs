use std::{collections::HashMap, sync::Arc};

use derived::Ctor;
use getset::Getters;

use crate::{
    errors::{Error, MinosResult},
    parser::tokens::{Identifier, Token},
};

use super::environment::Environment;

pub type ResourceId = String;
pub type ResourceName = String;

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct Resource {
    identifier: Identifier,
    environments: HashMap<Identifier, Environment>,
}

impl Resource {
    fn collect_envs_from_tokens<'a>(
        iterator: impl Iterator<Item = &'a Token>,
    ) -> MinosResult<Vec<Environment>> {
        iterator.map(Environment::try_from).collect()
    }

    fn collect_hash_map_env_from_vec(list: Vec<Environment>) -> HashMap<Identifier, Environment> {
        let mut environments: HashMap<Identifier, Environment> = HashMap::new();
        for mut env in list {
            if let Some(environment) = environments.get_mut(env.identifier()) {
                environment.add_policies(env.policies_mut());
            } else {
                environments.insert(env.identifier().clone(), env);
            }
        }

        environments
    }

    /// Adds an [Environment] into the [Resource].
    pub fn add_environment(&mut self, environment: Environment) {
        let mut env = environment;
        if let Some(environment) = self.environments.get_mut(env.identifier()) {
            environment.add_policies(env.policies_mut());
            return;
        }

        self.environments.insert(env.identifier().clone(), env);
    }


    /// Join two [Resource]'s if both have the same [Identifier].
    /// Returns `true` if the two resources are joined.
    pub fn join(&mut self, resource: Resource) -> bool {
        if self.identifier() != resource.identifier() {
            return false;
        }

        for (_, env) in resource.environments {
            self.add_environment(env);
        }

        true
    }
}

impl TryFrom<&Token> for Resource {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_resource().unwrap();

        let identifier = inner_tokens[0].inner_identifier().unwrap().clone();
        let env_list = Self::collect_envs_from_tokens(inner_tokens.iter().skip(1))?;
        let environments = Self::collect_hash_map_env_from_vec(env_list);

        Ok(Self {
            identifier,
            environments,
        })
    }
}

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct AttributedResource {
    identifier: Identifier,
    id: Arc<str>,
    environments: HashMap<Identifier, Environment>,
}

impl TryFrom<&Token> for AttributedResource {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_resource().unwrap();

        let identifier = inner_tokens[0].inner_identifier().unwrap().clone();
        let id = inner_tokens[1].inner_str().unwrap().clone();
        let env_list = Resource::collect_envs_from_tokens(inner_tokens.iter().skip(2))?;
        let environments = Resource::collect_hash_map_env_from_vec(env_list);

        Ok(Self {
            identifier,
            id,
            environments,
        })
    }
}
