use std::sync::Arc;

use derived::Ctor;
use either::Either;
use lazy_static::lazy_static;

use crate::{
    errors::{Error, MinosResult},
    language::{
        environment::Environment,
        policy::{Permission, Policy},
        resource::AttributedResource,
        resource::Resource as InternalResource,
        storage::Storage,
    },
};

use super::{Actor, Resource};

lazy_static! {
    pub static ref EMPTY_POLICY_VEC: Vec<Policy> = Vec::new();
}

pub struct AuthorizeRequest<'a> {
    pub env_name: Option<&'a str>,
    pub actor: Actor,
    pub resource: Resource,
}

struct InternalAuthorizeRequest<'a> {
    pub env_name: &'a Option<&'a str>,
    pub actor: &'a Actor,
    pub resource: &'a Resource,
    pub minos_resource: Either<&'a InternalResource, &'a AttributedResource>,
}

#[derive(Debug, Clone, Ctor)]
pub struct Engine<'s> {
    storage: &'s Storage,
}

impl<'s> Engine<'s> {
    fn append_permissions(
        permissions: &mut Vec<Permission>,
        environment: &Environment,
        actor: &Actor,
        resource: &Resource,
    ) {
        for policy in environment.policies() {
            if let Some(inner_permissions) = policy.apply(actor, resource) {
                let mut perms = inner_permissions.to_vec();
                permissions.append(&mut perms);
            }
        }
    }

    fn find_attributed_resource(
        &self,
        resource_id: Arc<str>,
        resource: &Resource,
    ) -> Option<&AttributedResource> {
        self.storage
            .attributed_resources()
            .get(&(resource.resource_type().into(), resource_id))
    }

    fn authorize_attributed_resource(
        &self,
        request: InternalAuthorizeRequest,
    ) -> MinosResult<Vec<Permission>> {
        let actor = request.actor;
        let resource = request.resource;
        let attr_resource = request.minos_resource.unwrap_right();

        let mut permissions = vec![];
        if let Some(default_env) = attr_resource.default_environment() {
            Self::append_permissions(&mut permissions, default_env, actor, resource);
        }

        if let Some(env_name) = request.env_name {
            let env = attr_resource
                .get_environment(env_name)
                .ok_or(Error::EnvironmentNotFound(env_name.to_string()))?;
            Self::append_permissions(&mut permissions, env, actor, resource);
        }

        if permissions.is_empty() {
            return Err(Error::ActorNotAuthorized(actor.actor_id().to_string()));
        }

        return Ok(permissions);
    }

    fn authorize_resource(&self, request: InternalAuthorizeRequest) -> MinosResult<Vec<Permission>> {
        let inner_resource = request.minos_resource.unwrap_left();

        let mut permissions = vec![];
        if let Some(default_env) = inner_resource.default_environment() {
            Self::append_permissions(&mut permissions, default_env, request.actor, request.resource);
        }

        if let Some(env_name) = request.env_name {
            let env = inner_resource
                .get_environment(env_name)
                .ok_or(Error::EnvironmentNotFound(env_name.to_string()))?;
            Self::append_permissions(&mut permissions, env, request.actor, request.resource);
        }

        Ok(permissions)
    }

    /// Return a list of [Permission] if the [Actor] is authorized.
    /// This method fails if:
    /// * The [Actor] is not authorized.
    /// * The environment's name not exist into the [Storage].
    pub fn authorize(&self, request: AuthorizeRequest) -> MinosResult<Vec<Permission>> {
        let AuthorizeRequest {
            env_name,
            actor,
            resource,
        } = &request;
        let mut permissions = vec![];

        if let Some(resource_id) = resource.resource_id() {
            if let Some(attr_resource) = self.find_attributed_resource(resource_id.clone(), resource) {
                return self.authorize_attributed_resource(InternalAuthorizeRequest {
                    env_name,
                    actor,
                    resource,
                    minos_resource: Either::Right(attr_resource),
                });
            }
        }

        if let Some(inner_resource) = self.storage.resources().get(&resource.resource_type().into()) {
            permissions = self.authorize_resource(InternalAuthorizeRequest {
                env_name,
                actor,
                resource,
                minos_resource: Either::Left(inner_resource),
            })?;
        }

        if permissions.is_empty() {
            return Err(Error::ActorNotAuthorized(actor.actor_id().to_string()));
        }

        Ok(permissions)
    }

    pub fn find_permission(
        &self,
        env_name: Option<&str>,
        actor: Actor,
        resource: Resource,
        permission: &Permission,
    ) -> MinosResult<()> {
        if let Some(resource_id) = resource.resource_id() {
            // if self
            //     .storage
            //     .attributed_resources()
            //     .contains_key(&Identifier(resource.resource_type().clone()))
            // {

            // }
        }

        // let permissions = self.authorize(env_name, actor, resource)?;
        // if !permissions.contains(permission) {
        //     return Err(Error::PermissionNotFound(permission.clone()));
        // }

        // Ok(())
        todo!()
    }

    pub fn find_permissions(
        &self,
        env_name: &str,
        actor: Actor,
        resource: Resource,
        permissions: &[Permission],
    ) -> MinosResult<()> {
        // let granted_permissions = self.authorize(env_name, actor, resource)?;
        // for permission in permissions {
        //     if !granted_permissions.contains(permission) {
        //         return Err(Error::PermissionNotFound(permission.clone()));
        //     }
        // }

        // Ok(())
        todo!()
    }
}
