use derived::Ctor;
use getset::{Getters, MutGetters};

#[derive(Debug, Clone, PartialEq, Eq, Ctor, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Actor {
    actor_type: String,
    actor_id: String,
    actor_groups: Vec<String>,
    actor_roles: Vec<String>,
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
