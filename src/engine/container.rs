use std::collections::HashMap;
use std::{marker::PhantomData, path::PathBuf};

use getset::Getters;

use crate::language::storage::Storage;
use crate::{engine::Engine, errors::MinosResult};

use crate::parser::MinosParser;

#[derive(Debug, Clone)]
pub struct EmptyContainer;

#[derive(Debug, Clone)]
pub struct StaticContainer;

/// Container is an high-level structure to load minos files.
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
    /// Container constructor.
    ///
    /// WARNING: is important to provide only absolute paths.
    pub fn new(id: String, description: String, paths: Vec<PathBuf>) -> Container<EmptyContainer> {
        Container {
            id,
            description,
            paths,
            storage: Storage::default(),
            state: PhantomData,
        }
    }

    /// Load all files from the provided paths. This function scan recursively.
    ///
    /// WARNING: This function not fail if the paths are not absolutes, but the files
    /// will not be readed.
    pub fn load(self) -> MinosResult<Container<StaticContainer>> {
        let Container {
            id,
            description,
            paths,
            storage,
            state: _,
        } = self;
        let mut storage = storage;
        let mut values_map = HashMap::new();
        for path in &paths {
            if path.is_dir() {
                let dir_storage = MinosParser::parse_dir(path, &mut values_map)?;
                storage.merge(dir_storage);
            } else if path.is_file() {
                let file_storage = MinosParser::parse_file(path, &mut values_map)?;
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
