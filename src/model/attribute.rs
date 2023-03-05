use crate::errors::MinosError;
use regex::Regex;
use std::fmt;
use std::fmt::{Display, Formatter};
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
        let captures = regex.captures(str).ok_or_else(||err.clone())?;
        let parent = captures.get(1).ok_or_else(|| err.clone())?.as_str().to_owned();
        let children = captures
            .get(2)
            .ok_or(err)?
            .as_str()
            .split('.')
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

#[derive(Debug, Clone)]
pub enum Value<'str> {
    Bool(bool),
    Integer(isize),
    Float(f64),
    String(String),
    Str(&'str str),
}

impl Value<'_> {
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Self::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_isize(&self) -> Option<isize> {
        match *self {
            Self::Integer(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Self::Float(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(v) => Some(v),
            Self::Str(v) => Some(v),
            _ => None,
        }
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(v) => Display::fmt(v, f),
            Value::Integer(v) => Display::fmt(v, f),
            Value::Float(v) => Display::fmt(v, f),
            Value::String(v) => Display::fmt(v, f),
            Value::Str(v) => Display::fmt(v, f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attribute<'a>(AttributePath, Value<'a>);
