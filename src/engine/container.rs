use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

use getset::Getters;

use crate::{
    engine::{Actor, Authorizator, Resource},
    errors::MinosResult,
};

use crate::language::{
    environment::{EnvName, Environment},
    policy::Permission,
};
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
    environments: HashMap<EnvName, Environment>,

    #[getset(skip)]
    state: PhantomData<State>,
}

impl Container {
    pub fn new(id: String, description: String, paths: Vec<PathBuf>) -> Container<EmptyContainer> {
        Container {
            id,
            description,
            paths,
            environments: HashMap::new(),
            state: PhantomData,
        }
    }

    pub fn load(self) -> MinosResult<Container<StaticContainer>> {
        let Container {
            id,
            description,
            paths,
            environments: _,
            state: _,
        } = self;
        let mut environments = HashMap::new();
        for path in &paths {
            if path.is_dir() {
                let envs = MinosParser::parse_dir(path)?;
                environments.extend(envs);
            } else if path.is_file() {
                let envs = MinosParser::parse_file(path)?;
                environments.extend(envs);
            }
        }

        Ok(Container {
            id,
            description,
            paths,
            environments,
            state: PhantomData,
        })
    }
}

impl Container<StaticContainer> {
    pub fn environments(&self) -> &HashMap<EnvName, Environment> {
        &self.environments
    }

    pub fn authorize(
        &self,
        env_name: &EnvName,
        actor: &Actor,
        resource: &Resource,
    ) -> MinosResult<Vec<Permission>> {
        let auth = Authorizator::new(&self.environments);

        auth.authorize(env_name, actor, resource)
    }
}
