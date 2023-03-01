/*use crate::core::actor::Actor;
use crate::core::authorization::{Authorization, Policy};
use crate::errors::MinosError;
use crate::prelude::ActorId;
use non_empty_string::NonEmptyString;

pub type ResourceId = NonEmptyString;

/// The use of ResourceType is a way to type the [Resource], who is an interface.
/// This identifier can differentiate each resource and simplify the storage and reuse of access policies.
pub type ResourceType = NonEmptyString;

pub trait Resource {
    fn id(&self) -> ResourceId;
    fn owner(&self) -> Option<ActorId>;
    fn policies(&self) -> Vec<Policy>;
    fn resource_type(&self) -> ResourceType;

    #[cfg(feature = "custom_authorization")]
    /// For custom-made rules implementation to generate an authorization (for more
    /// specific cases than those provided for policies rules).
    fn authorize<A: Actor>(&self, actor: &A) -> Result<Authorization, MinosError>;
}

pub trait AsResource<R: Resource> {
    type Error;
    fn as_resource(&mut self) -> Result<R, Self::Error>;
}
*/