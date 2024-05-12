use std::{collections::HashMap, sync::Arc};

use crate::{
    language::resource::{AttributedResource, Resource},
    parser::tokens::Identifier,
};

use super::to_text_repr::ToTextRepr;

impl ToTextRepr for HashMap<Identifier, Resource> {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let mut resources_str = String::new();
        for resource in self.values() {
            resources_str.push_str(&resource.to_text_repr());
        }

        resources_str
    }
}

impl ToTextRepr for Resource {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let identifier = &self.identifier().0;
        let envs = self.environments().to_text_repr();

        format!("resource {identifier} {{\n{envs}}}\n\n")
    }
}

impl ToTextRepr for HashMap<(Identifier, Arc<str>), AttributedResource> {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let mut attr_resources_str = String::new();
        for attr_resource in self.values() {
            attr_resources_str.push_str(&attr_resource.to_text_repr());
        }

        attr_resources_str
    }
}

impl ToTextRepr for AttributedResource {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let identifier = &self.identifier().0;
        let resource_id = format!("{:?}", self.id());
        let envs = self.environments().to_text_repr();

        format!("resource {identifier} {{\nid = {resource_id};\n\n {envs}}}\n\n")
    }
}
