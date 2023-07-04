use std::sync::Arc;

use derived::Ctor;
use getset::{CopyGetters, Getters};

use crate::{
    engine::{Actor, Resource},
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

impl Requirement {
    pub fn apply(&self, actor: &Actor, resource: &Resource) -> Option<bool> {
        match self {
            Requirement::Assertion(assertion) => assertion.apply(actor, resource),
            Requirement::Negation(negation) => negation.apply(actor, resource),
            Requirement::Search(search) => search.apply(actor, resource),
        }
    }
}

impl TryFrom<&Token> for Requirement {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
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
    Actor(ActorAttribute),
    Resource(ResourceAttribute),
}

impl TryFrom<&Token> for Attribute {
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
    Value(Value),
}

impl TryFrom<&Token> for ComparableValue {
    type Error = Error;
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let value = match token {
            Token::ActorAttribute(attr) => Self::Attribute(Attribute::Actor(*attr)),
            Token::ResourceAttribute(attr) => Self::Attribute(Attribute::Resource(*attr)),
            Token::String(value) => Self::Value(Value::String(value)),
            Token::Array(arr) => Self::Value(Value::Array(*arr)),
            Token::Identifier(ident) => Self::Value(Value::Identifier(*ident)),
            _ => Err(Error::InvalidToken {
                expected: "ActorAttribute, ResourceAttribute, String, Array or Identifier",
                found: token.to_string(),
            })?,
        };

        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(Arc<String>),
    Array(Array),
    Identifier(Identifier),
}

#[derive(Debug, Clone, PartialEq, Ctor, Getters, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Assertion {
    left: Attribute,
    right: ComparableValue,
}

impl Assertion {
    fn resource_attr_attr_comparison(
        resource_attr: &ResourceAttribute,
        attr: &Attribute,
    ) -> Option<bool> {
        match (resource_attr, attr) {
            (ResourceAttribute::Id, Attribute::Resource(ResourceAttribute::Id)) => todo!(),
            _ => None,
        }
    }

    /// Returns an assertion result if the operation are permited.
    pub fn apply(&self, actor: &Actor, resource: &Resource) -> Option<bool> {
        match (&self.left, &self.right) {
            (Attribute::Actor(left), ComparableValue::Attribute(Attribute::Resource(rigth))) => {
                Some(Some(actor.get_attribute(*left)) == resource.get_attribute(*rigth))
            }
            (Attribute::Resource(left), ComparableValue::Attribute(Attribute::Actor(rigth))) => {
                Some(Some(actor.get_attribute(*rigth)) == resource.get_attribute(*left))
            }
            (Attribute::Actor(attr), ComparableValue::Value(value)) => {
                Some(&actor.get_attribute(*attr) == value)
            }
            (Attribute::Resource(attr), ComparableValue::Value(value)) => {
                Some(resource.get_attribute(*attr).as_ref() == Some(value))
            }
            _ => None,
        }
    }
}

impl TryFrom<&Vec<Token>> for Assertion{
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

impl Negation {
    pub fn apply(&self, actor: &Actor, resource: &Resource) -> Option<bool> {
        match (&self.left, &self.right) {
            (Attribute::Actor(left), ComparableValue::Attribute(Attribute::Resource(rigth))) => {
                Some(Some(actor.get_attribute(*left)) != resource.get_attribute(*rigth))
            }
            (Attribute::Resource(left), ComparableValue::Attribute(Attribute::Actor(rigth))) => {
                Some(Some(actor.get_attribute(*rigth)) != resource.get_attribute(*left))
            }
            (Attribute::Actor(attr), ComparableValue::Value(value)) => {
                Some(&actor.get_attribute(*attr) != value)
            }
            (Attribute::Resource(attr), ComparableValue::Value(value)) => {
                Some(resource.get_attribute(*attr).as_ref() != Some(value))
            }
            _ => None,
        }
    }
}

impl TryFrom<&Vec<Token>> for Negation {
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

impl Search {
    pub fn find_list_in_list(reference: &[&str], to_find_values: &[&str]) -> bool {
        for value in to_find_values {
            if !reference.contains(&value) {
                return false;
            }
        }

        true
    }

    pub fn apply(&self, actor: &Actor, resource: &Resource) -> Option<bool> {
        match (&self.left, &self.right) {
            (
                Attribute::Actor(ActorAttribute::Groups),
                ComparableValue::Value(Value::Array(value)),
            ) => Some(Self::find_list_in_list(
                actor.actor_groups(),
                value.as_slice(),
            )),
            (
                Attribute::Actor(ActorAttribute::Groups),
                ComparableValue::Value(Value::String(value)),
            ) => Some(actor.actor_groups().contains(value)),
            (
                Attribute::Actor(ActorAttribute::Roles),
                ComparableValue::Value(Value::Array(value)),
            ) => Some(Self::find_list_in_list(
                actor.actor_roles(),
                value.as_slice(),
            )),
            (
                Attribute::Actor(ActorAttribute::Roles),
                ComparableValue::Value(Value::String(value)),
            ) => Some(actor.actor_roles().contains(value)),
            _ => None,
        }
    }
}

impl TryFrom<&Vec<Token>> for Search {
    type Error = Error;
    fn try_from(token: &Vec<Token>) -> Result<Self, Self::Error> {
        let left = Attribute::try_from(token.first().ok_or(Error::MissingToken)?)?;
        let right = ComparableValue::try_from(token.get(2).ok_or(Error::MissingToken)?)?;

        Ok(Self { left, right })
    }
}
