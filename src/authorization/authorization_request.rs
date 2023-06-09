use super::{Actor, Resource};

pub struct AuthorizationRequest {
    environment: String,
    resource: Box<dyn Resource>,
    actor: Box<dyn Actor>,
}