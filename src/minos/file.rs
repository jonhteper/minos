use std::collections::HashMap;

use derived::Ctor;
use getset::Getters;

use crate::errors::Error;

use super::{
    environment::{EnvName, Environment},
    parser::tokens::{FileVersion, Token},
};

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct File {
    sintaxis_version: FileVersion,
    environments: HashMap<EnvName, Environment>,
}

impl TryFrom<Token<'_>> for File {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_file().ok_or(Error::InvalidToken {
            expected: Token::File(vec![]).to_string(),
            found: token.to_string(),
        })?;
        let sintaxis_version = inner_tokens[0].inner_version().unwrap();
        let mut environments = HashMap::new();

        for inner_token in &inner_tokens[1..inner_tokens.len() - 1] {
            let env = Environment::try_from(inner_token)?;
            environments.insert(env.name().clone(), env);
        }

        Ok(File {
            sintaxis_version,
            environments,
        })
    }
}
