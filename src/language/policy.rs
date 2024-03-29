use std::{collections::HashMap, sync::Arc};

use derived::Ctor;
use getset::Getters;

use crate::{
    engine::{ActorRepr, ResourceRepr},
    errors::Error,
    parser::tokens::{Array, Token},
    MinosResult,
};

use super::rule::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Permission(pub Arc<str>);
impl From<&str> for Permission {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<String> for Permission {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct Policy {
    permissions: Vec<Permission>,
    rules: Vec<Arc<Rule>>,
    rules_map: HashMap<Permission, Vec<Arc<Rule>>>,
}

impl Policy {
    /// Indicates if an [Actor] has a specific [Permission] on a [Resource].
    pub(crate) fn actor_has_permission(
        &self,
        actor: &ActorRepr,
        resource: &ResourceRepr,
        permission: &Permission,
    ) -> bool {
        if let Some(rules) = self.rules_map.get(permission) {
            for rule in rules {
                if rule.apply(actor, resource) {
                    return true;
                }
            }
        }

        false
    }

    /// Returns the [Permission] list if the actor satisfies at least one of the rules.
    pub(crate) fn apply(&self, actor: &ActorRepr, resource: &ResourceRepr) -> Option<&[Permission]> {
        for rule in &self.rules {
            if rule.apply(actor, resource) {
                return Some(&self.permissions);
            }
        }

        None
    }
}

impl TryFrom<&Token> for Policy {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_policy().ok_or(Error::InvalidToken {
            expected: "Policy",
            found: token.to_string(),
        })?;

        let Array(permissions) = inner_tokens[0].inner_allow().unwrap()[0].inner_array().unwrap();

        let rules = inner_tokens
            .iter()
            .skip(1)
            .map(|token| Rule::try_from(token).map(Arc::new))
            .collect::<MinosResult<Vec<Arc<Rule>>>>()?;

        let mut rules_map = HashMap::new();

        for raw_permission in permissions {
            rules_map.insert(Permission(raw_permission.clone()), rules.clone());
        }

        let permissions = permissions.iter().map(|v| Permission(v.clone())).collect();

        Ok(Policy {
            permissions,
            rules,
            rules_map,
        })
    }
}
