use std::sync::Arc;

use derived::Ctor;
use getset::{Getters, MutGetters};

use crate::{
    language::requirements::Value,
    parser::tokens::{ActorAttribute, Array, Identifier},
};

#[derive(Debug, Clone, PartialEq, Eq, Ctor, Getters, MutGetters)]
#[get = "pub"]
pub struct Actor {
    pub id: Arc<str>,
    pub type_: Arc<str>,
    pub groups: Vec<Arc<str>>,
    pub roles: Vec<Arc<str>>,
}

impl Actor {
    pub(crate) fn get_attribute(&self, attr: ActorAttribute) -> Value {
        match attr {
            ActorAttribute::Type => Value::Identifier(Identifier(self.type_.clone())),
            ActorAttribute::Id => Value::String(self.id.clone()),
            ActorAttribute::Groups => {
                let arr = self.groups.to_vec();
                Value::Array(Array(arr))
            }
            ActorAttribute::Roles => {
                let arr = self.roles.to_vec();
                Value::Array(Array(arr))
            }
        }
    }

    pub fn to_vec_arc(list: &[String]) -> Vec<Arc<str>> {
        list.iter().map(|s| Arc::from(s.as_str())).collect()
    }

    pub fn from_generic_list<T: AsRef<str>>(list: &[T]) -> Vec<Arc<str>> {
        list.iter().map(|s| Arc::from(s.as_ref())).collect()
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
