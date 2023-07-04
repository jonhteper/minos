use std::{borrow::Cow, sync::Arc};

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
    actor_groups: Vec<Cow<'a, str>>,
    actor_roles: Vec<Cow<'a, str>>,
}

impl<'a> Actor<'a> {
    pub(crate) fn get_attribute(&self, attr: ActorAttribute) -> Value {
        match attr {
            ActorAttribute::Type => {
                Value::Identifier(Identifier(Arc::from(self.actor_type.as_ref())))
            }
            ActorAttribute::Id => Value::String(Arc::from(self.actor_id.as_ref())),
            ActorAttribute::Groups => {
                let arr = self
                    .actor_groups
                    .iter()
                    .map(|v| Arc::from(v.as_ref()))
                    .collect();
                Value::Array(Array(arr))
            }
            ActorAttribute::Roles => {
                let arr = self
                    .actor_roles
                    .iter()
                    .map(|v| Arc::from(v.as_ref()))
                    .collect();
                Value::Array(Array(arr))
            }
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
