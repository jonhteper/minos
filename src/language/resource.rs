use std::{collections::HashMap, sync::Arc};

use derived::Ctor;
use getset::Getters;

use crate::{
    errors::{Error, MinosResult},
    parser::tokens::{Identifier, Token},
};

use super::environment::{Environment, DEFAULT_ENV_IDENTIFIER};

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

    /// Merge two [Environment]. If exist repeatedly environments,
    ///  the inner rules will be merged.
    pub fn merge(&mut self, resource: Resource) {
        for (_, env) in resource.environments {
            self.add_environment(env);
        }
    }

    pub fn default_environment(&self) -> Option<&Environment> {
        self.environments.get(&Identifier(DEFAULT_ENV_IDENTIFIER.into()))
    }

    pub fn get_environment(&self, env: &str) -> Option<&Environment> {
        self.environments.get(&Identifier::from(env))
    }

    pub fn policies_len(&self) -> usize {
        let mut len = 0;
        for (_, env) in self.environments() {
            len += env.policies().len();
        }

        len
    }
}

impl TryFrom<&Token> for Resource {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_resource().ok_or(Error::InvalidToken {
            expected: "Resource",
            found: token.to_string(),
        })?;

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

impl AttributedResource {
    /// Adds an [Environment] into the [AttributedResource].
    pub fn add_environment(&mut self, environment: Environment) {
        let mut env = environment;
        if let Some(environment) = self.environments.get_mut(env.identifier()) {
            environment.add_policies(env.policies_mut());
            return;
        }

        self.environments.insert(env.identifier().clone(), env);
    }

    /// Merge two [Environment]. If exist repeatedly environments,
    ///  the inner rules will be merged.
    pub fn merge(&mut self, resource: AttributedResource) {
        for (_, env) in resource.environments {
            self.add_environment(env);
        }
    }

    pub fn default_environment(&self) -> Option<&Environment> {
        self.environments.get(&Identifier(DEFAULT_ENV_IDENTIFIER.into()))
    }

    pub fn get_environment(&self, env: &str) -> Option<&Environment> {
        self.environments.get(&Identifier::from(env))
    }

    pub fn policies_len(&self) -> usize {
        let mut len = 0;
        for (_, env) in self.environments() {
            len += env.policies().len();
        }

        len
    }
}

impl TryFrom<&Token> for AttributedResource {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_attributed_resource().ok_or(Error::InvalidToken {
            expected: "AttributedResource",
            found: token.to_string(),
        })?;

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
