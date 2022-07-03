///! Authorization library
pub mod authorization;
pub mod errors;
pub mod group;
pub mod resources;
pub mod user;
mod utils;

#[cfg(feature = "jwt")]
pub mod jwt;

#[cfg(test)]
mod test;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Status {
    Deleted,
    Disabled,
    Inactive,
    Active,
}

impl Status {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

impl From<usize> for Status {
    fn from(n: usize) -> Status {
        match n {
            3 => Status::Active,
            2 => Status::Inactive,
            1 => Status::Disabled,
            _ => Status::Deleted,
        }
    }
}

impl From<u8> for Status {
    fn from(n: u8) -> Status {
        match n {
            3 => Status::Active,
            2 => Status::Inactive,
            1 => Status::Disabled,
            _ => Status::Deleted,
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::Inactive
    }
}
