use crate::errors::MinosError;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd)]
pub struct Permission {
    /// expressive id
    name: String,
    /// in milliseconds
    duration: u64,
}

impl Permission {
    pub fn required_msg(&self) -> &str {
        &self.name
    }
}


impl TryFrom<Vec<String>> for Permission {
    type Error = MinosError;
    fn try_from(non_empty_string: Vec<String>) -> Result<Self, Self::Error> {
        todo!()
    }
}
