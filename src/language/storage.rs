use std::{collections::HashMap, sync::Arc};

use derived::Ctor;
use getset::Getters;

use crate::{
    parser::tokens::{Identifier, Token},
    Error,
};

use super::resource::{AttributedResource, Resource};

/// A collection of [Resource] and [AttributedResource].
#[derive(Debug, Clone, Ctor, Getters, PartialEq, Default)]
#[getset(get = "pub")]
pub struct Storage {
    resources: HashMap<Identifier, Resource>,
    attributed_resources: HashMap<(Identifier, Arc<str>), AttributedResource>,
}

impl Storage {
    pub fn merge(&mut self, storage: Storage) {
        for (_, resource) in storage.resources {
            self.add_resource(resource);
        }

        for (_, attributed_resource) in storage.attributed_resources {
            self.add_attributed_resource(attributed_resource);
        }
    }

    /// Add a [Resource] into [Storage]. if the resource's [Identifier] already exists,
    /// the two resources will be merged.
    pub fn add_resource(&mut self, resource: Resource) {
        if let Some(inner_resource) = self.resources.get_mut(resource.identifier()) {
            inner_resource.merge(resource);
            return;
        }

        self.resources.insert(resource.identifier().clone(), resource);
    }

    /// Add a [AttributedResource] into [Storage]. if the resource's [Identifier] already exists,
    /// the two resources will be merged.
    pub fn add_attributed_resource(&mut self, resource: AttributedResource) {
        if let Some(inner_resource) = self
            .attributed_resources
            .get_mut(&(resource.identifier().clone(), resource.id().clone()))
        {
            inner_resource.merge(resource);
            return;
        }

        self.attributed_resources
            .insert((resource.identifier().clone(), resource.id().clone()), resource);
    }

    pub fn policies_len(&self) -> usize {
        let mut len = 0;
        for resource in self.resources().values() {
            len += resource.policies_len();
        }

        for attr_resource in self.attributed_resources().values() {
            len += attr_resource.policies_len();
        }

        len
    }
}

impl TryFrom<Token> for Storage {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_file().ok_or(Error::InvalidToken {
            expected: "File",
            found: token.to_string(),
        })?;

        let mut storage = Storage::default();
        for inner_token in &inner_tokens[1..inner_tokens.len() - 1] {
            match inner_token {
                Token::Resource(_) => {
                    storage.add_resource(Resource::try_from(inner_token)?);
                }
                Token::AttributedResource(_) => {
                    storage.add_attributed_resource(AttributedResource::try_from(inner_token)?);
                }
                _ => unreachable!(),
            }
        }

        Ok(storage)
    }
}
