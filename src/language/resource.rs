use std::collections::HashMap;

use derived::Ctor;
use getset::Getters;

use crate::{
    errors::{Error, MinosResult},
    parser::tokens::{Identifier, Token},
};

use super::{policy::Policy, environment::Environment};

pub type ResourceId = String;
pub type ResourceName = String;

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct Resource {
    identifier: Identifier<'static>,
    environments: HashMap<Identifier<'static>, Environment>,
}

impl TryFrom<&Token<'_>> for Resource {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct AttributedResource {
    identifier: Identifier<'static>,
    id: &'static str,
    environments: HashMap<Identifier<'static>, Environment>,
}

impl TryFrom<&Token<'_>> for AttributedResource {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        todo!()
    }
}