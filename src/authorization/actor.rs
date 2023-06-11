
pub trait Actor {
    fn actor_type(&self) -> String;
    fn actor_id(&self) -> String;
    fn actor_groups(&self) -> Vec<String>;
    fn actor_roles(&self) -> Vec<String>;
}