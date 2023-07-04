use std::collections::HashMap;

use derived::Ctor;
use getset::Getters;

use crate::errors::Error;
use crate::parser::tokens::{FileVersion, Token};

use super::resource::Resource;
use super::storage::Storage;

#[derive(Debug, Clone, Ctor, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct File {
    sintaxis_version: FileVersion,
    storage: Storage,
}

impl TryFrom<Token> for File {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_file().ok_or(Error::InvalidToken {
            expected: "File",
            found: token.to_string(),
        })?;
        // let sintaxis_version = inner_tokens[0].inner_version().unwrap();
        // let mut resources = HashMap::new();

        // for inner_token in &inner_tokens[1..inner_tokens.len() - 1] {
        //     let resource = Resource::try_from(inner_token)?;
        //     resources.insert(resource.name().clone(), resource);
        // }

        // Ok(File {
        //     sintaxis_version,
        //     storage: Storage::new(resources),
        // })
        todo!()
    }
}
