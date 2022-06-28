use crate::Status;

#[derive(PartialEq, Debug, Clone, Default, PartialOrd)]
pub struct GroupId(String);

impl From<&str> for GroupId {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}

impl From<String> for GroupId {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl ToString for GroupId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl GroupId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
/// Groups users and defines the permissions of the users belonging to it.
pub struct Group {
    pub(crate) id: GroupId,
    pub(crate) alias: String,
    pub(crate) status: Status,
}

impl Group {
    // TODO: implementations
}
