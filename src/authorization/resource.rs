pub trait Resource {
    fn name(&self) -> String;
    fn id(&self) -> Option<String>;
}
