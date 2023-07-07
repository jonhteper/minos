//! Authorization library based in Minos lang
//!

pub mod engine;
pub mod errors;
pub(crate) mod language;
pub mod parser;
#[cfg(test)]
mod tests;

pub use engine::{Container, Engine, Actor, Resource};
pub use errors::*;
pub use parser::MinosParser;
