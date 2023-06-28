use std::collections::HashMap;

use derived::Ctor;
use lazy_static::lazy_static;

use crate::{
    errors::{Error, MinosResult},
    language::{
        environment::{EnvName, Environment},
        policy::{Permission, Policy},
    },
};

use super::{Actor, Resource};

lazy_static! {
    pub static ref EMPTY_POLICY_VEC: Vec<Policy> = Vec::new();
}

#[derive(Debug, Clone, Ctor)]
pub struct Engine<'env> {
    environments: &'env HashMap<EnvName, Environment>,
}

impl<'env> Engine<'env> {
    pub fn has_env(&self, env_name: &EnvName) -> bool {
        self.environments.contains_key(env_name)
    }

    fn get_policies_from_resource_identified<'a>(
        env: &'a Environment,
        resource: &Resource,
    ) -> MinosResult<&'a Vec<Policy>> {
        if let Some(id) = resource.id() {
            return Ok(env
                .resources_identified()
                .get(&(resource.resource_type().to_string(), id.to_string()))
                .map(|r| r.policies())
                .unwrap_or(&EMPTY_POLICY_VEC));
        }

        Ok(&EMPTY_POLICY_VEC)
    }

    fn get_policies(
        &self,
        env_name: &str,
        resource: &Resource,
    ) -> MinosResult<(&Vec<Policy>, &Vec<Policy>)> {
        let env = self
            .environments
            .get(env_name)
            .ok_or(Error::EnvironmentNotFound(env_name.to_string()))?;

        let resource_policies = env
            .resources()
            .get(resource.resource_type().as_ref())
            .ok_or(Error::ResourceNotFound(
                resource.resource_type().to_string(),
            ))?
            .policies();

        let policies_from_identified = Self::get_policies_from_resource_identified(env, resource)?;

        Ok((resource_policies, policies_from_identified))
    }

    /// Return a list of [Permission] if the [Actor] is authorized.
    /// This method fails if:
    /// * The [Actor] is not authorized
    pub fn authorize(
        &self,
        env_name: &str,
        actor: Actor,
        resource: Resource,
    ) -> MinosResult<Vec<Permission>> {
        let (policies, policies_from_identified) = self.get_policies(env_name, &resource)?;
        let mut permissions = vec![];

        for policy in policies {
            if let Some(granted_permissions) = policy.apply(&actor, &resource) {
                let mut perms = granted_permissions.clone();
                permissions.append(&mut perms);
            }
        }

        for policy in policies_from_identified {
            if let Some(granted_permissions) = policy.apply(&actor, &resource) {
                let mut perms = granted_permissions.clone();
                permissions.append(&mut perms);
            }
        }

        if permissions.is_empty() {
            return Err(Error::ActorNotAuthorized(actor.actor_id().to_string()));
        }

        Ok(permissions)
    }

    pub fn find_permission(
        &self,
        env_name: &str,
        actor: Actor,
        resource: Resource,
        permission: &Permission,
    ) -> MinosResult<()> {
        let permissions = self.authorize(env_name, actor, resource)?;
        if !permissions.contains(permission) {
            return Err(Error::PermissionNotFound(permission.clone()));
        }

        Ok(())
    }

    pub fn find_permissions(
        &self,
        env_name: &str,
        actor: Actor,
        resource: Resource,
        permissions: &[Permission],
    ) -> MinosResult<()> {
        let granted_permissions = self.authorize(env_name, actor, resource)?;
        for permission in permissions {
            if !granted_permissions.contains(permission) {
                return Err(Error::PermissionNotFound(permission.clone()));
            }
        }

        Ok(())
    }
}
