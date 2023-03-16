use crate::errors::MinosError;
use regex::Regex;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde_json::Value as JsValue;

pub const ATTRIBUTE_PATH_REGEX_VALUE: &str = r"([a-z_]+)\.([a-zA-Z\d\._]+)";

#[derive(Clone, Debug)]
pub struct Attribute {
    /// object key
    parent: String,
    /// attributes, can't be 0 size
    children: Vec<String>,
}

impl Attribute {
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

impl FromStr for Attribute {
    type Err = MinosError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let err = MinosError::InvalidAssertionSyntax(str.to_string());

        let regex = Regex::from_str(ATTRIBUTE_PATH_REGEX_VALUE)?;
        let captures = regex.captures(str).ok_or_else(|| err.clone())?;
        let parent = captures
            .get(1)
            .ok_or_else(|| err.clone())?
            .as_str()
            .to_owned();
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

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", &self.parent, &self.children.join("."))
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Vec(Vec<Value>),
    Attribute(Attribute),
}

impl Value {
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Self::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_isize(&self) -> Option<i64> {
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
            _ => None,
        }
    }

    pub fn as_vec(&self) -> Option<&[Value]> {
        match self {
            Self::Vec(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_attribute(&self) -> Option<&Attribute> {
        match self {
            Self::Attribute(v) => Some(v),
            _ => None,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(v) => Display::fmt(v, f),
            Value::Integer(v) => Display::fmt(v, f),
            Value::Float(v) => Display::fmt(v, f),
            Value::String(v) => Display::fmt(v, f),
            Value::Vec(v) => write!(f, "{:?}", v),
            Value::Attribute(v) => Display::fmt(v, f),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Comparator {
    /// =
    Equal,
    /// !=
    Distinct,
    /// <
    Minor,
    /// >
    Major,
    /// <=
    MinorOrEqual,
    /// >=
    MajorOrEqual,
    /// $include
    Include,
}

impl Display for Comparator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let operator = match self {
            Comparator::Equal => "=",
            Comparator::Distinct => "!=",
            Comparator::Minor => "<",
            Comparator::Major => ">",
            Comparator::MinorOrEqual => "<=",
            Comparator::MajorOrEqual => ">=",
            Comparator::Include => "$include",
        };

        write!(f, "{operator}")
    }
}

impl FromStr for Comparator {
    type Err = MinosError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "=" => Ok(Self::Equal),
            "!=" => Ok(Self::Distinct),
            "<" => Ok(Self::Minor),
            ">" => Ok(Self::Major),
            "<=" => Ok(Self::MinorOrEqual),
            ">=" => Ok(Self::MajorOrEqual),
            "$include" => Ok(Self::Include),
            _ => Err(MinosError::InvalidAssertionChar(str.to_string())),
        }
    }
}