use std::borrow::Cow;

use derived::Ctor;
use getset::{Getters, MutGetters};

use crate::{
    language::requirements::Value,
    parser::tokens::{ActorAttribute, Array, Identifier},
};

#[derive(Debug, Clone, PartialEq, Eq, Ctor, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct Actor<'a> {
    actor_type: Cow<'a, str>,
    actor_id: Cow<'a, str>,
    actor_groups: Vec<&'a str>,
    actor_roles: Vec<&'a str>,
}

impl<'a> Actor<'a> {
    pub(crate) fn get_attribute(&self, attr: ActorAttribute) -> Value {
        match attr {
            ActorAttribute::Type => Value::Identifier(Identifier(&self.actor_type)),
            ActorAttribute::Id => Value::String(&self.actor_id),
            ActorAttribute::Groups => Value::Array(Array(self.actor_groups)),
            ActorAttribute::Roles => Value::Array(Array(self.actor_roles)),
        }
    }
}

pub trait AsActor {
    fn as_actor(&self) -> Actor;
}

pub trait IntoActor {
    fn into_actor<'a>(self) -> Actor<'a>;
}

pub trait TryIntoActor {
    type Error;
    fn try_into_actor<'a>(self) -> Result<Actor<'a>, Self::Error>;
}
