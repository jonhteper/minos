#[cfg(test)]
use crate::authorization::{Authorization, AuthorizationClaims};
use crate::authorization::{AuthorizationBuilder, Permission, Policy};
use crate::errors::MinosError;
use crate::group::{Group, GroupId};
use crate::resources::Owner;
use crate::resources::Resource;
use crate::resources::ResourceType;
use crate::user::UserAttributes;
use crate::utils::formatted_datetime_now;
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

fn admin_user() -> UserAttributes {
    UserAttributes {
        id: "admin-user-id".to_string(),
        alias: "Admin User".to_string(),
        status: Status::Active,
        groups: vec![admin_group().id],
    }
}

struct Message {
    id: String,
    owner: Owner,
    policies: Vec<Policy>,
}

impl Resource for Message {
    fn id(&self) -> &str {
        &self.id
    }
    fn resource_type(&self) -> ResourceType {
        ResourceType {
            label: "".to_string(),
            owner: Some(self.owner.clone()),
            policies: self.policies.clone(),
        }
    }
}

#[test]
fn authorization_by_user_test() {
    let message = Message {
        id: "example-message-id".to_string(),
        owner: Owner::User(regular_user().id.clone()),
        policies: vec![Policy {
            duration: 60,
            by_owner: true,
            groups_ids: None,
            permissions: Permission::crud(),
        }],
    };
    let policy = &message.resource_type().policies[0];

    let auth = AuthorizationBuilder::new(&policy)
        .build(&message.id, &message.resource_type(), &regular_user())
        .expect("Error building Authorization");

    let _ = auth
        .check(message.id(), &regular_user(), &Permission::Create)
        .expect("Error with authorization");
}

#[test]
fn authorization_by_groups() {
    let message = Message {
        id: "example-message-id".to_string(),
        owner: Owner::User(regular_user().id.clone()),
        policies: vec![Policy {
            duration: 200,
            by_owner: false,
            groups_ids: Some(vec![admin_group().id]),
            permissions: vec![Permission::Read],
        }],
    };

    let reader_user = admin_user();

    let policy = &message.policies[0];

    let auth = AuthorizationBuilder::new(&policy)
        .build(&message.id, &message.resource_type(), &reader_user)
        .expect("Error building Authorization");

    let _ = auth
        .check(message.id(), &reader_user, &Permission::Read)
        .expect("Error with authorization");
}

#[test]
fn authorization_as_claims() -> Result<(), MinosError> {
    let resource_type = ResourceType {
        label: "resource".to_string(),
        owner: None,
        policies: vec![],
    };

    let auth = Authorization {
        permissions: Permission::crud(),
        user_id: "user-id".to_string(),
        resource_id: "resource-id".to_string(),
        resource_type: resource_type.clone(),
        expiration: formatted_datetime_now()?,
    };

    let generate_claims = &auth.as_claims();
    let generated_auth = &generate_claims
        .as_authorization(&resource_type)
        .expect("Error creating authorization from claims");

    assert_eq!(&&auth, &generated_auth);
    assert_eq!(&generate_claims, &&generated_auth.as_claims());

    Ok(())
}
