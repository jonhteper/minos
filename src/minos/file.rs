use std::collections::HashMap;

use derived::Ctor;
use getset::Getters;

use super::{lang::FileVersion, environment::{EnvName, Environment}};

#[derive(Debug, Clone, Ctor, Getters)]
#[getset(get = "pub")]
pub struct File {
    sintaxis_version: FileVersion,
    environments: HashMap<EnvName, Environment>,
}