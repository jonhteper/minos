use crate::authorization::Authorization;
use crate::authorization::{AuthorizationBuilder, Permission, Policy};
use crate::errors::MinosError;
use crate::group::{Group, GroupId};
use crate::resources::Owner;
use crate::resources::Resource;
use crate::resources::ResourceType;
use crate::user::UserAttributes;
use crate::utils::formatted_datetime_now;
use crate::Status;
#[cfg(test)]
use std::time::Instant;

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
fn authorization_by_group() {
    let message = Message {
        id: "example-message-id".to_string(),
        owner: Owner::User(regular_user().id),
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
fn unauthorized() {
    let message = Message {
        id: "example-message-id".to_string(),
        owner: Owner::Group(admin_group().id.to_string()),
        policies: vec![Policy {
            duration: 30,
            by_owner: false,
            groups_ids: Some(vec![admin_group().id]),
            permissions: Permission::crud(),
        }],
    };

    let invalid_user = regular_user();
    let policy = &message.policies[0];
    let _ = AuthorizationBuilder::new(&policy)
        .build(&message.id, &message.resource_type(), &invalid_user)
        .expect_err("Authorization should not be able to be created");
    let auth = AuthorizationBuilder::new(&policy)
        .build(&message.id, &message.resource_type(), &admin_user())
        .expect("Error building auth");
    let _ = auth
        .check(&message.id, &invalid_user, &Permission::Read)
        .expect_err("The user should not be able to read the resource");
}

#[test]
fn multi_groups() {
    let bench_instant = Instant::now();
    let message = Message {
        id: "example-message-id".to_string(),
        owner: Owner::Group(admin_group().id.to_string()),
        policies: vec![Policy {
            duration: 30,
            by_owner: false,
            groups_ids: Some(vec![
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                GroupId::from("other.group.id"),
                GroupId::from("2.group.id"),
                GroupId::from("3.group.id"),
                admin_group().id,
            ]),
            permissions: Permission::crud(),
        }],
    };
    let reader_user = admin_user();
    let policy = &message.policies[0];

    let auth = AuthorizationBuilder::new(&policy)
        .build(&message.id, &message.resource_type(), &reader_user)
        .expect("Error building auth");
    let _ = auth
        .check(&message.id, &reader_user, &Permission::Read)
        .expect("Error with auth checking");

    println!(
        "len: {}",
        &message.policies[0].groups_ids.as_ref().unwrap().len()
    );
    println!("Multi-group benchmark: {:.2?}", bench_instant.elapsed());
}

#[cfg(feature = "jwt")]
mod jwt_test {
    use crate::authorization::{Authorization, Permission};
    use crate::errors::MinosError;
    use crate::jwt::{AuthorizationClaims, TokenServer};
    use crate::resources::ResourceType;
    use crate::utils::{datetime_now, formatted_datetime_now};
    use chrono::Duration;
    use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

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

        let generate_claims = AuthorizationClaims::from(&auth);
        let generated_auth = &generate_claims
            .as_authorization(&resource_type)
            .expect("Error creating authorization from claims");

        assert_eq!(&&auth, &generated_auth);
        assert_eq!(&generate_claims, &AuthorizationClaims::from(generated_auth));

        Ok(())
    }

    #[test]
    fn token_test() {
        let key = b"secret";
        let token_server = TokenServer::new(
            Header::default(),
            EncodingKey::from_secret(key.as_slice()),
            DecodingKey::from_secret(key.as_slice()),
            Algorithm::HS256,
        );

        let user_id = "user-id";
        let resource_id = "resource-id";
        let resource_type = "example.resource";
        let auth_claims = AuthorizationClaims::new(
            vec![Permission::Read.to_string(), Permission::Update.to_string()],
            user_id.to_string(),
            resource_id.to_string(),
            resource_type.to_string(),
            datetime_now()
                .checked_add_signed(Duration::seconds(30))
                .unwrap(),
        );

        let token = token_server
            .generate_token(&auth_claims)
            .expect("Error generating token");

        let decoded_claims = token_server
            .get_claims_by_token(&token.as_str())
            .expect("Error obtaining claims ");

        assert_eq!(&auth_claims, &decoded_claims)
    }
}
