use std::collections::HashMap;

use derived::Ctor;
use getset::Getters;

use crate::{
    errors::Error,
    parser::tokens::{Identifier, Token},
};

use super::{policy::Permission, rule::Rule};

pub const DEFAULT_ENV_IDENTIFIER: &str = "DEFAULT";

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct Environment {
    identifier: Identifier<'static>,
    policies: HashMap<Permission<'static>, Vec<Rule>>,
}

impl TryFrom<&Token<'_>> for Environment {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        todo!()
    }
}
