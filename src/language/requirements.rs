use derived::Ctor;
use getset::{CopyGetters, Getters};

use crate::{
    errors::Error,
    parser::tokens::{ActorAttribute, Array},
};

use crate::parser::tokens::{Identifier, ResourceAttribute, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Requirement {
    Assertion(Assertion),
    Negation(Negation),
    Search(Search),
}

impl TryFrom<&Token<'_>> for Requirement {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_requirement().ok_or(Error::InvalidToken {
            expected: "Requirement",
            found: token.to_string(),
        })?;

        let requirement = match &inner_tokens[0] {
            Token::Assertion(inner) => Self::Assertion(Assertion::try_from(inner)?),
            Token::Negation(inner) => Self::Negation(Negation::try_from(inner)?),
            Token::Search(inner) => Self::Search(Search::try_from(inner)?),
            _ => unreachable!(),
        };

        Ok(requirement)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Attribute {
    Resource(ResourceAttribute),
    Actor(ActorAttribute),
}

impl TryFrom<&Token<'_>> for Attribute {
    type Error = Error;
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let attribute = match token {
            Token::ActorAttribute(attr) => Self::Actor(*attr),
            Token::ResourceAttribute(attr) => Self::Resource(*attr),
            _ => Err(Error::InvalidToken {
                expected: "ActorAttribute or ResourceAttribute",
                found: token.to_string(),
            })?,
        };

        Ok(attribute)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparableValue {
    Attribute(Attribute),
    String(&'static str),
    Array(Array<'static>),
    Identifier(Identifier<'static>),
}

impl TryFrom<&Token<'_>> for ComparableValue {
    type Error = Error;
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let value = match token {
            Token::ActorAttribute(attr) => Self::Attribute(Attribute::Actor(*attr)),
            Token::ResourceAttribute(attr) => Self::Attribute(Attribute::Resource(*attr)),
            Token::String(value) => Self::String(value),
            Token::Array(arr) => Self::Array(*arr),
            Token::Identifier(ident) => Self::Identifier(*ident),
            _ => Err(Error::InvalidToken {
                expected: "ActorAttribute, ResourceAttribute, String, Array or Identifier",
                found: token.to_string(),
            })?,
        };

        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Ctor, Getters, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Assertion {
    left: Attribute,
    right: ComparableValue,
}

impl TryFrom<&Vec<Token<'_>>> for Assertion {
    type Error = Error;
    fn try_from(token: &Vec<Token>) -> Result<Self, Self::Error> {
        let left = Attribute::try_from(token.first().ok_or(Error::MissingToken)?)?;
        let right = ComparableValue::try_from(token.get(2).ok_or(Error::MissingToken)?)?;

        Ok(Self { left, right })
    }
}

#[derive(Debug, Clone, PartialEq, Ctor, Getters, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Negation {
    left: Attribute,
    right: ComparableValue,
}

impl TryFrom<&Vec<Token<'_>>> for Negation {
    type Error = Error;
    fn try_from(token: &Vec<Token>) -> Result<Self, Self::Error> {
        let left = Attribute::try_from(token.first().ok_or(Error::MissingToken)?)?;
        let right = ComparableValue::try_from(token.get(2).ok_or(Error::MissingToken)?)?;

        Ok(Self { left, right })
    }
}

#[derive(Debug, Clone, PartialEq, Ctor, Getters, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Search {
    left: Attribute,
    right: ComparableValue,
}

impl TryFrom<&Vec<Token<'_>>> for Search {
    type Error = Error;
    fn try_from(token: &Vec<Token>) -> Result<Self, Self::Error> {
        let left = Attribute::try_from(token.first().ok_or(Error::MissingToken)?)?;
        let right = ComparableValue::try_from(token.get(2).ok_or(Error::MissingToken)?)?;

        Ok(Self { left, right })
    }
}
