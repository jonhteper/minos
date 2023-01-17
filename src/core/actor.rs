use non_empty_string::NonEmptyString;

pub type ActorId = NonEmptyString;


pub trait Actor {
    fn id(&self) -> ActorId;
    fn groups(&self) -> Vec<NonEmptyString>;
}
