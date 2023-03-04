use std::fmt::{Display, Formatter};
use crate::errors::MinosError;
use regex::Regex;
use std::str::FromStr;

pub const ATTRIBUTE_PATH_REGEX_VALUE: &str = r"([a-z_]+)\.([a-zA-Z\d\._]+)";

#[derive(Clone, Debug)]
pub struct AttributePath {
    /// object key
    parent: String,
    /// attributes, can't be 0 size
    children: Vec<String>,
}

impl AttributePath {
    pub fn new(parent: String, children: Vec<String>) -> Self {
        Self { parent, children }
    }
    pub fn parent(&self) -> &str {
        &self.parent
    }
    pub fn children(&self) -> &[String] {
        &self.children
    }
}

impl FromStr for AttributePath {
    type Err = MinosError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let err = MinosError::InvalidAssertionSyntax(str.to_string());

        let regex = Regex::from_str(ATTRIBUTE_PATH_REGEX_VALUE)?;
        let captures = regex.captures(str).ok_or(err.clone())?;
        let parent = captures.get(1).ok_or(err.clone())?.as_str().to_owned();
        let children = captures
            .get(2)
            .ok_or(err)?
            .as_str()
            .split(".")
            .map(|str| str.to_owned())
            .collect();

        Ok(Self { parent, children })
    }
}

impl Display for AttributePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", &self.parent, &self.children.join("."))
    }
}

