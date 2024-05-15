use std::{ops::Deref, sync::Arc};

use derived::Ctor;

use crate::{
    language::{environment::Environment, storage::Storage},
    parser::tokens::Identifier,
};

#[derive(Debug, Copy, Clone, Ctor)]
pub struct EngineInfo<'a> {
    storage: &'a Storage,
}

impl<'a> EngineInfo<'a> {
    /// Return the number of policies of selected resource. Return 0 if no resource is found.
    pub fn policies_len(&self, search_criteria: Option<Criteria>) -> usize {
        if search_criteria.is_none() {
            return self.storage.policies_len();
        }

        match search_criteria.unwrap() {
            Criteria::ResourceType(ty) => {
                let r_type = Identifier::from(ty);
                match self.storage.resources().get(&r_type) {
                    Some(resource) => resource.policies_len(),
                    None => 0,
                }
            }
            Criteria::ResourceId(id) => {
                for attr_resource in self.storage.attributed_resources().values() {
                    if attr_resource.id().deref() == id {
                        return attr_resource.policies_len();
                    }
                }

                0
            }
        }
    }

    /// Returns an iterator with the resource's [`Environment`]s, if it exists.
    ///
    /// # Parameters
    ///
    /// * `search_criteria` - A `Criteria` enum that specifies the search criteria.
    ///
    /// # Returns
    ///
    /// * `Option<impl Iterator<Item = &Environment> + '_>` - An `Option` that contains an iterator with the resource's environments if it exists.
    ///   If the resource is not found, it returns `None`.
    ///
    pub fn environments(
        &self,
        search_criteria: Criteria,
    ) -> Option<impl Iterator<Item = &Environment> + '_> {
        match search_criteria {
            Criteria::ResourceType(ty) => {
                let r_type = Identifier::from(ty);
                self.storage
                    .resources()
                    .get(&r_type)
                    .map(|r| r.environments().values())
            }
            Criteria::ResourceId(id) => {
                for attr_resource in self.storage.attributed_resources().values() {
                    if attr_resource.id().deref() == id {
                        return Some(attr_resource.environments().values());
                    }
                }

                None
            }
        }
    }

    pub fn resources_len(&self) -> usize {
        self.storage.resources().len()
    }

    pub fn attr_resources_len(&self) -> usize {
        self.storage.attributed_resources().len()
    }

    pub fn resources_names(&self) -> Vec<Identifier> {
        self.storage.resources().keys().cloned().collect()
    }

    pub fn attr_resources_names(&self) -> Vec<(Identifier, Arc<str>)> {
        self.storage.attributed_resources().keys().cloned().collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Criteria<'a> {
    ResourceType(&'a str),
    ResourceId(&'a str),
}
