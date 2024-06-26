use std::{collections::HashMap, sync::Arc};

use crate::{
    language::{
        environment::Environment,
        resource::{AttributedResource, Resource},
    },
    parser::tokens::Identifier,
    text_repr::environment_text_repr::EnvironmentsFormatter,
};

use super::to_text_repr::ToTextRepr;

impl ToTextRepr for HashMap<Identifier, Resource> {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let mut resources_str = String::new();
        for (index, resource) in self.values().enumerate() {
            resources_str.push_str(&resource.to_text_repr());

            if index < self.len() - 1 {
                resources_str.push('\n');
            }
        }

        resources_str
    }
}

impl ToTextRepr for Resource {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let identifier = &self.identifier().0;
        let envs_list = self.environments().values();
        let envs = EnvironmentsFormatter::new(envs_list).to_text_repr();

        format!("resource {identifier} {{\n{envs}}}\n")
    }
}

impl ToTextRepr for HashMap<(Identifier, Arc<str>), AttributedResource> {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let mut attr_resources_str = String::new();
        if !self.is_empty() {
            attr_resources_str.push('\n'); // add an empty line to separate the resources from the attributed resources
        }

        for (index, attr_resource) in self.values().enumerate() {
            attr_resources_str.push_str(&attr_resource.to_text_repr());

            if index < self.len() - 1 {
                attr_resources_str.push('\n');
            }
        }

        attr_resources_str
    }
}

impl ToTextRepr for AttributedResource {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let identifier = &self.identifier().0;
        let resource_id = format!("{:?}", self.id());
        let resource_id_ind = Environment::INDENTATION;
        let envs_list = self.environments().values();
        let envs = EnvironmentsFormatter::new(envs_list).to_text_repr();

        format!("resource {identifier} {{\n{resource_id_ind}id = {resource_id};\n\n{envs}}}\n")
    }
}
