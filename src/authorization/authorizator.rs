use std::collections::HashMap;

use derived::Ctor;

use crate::{
    errors::Error,
    minos::{
        environment::{EnvName, Environment},
        policy::{Permission, Policy},
    },
};

use super::{resource, Actor, Resource};

#[derive(Debug, Clone, Ctor)]
pub struct Authorizator {
    environments: HashMap<EnvName, Environment>,
}

impl Authorizator {
    pub fn has_env(&self, env_name: &EnvName) -> bool {
        self.environments.contains_key(env_name)
    }

    pub fn add_environment(&mut self, env: Environment) {
        self.environments.insert(env.name().clone(), env);
    }

    fn get_policies(
        &self,
        env_name: &EnvName,
        resource: &impl Resource,
    ) -> Result<&Vec<Policy>, Error> {
        let env = self
            .environments
            .get(env_name)
            .ok_or(Error::EnvironmentNotFound(env_name.clone()))?;

        let resource = env
            .resources()
            .get(&(resource.name(), resource.id()))
            .ok_or(Error::ResourceNotFound(resource.name().clone()))?;

        Ok(resource.policies())
    }

    /// Return a list of [Permission] if the [Actor] is authorized.
    /// This method fails if:
    /// * The [Actor] is not authorized
    pub fn authorize(
        &self,
        env_name: &EnvName,
        actor: &impl Actor,
        resource: &impl Resource,
    ) -> Result<Vec<Permission>, Error> {
        let policies = self.get_policies(env_name, resource)?;
        let mut permissions = vec![];

        for policy in policies {
            if let Some(granted_permissions) = policy.apply(actor) {
                let mut perms = granted_permissions.clone();
                permissions.append(&mut perms);
            }
        }

        if permissions.is_empty() {
            return Err(Error::ActorNotAuthorized(actor.actor_id()));
        }

        Ok(permissions)
    }
}
