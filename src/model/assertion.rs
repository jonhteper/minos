use crate::errors::MinosError;
use crate::model::attribute::{Attribute, Comparator};
use regex::Regex;
use serde_json::{Map, Value};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

pub const ASSERTION_REGEX_VALUE: &str = r"((actor|resource|environment)\.([a-zA-Z\d\._]+)) (>|=|!=|>=|<=|<) ((actor|resource|environment)\.([a-zA-Z\d\._]+)+)$";



/// Always must be true
#[derive(Clone, Debug)]
pub struct Assertion {
    left: Attribute,
    operator: Comparator,
    right: Attribute,
}

impl FromStr for Assertion {
    type Err = MinosError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let error_fn = MinosError::InvalidAssertionSyntax(str.to_string());
        let regex = Regex::from_str(ASSERTION_REGEX_VALUE)?;
        let captures = regex.captures(str).ok_or_else(|| error_fn.clone())?;

        let left = Attribute::from_str(captures.get(1).ok_or_else(|| error_fn.clone())?.as_str())?;

        let operator =
            Comparator::from_str(captures.get(4).ok_or_else(|| error_fn.clone())?.as_str())?;

        let right = Attribute::from_str(captures.get(5).ok_or(error_fn)?.as_str())?;

        Ok(Self {
            left,
            operator,
            right,
        })
    }
}

impl ToString for Assertion {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.left, self.operator, self.right)
    }
}

pub trait ToAssertions {
    fn to_assertions(&self) -> Result<Vec<Assertion>, MinosError>;
}
