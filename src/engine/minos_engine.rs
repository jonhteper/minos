use std::sync::Arc;

use derived::Ctor;
use either::Either;

use crate::{
    errors::{Error, MinosResult},
    language::{
        environment::Environment, policy::Permission, resource::AttributedResource,
        resource::Resource as InternalResource, storage::Storage,
    },
};

use super::{Actor, Resource};

#[derive(Debug)]
pub struct AuthorizeRequest<'a> {
    pub env_name: Option<&'a str>,
    pub actor: &'a Actor,
    pub resource: &'a Resource,
}

#[derive(Debug)]
pub struct FindPermissionRequest<'a> {
    pub env_name: Option<&'a str>,
    pub actor: &'a Actor,
    pub resource: &'a Resource,
    pub permission: Permission,
}

#[derive(Debug)]
pub struct FindPermissionsRequest<'a> {
    pub env_name: Option<&'a str>,
    pub actor: &'a Actor,
    pub resource: &'a Resource,
    pub permissions: &'a [Permission],
}

struct InternalAuthorizeRequest<'a> {
    pub env_name: Option<&'a str>,
    pub actor: &'a Actor,
    pub resource: &'a Resource,
    pub minos_resource: Either<&'a InternalResource, &'a AttributedResource>,
}

struct InternalFindPermissionRequest<'a> {
    pub env_name: Option<&'a str>,
    pub actor: &'a Actor,
    pub resource: &'a Resource,
    pub minos_resource: Either<&'a InternalResource, &'a AttributedResource>,
    pub permission: &'a Permission,
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
            .get(&(resource.type_().into(), resource_id))
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
            return Err(Error::ActorNotAuthorized(actor.id().to_string()));
        }

        Ok(permissions)
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
        } = request;
        let mut permissions = vec![];

        if let Some(resource_id) = resource.id() {
            if let Some(attr_resource) = self.find_attributed_resource(resource_id.clone(), resource) {
                return self.authorize_attributed_resource(InternalAuthorizeRequest {
                    env_name,
                    actor,
                    resource,
                    minos_resource: Either::Right(attr_resource),
                });
            }
        }

        if let Some(inner_resource) = self.storage.resources().get(&resource.type_().into()) {
            permissions = self.authorize_resource(InternalAuthorizeRequest {
                env_name,
                actor,
                resource,
                minos_resource: Either::Left(inner_resource),
            })?;
        }

        if permissions.is_empty() {
            return Err(Error::ActorNotAuthorized(actor.id().to_string()));
        }

        Ok(permissions)
    }

    fn is_permission_in_env(
        environment: &Environment,
        actor: &Actor,
        resource: &Resource,
        permission: &Permission,
    ) -> bool {
        for policy in environment.policies() {
            if policy.actor_has_permission(actor, resource, permission) {
                return true;
            }
        }

        false
    }

    fn find_permission_in_attributed_resource(
        &self,
        request: InternalFindPermissionRequest,
    ) -> MinosResult<()> {
        let actor = request.actor;
        let resource = request.resource;
        let attr_resource = request.minos_resource.unwrap_right();
        let permission = request.permission;
        if let Some(default_env) = attr_resource.default_environment() {
            if Self::is_permission_in_env(default_env, actor, resource, permission) {
                return Ok(());
            }
        }

        if let Some(env_name) = request.env_name {
            let env = attr_resource
                .get_environment(env_name)
                .ok_or(Error::EnvironmentNotFound(env_name.to_string()))?;
            if Self::is_permission_in_env(env, actor, resource, permission) {
                return Ok(());
            }
        }

        Err(Error::ActorNotAuthorized(actor.id().to_string()))
    }

    fn find_permission_in_resource(&self, request: InternalFindPermissionRequest) -> MinosResult<()> {
        let inner_resource = request.minos_resource.unwrap_left();
        let permission = request.permission;
        let actor = request.actor;
        let resource = request.resource;

        if let Some(default_env) = inner_resource.default_environment() {
            if Self::is_permission_in_env(default_env, actor, resource, permission) {
                return Ok(());
            }
        }

        if let Some(env_name) = request.env_name {
            let env = inner_resource
                .get_environment(env_name)
                .ok_or(Error::EnvironmentNotFound(env_name.to_string()))?;
            if Self::is_permission_in_env(env, request.actor, request.resource, permission) {
                return Ok(());
            }
        }

        Err(Error::ActorNotAuthorized(actor.id().to_string()))
    }

    pub fn find_permission(&self, request: FindPermissionRequest) -> MinosResult<()> {
        let FindPermissionRequest {
            env_name,
            actor,
            resource,
            permission,
        } = request;

        if let Some(resource_id) = resource.id() {
            if let Some(attr_resource) = self.find_attributed_resource(resource_id.clone(), resource) {
                return self.find_permission_in_attributed_resource(InternalFindPermissionRequest {
                    env_name,
                    actor,
                    resource,
                    minos_resource: Either::Right(attr_resource),
                    permission: &permission,
                });
            }
        }

        if let Some(inner_resource) = self.storage.resources().get(&resource.type_().into()) {
            return self.find_permission_in_resource(InternalFindPermissionRequest {
                env_name,
                actor,
                resource,
                minos_resource: Either::Left(inner_resource),
                permission: &permission,
            });
        }

        Err(Error::ActorNotAuthorized(actor.id().to_string()))
    }

    /// WARNING: this function implements [Engine::find_permission] inside, so
    /// the performance is not the best.
    pub fn find_permissions(&self, request: FindPermissionsRequest) -> MinosResult<()> {
        let FindPermissionsRequest {
            env_name,
            actor,
            resource,
            permissions,
        } = request;

        for permission in permissions {
            if self
                .find_permission(FindPermissionRequest {
                    env_name,
                    actor,
                    resource,
                    permission: permission.clone(),
                })
                .is_err()
            {
                return Err(Error::ActorNotAuthorized(actor.id().to_string()));
            }
        }

        Ok(())
    }

    pub fn policies_len(&self) -> usize {
        self.storage.policies_len()
    }
}
