use crate::core::actor::Actor;
use crate::core::authorization::{Authorization, Policy};
use crate::errors::MinosError;
use non_empty_string::NonEmptyString;

pub trait Resource {
    fn id(&self) -> NonEmptyString;
    fn owner(&self) -> Option<NonEmptyString>;
    fn policies(&self) -> Vec<Policy>;
    fn resource_type(&self) -> Option<NonEmptyString>;

    #[cfg(feature = "custom_authorization")]
    /// For custom-made rules implementation to generate an authorization (for more
    /// specific cases than those provided for policies rules).
    fn authorize<A: Actor>(&self, actor: &A) -> Result<Authorization, MinosError>;
}

pub trait AsResource<R: Resource> {
    type Error;
    fn as_resource(&mut self) -> Result<R, Self::Error>;
}
