use crate::authorization::{
    AuthorizationBuilder, Owner, Permission, Policy, Resource, ResourceType,
};
use crate::group::{Group, GroupId};
use crate::user::UserAttributes;
use crate::Status;

fn users_group() -> Group {
    Group {
        id: GroupId::from("Users-Group-Id"),
        alias: "Users".to_string(),
        status: Status::Active,
    }
}

fn admin_group() -> Group {
    Group {
        id: GroupId::from("Admins-Group-Id"),
        alias: "Admins".to_string(),
        status: Status::Active,
    }
}

fn regular_user() -> UserAttributes {
    UserAttributes {
        id: "regular-user-id".to_string(),
        alias: "Regular User".to_string(),
        status: Status::Active,
        groups: vec![users_group().id],
    }
}

struct Message {
    id: String,
}

impl Resource for Message {
    fn id(&self) -> &str {
        &self.id
    }
    fn resource_type(&self) -> ResourceType {
        ResourceType {
            label: "".to_string(),
            owner: Some(Owner::User(regular_user().id.clone())),
            policies: vec![Policy {
                duration: 60,
                by_owner: true,
                groups_ids: None,
                permissions: Permission::crud(),
            }],
        }
    }
}

#[cfg(test)]
#[test]
fn authorization_by_user_test() {
    let message = Message {
        id: "example-message-id".to_string(),
    };
    let policies = &message.resource_type().policies;

    let auth = AuthorizationBuilder::new(&policies[0])
        .build(&message.id, &message.resource_type(), &regular_user())
        .expect("Error building Authorization");

    let _ = auth
        .check(message.id(), &regular_user(), &Permission::Create)
        .expect("Error with authorization");
}
