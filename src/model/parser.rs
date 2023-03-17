use std::collections::HashMap;
use crate::errors::MinosError;
use crate::model::actor::{Actor, ToActor};
use crate::model::assertion::{Assertion, ToAssertions};
use crate::model::attribute;
use crate::model::attribute::Attribute;
use crate::model::permission::{Permission, ToPermissions};
use fundu::parse_duration;
use rayon::iter::ParallelIterator;
use rayon::iter::IndexedParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use serde_json::Value::Bool;
use serde_json::{Map, Value};
use std::ops::Deref;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::{Captures, Match, Regex};
use toml::Table;
use versions::Versioning;
use crate::model::policies::Policies;

lazy_static! {
    static ref MIN_COMPATIBLE_SYNTAX_VERSION: Versioning = Versioning::new("0.10").unwrap();
    static ref MAX_COMPATIBLE_SYNTAX_VERSION: Versioning = Versioning::new("0.10").unwrap();
    static ref FILE_POLICIES_REGEX: Regex = Regex::new(r#"syntax_version\s*=\s*"([\d\.-]+)"|\[\[policies\]\]\n+(?:(?:resource_type|resource_id)\s*=\s*".+")\n+(?:(?:\[.+policies\.rules\]\])\n+(?:.+\n?)*\n*)*"#).unwrap();
    static ref FILE_RULES_REGEX: Regex = Regex::new(r#"(resource_type|resource_id)\s*=\s*"(.+)"|(?:\[.+policies\.rules\]\])\n+permissions\s+=\s*\[\n*(?:\s*".+:\d+(?:ns|Ms|ms|s|m|h|d)",*\n*)+\]\n+(?:(?:actor|resource|environment)\.[a-zA-Z\d\._]+\s*(?:=|\$contains|>|<|>=|<=|!=)\s*.+\n*)+"#).unwrap();
}



pub struct FileParser<'a> {
    file_content: &'a str,
    policies_by_resource_type: HashMap<String, Policies>,
    policies_by_resource_id: HashMap<String, Policies>,
}

impl<'a> FileParser<'a> {
    pub fn new(file_content: &'a str) -> Self {
        Self {
            file_content,
            policies_by_resource_type: HashMap::new(),
            policies_by_resource_id: HashMap::new(),
        }
    }

    pub fn obtain_policies(&self) -> Result<Vec<&str>, MinosError> {
        let regex = &FILE_POLICIES_REGEX;
        let captures = regex.captures_iter(self.file_content).collect::<Vec<Captures>>();
        let version = captures.get(0)
            .and_then(|c| c.get(1))
            .ok_or(MinosError::FormatWithoutVersion)?
            .as_str();
        let version = Versioning::new(version)
            .ok_or(MinosError::InvalidVersion)?;
        if version < *MIN_COMPATIBLE_SYNTAX_VERSION || version > *MAX_COMPATIBLE_SYNTAX_VERSION {
            return Err(MinosError::InvalidVersion);
        }

        let policies = captures.iter().skip(1).map(|c| {
            let policies = c.get(0)
                .ok_or(MinosError::InvalidPolicyFormat)?;

            Ok(policies.as_str())
        }).collect();

        policies
    }

    pub fn obtain_rules_from_policy(&self, policy: &str) -> Result<HashMap<ResourceIdentifier<'_>, &str>, MinosError> {
        let captures = FILE_RULES_REGEX.captures_iter(policy).collect::<Vec<Captures>>();
        let resource = captures.get(0).unwrap();
        let identifier_key = resource.get(1).unwrap().as_str();
        let identifier_value  = resource.get(2).unwrap().as_str();
        let resource_identifier = match identifier_key {
            "resource_id" => ResourceIdentifier::ResourceId(identifier_value),
            "resource_type" => ResourceIdentifier::ResourceType(identifier_value),
            _ => unreachable!(),
        };

        dbg!(resource_identifier);

        Ok(HashMap::new())
    }
}

#[derive(Debug)]
pub enum ResourceIdentifier<'a> {
    ResourceId(&'a str),
    ResourceType(&'a str),
}