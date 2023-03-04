use std::str::FromStr;
use regex::Regex;
use crate::errors::MinosError;

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
        let err = || MinosError::InvalidAssertionSyntax(str.to_string());

        let regex = Regex::from_str(ATTRIBUTE_PATH_REGEX_VALUE)?;
        let captures = regex.captures(str)
            .ok_or_else(err)?;
        let parent = captures.get(1).ok_or_else(err)?.as_str().to_owned();
        let children = captures.get(2)
            .ok_or_else(err)?
            .as_str()
            .split(".")
            .map(|str| str.to_owned())
            .collect();

        Ok(Self {
            parent,
            children
        })
    }
}

impl ToString for AttributePath {
    fn to_string(&self) -> String {
        format!("{}.{}", &self.parent, &self.children.join("."))
    }
}
