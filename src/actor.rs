use crate::NonEmptyString;

pub trait Actor {
    fn id(&self) -> NonEmptyString;
    fn groups(&self) -> Vec<NonEmptyString>;
}
