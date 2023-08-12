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

use super::{Actor, ActorRepr, Permissions, Resource, ResourceRepr};

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
    pub permission: String,
}

#[derive(Debug)]
pub struct FindPermissionsRequest<'a> {
    pub env_name: Option<&'a str>,
    pub actor: &'a Actor,
    pub resource: &'a Resource,
    pub permissions: Vec<String>,
}

struct InternalAuthorizeRequest<'a> {
    pub env_name: Option<&'a str>,
    pub actor: &'a ActorRepr,
    pub resource: &'a ResourceRepr,
    pub minos_resource: Either<&'a InternalResource, &'a AttributedResource>,
}

struct InternalFindPermissionRequest<'a> {
    pub env_name: Option<&'a str>,
    pub actor: &'a ActorRepr,
    pub resource: &'a ResourceRepr,
    pub minos_resource: Either<&'a InternalResource, &'a AttributedResource>,
    pub permission: &'a str,
}

#[derive(Debug, Clone, Ctor)]
pub struct Engine<'s> {
    storage: &'s Storage,
}

impl<'s> Engine<'s> {
    fn append_permissions(
        permissions: &mut Permissions,
        environment: &Environment,
        actor: &ActorRepr,
        resource: &ResourceRepr,
    ) {
        for policy in environment.policies() {
            if let Some(inner_permissions) = policy.apply(actor, resource) {
                permissions.append_permissions(inner_permissions);
            }
        }
    }

    fn find_attributed_resource(
        &self,
        resource_id: Arc<str>,
        resource: &ResourceRepr,
    ) -> Option<&AttributedResource> {
        self.storage
            .attributed_resources()
            .get(&(resource.type_().into(), resource_id))
    }

    fn authorize_attributed_resource(
        &self,
        request: InternalAuthorizeRequest,
    ) -> MinosResult<Permissions> {
        let actor = request.actor;
        let resource = request.resource;
        let attr_resource = request.minos_resource.unwrap_right();

        let mut permissions = Permissions::new();
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

    fn authorize_resource(
        &self,
        request: InternalAuthorizeRequest
    ) -> MinosResult<Permissions> {
        let inner_resource = request.minos_resource.unwrap_left();
        let mut permissions = Permissions::new();

        if let Some(default_env) = inner_resource.default_environment() {
            Self::append_permissions(&mut permissions, default_env, request.actor, request.resource);
        }

        if let Some(env_name) = request.env_name {
            let env = inner_resource
                .get_environment(env_name)
                .ok_or(Error::EnvironmentNotFound(env_name.to_string()))?;
            Self::append_permissions(&mut permissions, env, request.actor, request.resource);
        }

        if permissions.is_empty() {
            return Err(Error::ActorNotAuthorized(request.actor.id().to_string()));
        }

        Ok(permissions)
    }

    /// Return the granted [Permissions] if the [Actor] is authorized.
    /// This function fails if:
    /// * The [Actor] is not authorized.
    /// * Tha resource not exist into the [Storage].
    /// * The environment's name not exist into the [Storage].
    pub fn authorize(&self, request: AuthorizeRequest) -> MinosResult<Permissions> {
        let env_name = request.env_name;
        let actor = &ActorRepr::from(request.actor);
        let resource = &ResourceRepr::from(request.resource);

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
            return self.authorize_resource(
                InternalAuthorizeRequest {
                    env_name,
                    actor,
                    resource,
                    minos_resource: Either::Left(inner_resource),
                }
            );
        }

        Err(Error::ResourceNotFound(resource.type_.to_string()))
    }

    fn is_permission_in_env(
        environment: &Environment,
        actor: &ActorRepr,
        resource: &ResourceRepr,
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
    ) -> MinosResult<bool> {
        let actor = request.actor;
        let resource = request.resource;
        let attr_resource = request.minos_resource.unwrap_right();
        let permission = &Permission::from(request.permission);
        if let Some(default_env) = attr_resource.default_environment() {
            return Ok(Self::is_permission_in_env(
                default_env,
                actor,
                resource,
                permission,
            ));
        }

        if let Some(env_name) = request.env_name {
            let env = attr_resource
                .get_environment(env_name)
                .ok_or(Error::EnvironmentNotFound(env_name.to_string()))?;
            return Ok(Self::is_permission_in_env(env, actor, resource, permission));
        }

        Ok(false)
    }

    fn find_permission_in_resource(&self, request: InternalFindPermissionRequest) -> MinosResult<bool> {
        let inner_resource = request.minos_resource.unwrap_left();
        let actor = request.actor;
        let resource = request.resource;
        let permission = &Permission::from(request.permission);

        if let Some(default_env) = inner_resource.default_environment() {
            return Ok(Self::is_permission_in_env(
                default_env,
                actor,
                resource,
                permission,
            ));
        }

        if let Some(env_name) = request.env_name {
            let env = inner_resource
                .get_environment(env_name)
                .ok_or(Error::EnvironmentNotFound(env_name.to_string()))?;

            return Ok(Self::is_permission_in_env(
                env,
                request.actor,
                request.resource,
                permission,
            ));
        }

        Ok(false)
    }

    /// Check if the actor has the selected permission over the resource.
    ///
    /// This method fails if:    
    /// * Tha resource not exist into the [Storage].
    /// * The environment's name not exist into the [Storage].
    pub fn actor_has_permission(&self, request: FindPermissionRequest) -> MinosResult<bool> {
        let env_name = request.env_name;
        let actor = &ActorRepr::from(request.actor);
        let resource = &ResourceRepr::from(request.resource);
        let permission = request.permission;

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

        Err(Error::ResourceNotFound(resource.type_.to_string()))
    }

    /// Check if the user has the selected permissions over the resource. 
    /// If not all permissions granted, this functions returns false.
    /// 
    /// This method fails if:
    /// * Tha resource not exist into the [Storage].
    /// * The environment's name not exist into the [Storage].
    /// 
    /// WARNING: this function search permissions individually, with performance penalties for
    /// long permissions list. In this case use [`Engine::authorize`]
    pub fn actor_has_permissions(&self, request: FindPermissionsRequest) -> MinosResult<bool> {
        let env_name = request.env_name;
        let actor = &ActorRepr::from(request.actor);
        let resource = &ResourceRepr::from(request.resource);
        let permissions = request.permissions;
        let mut n_permissions_granted = 0;

        if let Some(resource_id) = resource.id() {
            if let Some(attr_resource) = self.find_attributed_resource(resource_id.clone(), resource) {
                for permission in &permissions {
                    if self.find_permission_in_attributed_resource(InternalFindPermissionRequest {
                        env_name,
                        actor,
                        resource,
                        minos_resource: Either::Right(attr_resource),
                        permission: &permission,
                    })? {
                        n_permissions_granted += 1;
                    }
                }

                return Ok(n_permissions_granted == permissions.len());
            }
        }

        if let Some(inner_resource) = self.storage.resources().get(&resource.type_().into()) {
            for permission in &permissions {
                if self.find_permission_in_resource(InternalFindPermissionRequest {
                    env_name,
                    actor,
                    resource,
                    minos_resource: Either::Left(inner_resource),
                    permission: &permission,
                })? {
                    n_permissions_granted += 1;
                }

                return Ok(n_permissions_granted == permissions.len());
            }
        }

        Err(Error::ResourceNotFound(resource.type_.to_string()))
    }

    pub fn policies_len(&self) -> usize {
        self.storage.policies_len()
    }
}
