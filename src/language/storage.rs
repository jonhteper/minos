use std::collections::HashMap;

use derived::Ctor;
use getset::Getters;

use crate::parser::tokens::Identifier;

use super::resource::{AttributedResource, Resource};

/// A collection of [Resource] and [AttributedResource].
#[derive(Debug, Clone, Ctor, Getters, PartialEq, Default)]
#[getset(get = "pub")]
pub struct Storage {
    resources: HashMap<Identifier, Resource>,
    attributed_resources: HashMap<Identifier, AttributedResource>,
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

        self.resources
            .insert(resource.identifier().clone(), resource);
    }

    /// Add a [AttributedResource] into [Storage]. if the resource's [Identifier] already exists,
    /// the two resources will be merged.
    pub fn add_attributed_resource(&mut self, resource: AttributedResource) {
        if let Some(inner_resource) = self.attributed_resources.get_mut(resource.identifier()) {
            if inner_resource.id() == resource.id() {
                inner_resource.merge(resource);
                return;
            }
        }

        self.attributed_resources
            .insert(resource.identifier().clone(), resource);
    }
}
