use derived::Ctor;
use getset::{Getters, MutGetters};

use crate::{
    errors::Error,
    parser::tokens::{Identifier, Token},
    MinosResult,
};

use super::policy::Policy;

pub const DEFAULT_ENV_IDENTIFIER: &str = "DEFAULT";

#[derive(Debug, Clone, Ctor, Getters, MutGetters, PartialEq)]
pub struct Environment {
    #[get = "pub"]
    identifier: Identifier,
    #[getset(get = "pub", get_mut = "pub")]
    policies: Vec<Policy>,
}

impl Environment {
    fn from_named_env(tokens: &Vec<Token>) -> MinosResult<Self> {
        let identifier = tokens[0].inner_identifier().unwrap().clone();
        let policies =
            tokens.iter().skip(1).map(Policy::try_from).collect::<MinosResult<Vec<Policy>>>()?;

        Ok(Self {
            identifier,
            policies,
        })
    }

    fn from_default_env(tokens: &Vec<Token>) -> MinosResult<Self> {
        let policies = tokens.iter().map(Policy::try_from).collect::<MinosResult<Vec<Policy>>>()?;

        Ok(Self {
            identifier: Identifier(DEFAULT_ENV_IDENTIFIER.into()),
            policies,
        })
    }

    pub fn add_policy(&mut self, policy: Policy) {
        self.policies.push(policy);
    }

    pub fn add_policies(&mut self, policies: &mut Vec<Policy>) {
        self.policies.append(policies);
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
