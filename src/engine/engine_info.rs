use std::{collections::HashMap, ops::Deref, sync::Arc};

use derived::Ctor;

use crate::{
    language::{environment::Environment, policy::Policy, storage::Storage},
    parser::tokens::Identifier,
};

#[derive(Debug, Copy, Clone, Ctor)]
pub struct EngineInfo<'a> {
    storage: &'a Storage,
}

impl<'a> EngineInfo<'a> {
    pub fn policies_len(&self, criteria: Option<Criteria>) -> usize {
        if criteria.is_none() {
            return self.storage.policies_len();
        }

        match criteria.unwrap() {
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

    fn collect_policies(environments: &HashMap<Identifier, Environment>) -> Vec<&Policy> {
        let mut resource_policies = vec![];
        for env in environments.values() {
            for policy in env.policies() {
                resource_policies.push(policy);
            }
        }

        resource_policies
    }

    pub fn policies(&self, criteria: Criteria) -> Vec<&Policy> {
        match criteria {
            Criteria::ResourceType(ty) => {
                let r_type = Identifier::from(ty);
                match self.storage.resources().get(&r_type) {
                    Some(resource) => Self::collect_policies(resource.environments()),
                    None => vec![],
                }
            }
            Criteria::ResourceId(id) => {
                for attr_resource in self.storage.attributed_resources().values() {
                    if attr_resource.id().deref() == id {
                        return Self::collect_policies(attr_resource.environments());
                    }
                }

                vec![]
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