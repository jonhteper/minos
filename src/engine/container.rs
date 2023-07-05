use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

use getset::Getters;

use crate::language::storage::Storage;
use crate::{engine::Engine, errors::MinosResult};

use crate::parser::MinosParser;

#[derive(Debug, Clone)]
pub struct EmptyContainer;

#[derive(Debug, Clone)]
pub struct StaticContainer;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct Container<State = EmptyContainer> {
    id: String,
    description: String,
    paths: Vec<PathBuf>,

    #[getset(skip)]
    storage: Storage,

    #[getset(skip)]
    state: PhantomData<State>,
}

impl Container {
    pub fn new(id: String, description: String, paths: Vec<PathBuf>) -> Container< EmptyContainer> {
        Container {
            id,
            description,
            paths,
            storage: Storage::default(),
            state: PhantomData,
        }
    }

    pub fn load(self) -> MinosResult<Container<StaticContainer>> {
        let Container {
            id,
            description,
            paths,
            storage,
            state: _,
        } = self;
        let mut storage = storage;
        for path in &paths {
            if path.is_dir() {
                let dir_storage = MinosParser::parse_dir(path)?;
                storage.merge(dir_storage);
            } else if path.is_file() {
                let file_storage = MinosParser::parse_file(path)?;
                storage.merge(file_storage);
            }
        }

        Ok(Container {
            id,
            description,
            paths,
            storage,
            state: PhantomData,
        })
    }
}

impl Container<StaticContainer> {
    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    pub fn engine(&self) -> Engine {
        Engine::new(&self.storage)
    }
}
