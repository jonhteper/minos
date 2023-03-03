use std::fs::Permissions;
use serde_json::Value;
use crate::errors::MinosError;

pub trait ParsePermissions {
    fn to_permissions(&self, vec: &[Value]) -> Result<Vec<Permission>, MinosError>;

    fn permissions_to_string(&self, permissions: &[Permission]) -> String;
}


#[derive(Debug, PartialEq, Eq, Clone, PartialOrd)]
pub struct Permission {
    /// expressive id
    name: String,
    /// in milliseconds
    duration: u128,
}

impl Permission {
    pub fn new(name: &str, duration: u128) -> Self {
        Self {name: name.to_owned(), duration}
    }


    pub fn required_msg(&self) -> &str {
        &self.name
    }
}
