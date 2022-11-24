//! Authorization library
//!

pub mod actor;
pub mod authorization;
pub mod authorization_builder;
pub mod errors;
pub mod prelude;
pub mod resources;
pub mod utils;

#[cfg(feature = "jwt")]
pub mod jwt;

#[cfg(feature = "toml_storage")]
pub mod toml;

#[cfg(feature = "manifest")]
pub mod resource_manifest;

#[cfg(test)]
mod test;
