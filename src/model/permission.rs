use crate::errors::MinosError;

pub trait ToPermissions {
    fn to_permissions(&self) -> Result<Vec<Permission>, MinosError>;
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
        Self {
            name: name.to_owned(),
            duration,
        }
    }

    pub fn required_msg(&self) -> &str {
        &self.name
    }
}
