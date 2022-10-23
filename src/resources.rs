use crate::agent::Agent;
use crate::authorization::{Authorization, Policy};
use crate::errors::MinosError;
use crate::NonEmptyString;

pub trait Resource {
    fn id(&self) -> NonEmptyString;
    fn owner(&self) -> Option<NonEmptyString>;
    fn policies(&self) -> Vec<Policy>;
    fn resource_type(&self) -> Option<NonEmptyString>;

    #[cfg(feature = "resource_utils")]
    /// For custom-made rules implementation to generate an authorization (for more
    /// specific cases than those provided for by-group rules).
    fn authorize(&self, agent: &impl Agent) -> Result<Authorization, MinosError>;
}

#[cfg(feature = "resource_utils")]
pub trait AsResource<R: Resource> {
    type Error;
    fn as_resource(&self) -> Result<R, Self::Error>;
}
