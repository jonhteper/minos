use derived::Ctor;
use getset::Getters;

use crate::{
    engine::{Actor, Resource},
    errors::{Error, MinosResult},
    parser::tokens::{Array, Token},
};

use super::rule::Rule;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Permission<'a>(pub &'a str);

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct Policy {
    allow: Vec<Permission<'static>>,
    rules: Vec<Rule>,
}

impl Policy {
    /// Returns the [Permission] list if the actor satisfies at least one of the rules.
    pub fn apply(&self, actor: &Actor, resource: &Resource) -> Option<&Vec<Permission>> {
        for rule in &self.rules {
            if rule.apply(actor, resource) {
                return Some(&self.allow);
            }
        }

        None
    }
}

impl TryFrom<&Token<'_>> for Policy {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        // let inner_tokens = token.inner_policy().ok_or(Error::InvalidToken {
        //     expected: Token::Policy(vec![]).to_string(),
        //     found: token.to_string(),
        // })?;
        // let Array(borrowed_allow) = inner_tokens[0].inner_allow().unwrap()[0]
        //     .inner_array()
        //     .unwrap();
        // let allow = borrowed_allow.iter().map(|p| p.to_string()).collect();
        // let rules: MinosResult<Vec<Rule>> =
        //     inner_tokens.iter().skip(1).map(Rule::try_from).collect();

        // Ok(Policy {
        //     allow,
        //     rules: rules?,
        // })

        todo!()
    }
}
