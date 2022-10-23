use crate::NonEmptyString;

pub trait Agent {
    fn id(&self) -> NonEmptyString;
    fn groups(&self) -> Vec<NonEmptyString>;
}
