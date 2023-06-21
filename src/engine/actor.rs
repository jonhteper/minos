use std::borrow::Cow;

use derived::Ctor;
use getset::{Getters, MutGetters};

#[derive(Debug, Clone, PartialEq, Eq, Ctor, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct Actor<'a> {
    actor_type: Cow<'a, str>,
    actor_id: Cow<'a, str>,
    actor_groups: Cow<'a, [String]>,
    actor_roles: Cow<'a, [String]>,
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
