use std::sync::Arc;

use derived::Ctor;
use getset::Getters;

use crate::{
    errors::Error,
    parser::tokens::{Identifier, Token},
    MinosResult,
};

use super::policy::Policy;

pub const DEFAULT_ENV_IDENTIFIER: &str = "DEFAULT";

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct Environment {
    identifier: Identifier,
    policies: Vec<Policy>,
}

impl Environment {
    fn from_named_env(tokens: &Vec<Token>) -> MinosResult<Self> {
        let identifier = tokens[0].inner_identifier().unwrap();
        let policies = tokens.iter().skip(1).map(Policy::try_from).collect()?;

        Ok(Self {
            identifier,
            policies,
        })
    }

    fn from_default_env(tokens: &Vec<Token>) -> MinosResult<Self> {
        let policies = tokens.iter().map(Policy::try_from).collect()?;

        Ok(Self {
            identifier: Identifier(Arc::new(DEFAULT_ENV_IDENTIFIER.to_string())),
            policies,
        })
    }

    pub fn add_policy(&mut self, policy: Policy) {
        self.policies.push(policy);
    }
}

impl TryFrom<&Token> for Environment {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::NamedEnv(inner_tokens) => Self::from_named_env(inner_tokens),
            Token::DefaultEnv(inner_tokens) => Self::from_default_env(inner_tokens),
            Token::ImplicitDefaultEnv(inner_tokens) => Self::from_default_env(inner_tokens),
            _ => Err(Error::InvalidToken {
                expected: "NamedEnv, DefaultEnv or ImplicitDefaultEnv",
                found: token.to_string(),
            })?,
        }
    }
}
