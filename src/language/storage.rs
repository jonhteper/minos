use std::collections::HashMap;

use derived::Ctor;
use getset::Getters;

use crate::parser::tokens::Identifier;

use super::resource::{Resource, AttributedResource};

/// A collection of policies
#[derive(Debug, Clone, Ctor, Getters, PartialEq, Default)]
#[getset(get = "pub")]
pub struct Storage{
    resources: HashMap<Identifier, Resource>,
    attributed_resources: HashMap<&'static str, AttributedResource>
}

impl Storage {
    pub fn extend_with(&mut self, storage: Storage) {
        todo!()
    }
}
