use std::sync::Arc;

use getset::Getters;

use crate::{
    language::requirements::Value,
    parser::tokens::{ActorAttribute, Array, Identifier},
};

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct Actor {
    pub id: String,
    pub type_: String,
    pub status: Option<String>,
    pub groups: Vec<String>,
    pub roles: Vec<String>,
}

pub trait AsActor {
    fn as_actor(&self) -> Actor;
}

pub trait IntoActor {
    fn into_actor(self) -> Actor;
}

pub trait TryIntoActor {
    type Error;
    fn try_into_actor(self) -> Result<Actor, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub(crate) struct ActorRepr {
    pub id: Arc<str>,
    pub type_: Arc<str>,
    pub status: Option<Arc<str>>,
    pub groups: Vec<Arc<str>>,
    pub roles: Vec<Arc<str>>,
}

impl ActorRepr {
    pub(crate) fn get_attribute(&self, attr: ActorAttribute) -> Option<Value> {
        match attr {
            ActorAttribute::Type => Some(Value::Identifier(Identifier(self.type_.clone()))),
            ActorAttribute::Id => Some(Value::String(self.id.clone())),
            ActorAttribute::Groups => {
                let arr = self.groups.to_vec();
                Some(Value::Array(Array(arr)))
            }
            ActorAttribute::Roles => {
                let arr = self.roles.to_vec();
                Some(Value::Array(Array(arr)))
            }
            ActorAttribute::Status => self
                .status
                .as_ref()
                .map(|status| Value::Identifier(Identifier(status.clone()))),
        }
    }

    fn transform_list(list: &[String]) -> Vec<Arc<str>> {
        list.iter().map(|s| Arc::from(s.as_str())).collect()
    }
}

impl From<&Actor> for ActorRepr {
    fn from(actor: &Actor) -> Self {
        Self {
            id: Arc::from(actor.id.as_str()),
            type_: Arc::from(actor.type_.as_str()),
            status: actor.status.as_ref().map(|s| Arc::from(s.as_str())),
            groups: Self::transform_list(&actor.groups),
            roles: Self::transform_list(&actor.roles),
        }
    }
}
