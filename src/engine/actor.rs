use std::sync::Arc;

use derived::Ctor;
use getset::{Getters, MutGetters};

use crate::{
    language::requirements::Value,
    parser::tokens::{ActorAttribute, Array, Identifier},
};

#[derive(Debug, Clone, PartialEq, Eq, Ctor, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct Actor {
    actor_type: Arc<str>,
    actor_id: Arc<str>,
    actor_groups: Vec<Arc<str>>,
    actor_roles: Vec<Arc<str>>,
}

impl Actor {
    pub(crate) fn get_attribute(&self, attr: ActorAttribute) -> Value {
        match attr {
            ActorAttribute::Type => Value::Identifier(Identifier(self.actor_type.clone())),
            ActorAttribute::Id => Value::String(self.actor_id.clone()),
            ActorAttribute::Groups => {
                let arr = self.actor_groups.to_vec();
                Value::Array(Array(arr))
            }
            ActorAttribute::Roles => {
                let arr = self.actor_roles.to_vec();
                Value::Array(Array(arr))
            }
        }
    }
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
