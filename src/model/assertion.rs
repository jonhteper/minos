use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use regex::Regex;
use serde_json::{Map, Value};
use crate::errors::MinosError;
use crate::model::attribute_path::AttributePath;

pub const ASSERTION_REGEX_VALUE :&str = r"((actor|resource|environment)\.([a-zA-Z\d\.\_]+)) (>|=|!=|>=|<=|<) ((actor|resource|environment)\.([a-zA-Z\d\.\_]+)+)$";

#[derive(Clone, Copy, Debug)]
enum Operator {
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
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Equal => "=".to_string(),
            Operator::Distinct => "!=".to_string(),
            Operator::Minor => "<".to_string(),
            Operator::Major => ">".to_string(),
            Operator::MinorOrEqual => "<=".to_string(),
            Operator::MajorOrEqual => ">=".to_string(),
        }
    }
}

impl FromStr for Operator {
    type Err = MinosError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "=" => Ok(Self::Equal),
            "!=" => Ok(Self::Distinct),
            "<" => Ok(Self::Minor),
            ">" => Ok(Self::Major),
            "<=" => Ok(Self::MinorOrEqual),
            ">=" => Ok(Self::MajorOrEqual),
            _ => Err(MinosError::InvalidAssertionChar(str.to_string()))
        }
    }
}


/// Always must be true
#[derive(Clone, Debug)]
pub struct  Assertion {
    left: AttributePath,
    operator: Operator,
    right: AttributePath,
}

impl FromStr for Assertion {
    type Err = MinosError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let regex = Regex::from_str(ASSERTION_REGEX_VALUE)?;
        let captures = regex.captures(str)
            .ok_or_else(||MinosError::InvalidAssertionSyntax(str.to_string()))?;


        Err(MinosError::InvalidAssertionSyntax(str.to_string()))
    }
}


pub trait ToAssertions {
    fn to_assertions(&self)-> Result<Vec<Assertion>, MinosError>;
}