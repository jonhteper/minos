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
use regex::{Captures, Match, Regex};
use toml::Table;
use versions::Versioning;
use crate::model::policies::Policies;



pub(crate) const MIN_COMPATIBLE_SYNTAX_VERSION: &str = "0.10";
pub(crate) const MAX_COMPATIBLE_SYNTAX_VERSION: &str = "0.10";
pub(crate) const FILE_POLICIES_REGEX: &str = r#"syntax_version\s*=\s*"([\d\.-]+)"|\[\[policies\]\]\n+(?:(?:resource_type|resource_id)\s*=\s*".+")\n+(?:(?:\[.+policies\.rules\]\])\n+(?:.+\n?)*\n*)*"#;
pub(crate) const VERSION_REGEX: &str = r#"syntax_version\s*=\s*"([\d\.-]+)""#;
pub(crate) const FILE_RULES_REGEX: &str = r#"(?:resource_type|resource_id)\s*=\s*".+"|(?:\[.+policies\.rules\]\])\n+permissions\s+=\s*\[\n*(?:\s*".+:\d+(?:ns|Ms|ms|s|m|h|d)",*\n*)+\]\n+(?:(?:actor|resource|environment)\.[a-zA-Z\d\._]+\s*(?:=|\$contains|>|<|>=|<=|!=)\s*.+\n*)+"#;

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
        let regex = Regex::new(FILE_POLICIES_REGEX)?;
        let captures = regex.captures_iter(self.file_content).collect::<Vec<Captures>>();
        let version = captures.get(0)
            .and_then(|c| c.get(1))
            .ok_or(MinosError::FormatWithoutVersion)?
            .as_str();
        let version = Versioning::new(version)
            .ok_or(MinosError::InvalidVersion)?;
        let min_valid_version = Versioning::new(MIN_COMPATIBLE_SYNTAX_VERSION).unwrap();
        let max_valid_version = Versioning::new(MAX_COMPATIBLE_SYNTAX_VERSION).unwrap();
        if version < min_valid_version || version > max_valid_version {
            return Err(MinosError::InvalidVersion);
        }

        let policies = captures.iter().skip(1).map(|c| {
            let policies = c.get(0)
                .ok_or(MinosError::InvalidPolicyFormat)?;

            Ok(policies.as_str())
        }).collect();

        policies
    }
}
