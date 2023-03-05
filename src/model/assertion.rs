use crate::errors::MinosError;
use crate::model::attribute::KeyPath;
use regex::Regex;
use serde_json::{Map, Value};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

pub const ASSERTION_REGEX_VALUE: &str = r"((actor|resource|environment)\.([a-zA-Z\d\._]+)) (>|=|!=|>=|<=|<) ((actor|resource|environment)\.([a-zA-Z\d\._]+)+)$";

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

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let operator = match self {
            Operator::Equal => "=",
            Operator::Distinct => "!=",
            Operator::Minor => "<",
            Operator::Major => ">",
            Operator::MinorOrEqual => "<=",
            Operator::MajorOrEqual => ">=",
        };

        write!(f, "{operator}")
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
            _ => Err(MinosError::InvalidAssertionChar(str.to_string())),
        }
    }
}

/// Always must be true
#[derive(Clone, Debug)]
pub struct Assertion {
    left: KeyPath,
    operator: Operator,
    right: KeyPath,
}

impl FromStr for Assertion {
    type Err = MinosError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let error_fn = MinosError::InvalidAssertionSyntax(str.to_string());
        let regex = Regex::from_str(ASSERTION_REGEX_VALUE)?;
        let captures = regex.captures(str).ok_or_else(||error_fn.clone())?;

        let left = KeyPath::from_str(captures.get(1).ok_or_else(||error_fn.clone())?.as_str())?;

        let operator = Operator::from_str(captures.get(4).ok_or_else(||error_fn.clone())?.as_str())?;

        let right = KeyPath::from_str(captures.get(5).ok_or(error_fn)?.as_str())?;

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
