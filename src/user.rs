use crate::group::GroupId;
use crate::Status;

#[derive(PartialEq, Debug, Clone, Default)]
pub struct UserAttributes {
    pub id: String,
    pub alias: String,
    pub status: Status,
    pub groups: Vec<GroupId>,
}
