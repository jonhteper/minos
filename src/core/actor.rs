use non_empty_string::NonEmptyString;

pub trait Actor {
    fn id(&self) -> NonEmptyString;
    fn groups(&self) -> Vec<NonEmptyString>;
}
