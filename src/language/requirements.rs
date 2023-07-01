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

        let requirement = match inner_tokens[0] {
            Token::Assertion(inner) => Self::Assertion(Assertion::try_from(inner)?),
            Token::Negation(inner) => Self::Negation(Negation::try_from(inner)?),
            Token::Search(inner) => Self::Search(Search::try_from(inner)?),
            _=> unreachable!(),
        };

        Ok(requirement)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Attribute {
    Resource(ResourceAttribute),
    Actor(ActorAttribute),
}


#[derive(Debug, Clone, PartialEq)]
pub enum ComparableValue {
    Attribute(Attribute),
    String(&'static str),
    Array(Array<'static>),
    Identifier(Identifier<'static>),
}

#[derive(Debug, Clone, PartialEq, Ctor, Getters, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Assertion {
    left: Attribute,
    right: ComparableValue,
}

impl TryFrom<&Token<'_>> for Assertion {
    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Ctor, Getters, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Negation {
    left: Attribute,
    right: ComparableValue,
}

impl TryFrom<&Token<'_>> for Negation {
    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Ctor, Getters, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Search {
    left: Attribute,
    right: ComparableValue,
}

impl TryFrom<&Token<'_>> for Search {
    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        todo!()
    }
}
