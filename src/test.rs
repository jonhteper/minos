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
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
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

    let auth = AuthorizationBuilder::new(&message.resource_type())
        .build(&message.id, &regular_user())
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
    let auth = AuthorizationBuilder::new(&message.resource_type())
        .build(&message.id, &reader_user)
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
    let _ = AuthorizationBuilder::new(&message.resource_type())
        .build(&message.id, &invalid_user)
        .expect_err("Authorization should not be able to be created");
    let auth = AuthorizationBuilder::new(&message.resource_type())
        .build(&message.id, &admin_user())
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
    let auth = AuthorizationBuilder::new(&message.resource_type())
        .build(&message.id, &reader_user)
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
#[cfg(test)]
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
            resource_type: resource_type.label().to_string(),
            expiration: formatted_datetime_now()?,
        };

        let generate_claims = AuthorizationClaims::from(&auth);
        let generated_auth = &generate_claims
            .as_authorization()
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

fn create_temp_file(content: &str) -> Result<PathBuf, MinosError> {
    let mut path = env::temp_dir();
    path.push("example.resource.toml");
    let mut file = File::create(&path)?;

    let _ = file.write_all(&content.as_bytes())?;

    Ok(path)
}

#[cfg(feature = "toml_storage")]
#[cfg(test)]
mod toml_test {
    use crate::errors::MinosError;
    use crate::resources::{Owner, ResourceType};
    use crate::test::create_temp_file;
    use crate::toml::TomlFile;
    use std::time::Instant;

    static FILE_CONTENT: &str = r#"
            label = "example resource"
            owner = {user = true, group = false}

            [[policies]]
            duration = 120
            by_owner = true
            permissions = ["create", "read", "update", "delete"]

            [[policies]]
            duration = 300
            by_owner = false
            groups_ids = ["example-group-id-1", "example-group-id-2"]
            permissions = ["read"]
        "#;

    #[test]
    fn resource_type_by_file() -> Result<(), MinosError> {
        let bench_instant = Instant::now();
        let path = create_temp_file(FILE_CONTENT)?;
        let toml_file = TomlFile::try_from(path)?;
        let mut resource_type = ResourceType::try_from(toml_file)?;
        resource_type.owner = Some(Owner::User("user-id".to_string()));

        println!("{:#?}", resource_type);
        println!(
            "resource type by file benchmark: {:?}",
            bench_instant.elapsed()
        );

        Ok(())
    }
}

#[cfg(feature = "custom_permission")]
#[cfg(test)]
mod custom_permission_test {
    use crate::authorization::{AuthorizationBuilder, Permission};
    use crate::errors::MinosError;
    use crate::group::GroupId;
    use crate::resources::{Owner, ResourceType};
    use crate::test::create_temp_file;
    use crate::toml::TomlFile;
    use crate::user::UserAttributes;
    use crate::Status;

    static FILE_CONTENT: &str = r#"
            label = "payment blog post"
            owner = {user = false, group = true}

            [[policies]]
            duration = 120
            by_owner = true
            permissions = ["create", "update", "delete", "read_header", "read_post"]

            [[policies]]
            duration = 300
            by_owner = false
            groups_ids = ["non-suscribed-users-id", "suscribed-users-id"]
            permissions = ["read_header"]

            [[policies]]
            duration = 300
            by_owner = false
            groups_ids = ["suscribed-users-id"]
            permissions = ["read_post"]
        "#;

    fn editor_user() -> UserAttributes {
        UserAttributes {
            id: "editor-jd".to_string(),
            alias: "John Doe".to_string(),
            status: Status::Active,
            groups: vec![GroupId::from("editors-group-id")],
        }
    }

    #[test]
    fn custom_permissions_by_file() -> Result<(), MinosError> {
        let path = create_temp_file(FILE_CONTENT)?;
        let toml_file = TomlFile::try_from(path)?;
        let mut payment_blog_post_rt = ResourceType::try_from(toml_file)?;
        payment_blog_post_rt.owner = Some(Owner::Group("editors-group-id".to_string()));

        let default_id = "DEFAULT_ID";
        let user_attr = editor_user();
        let auth =
            AuthorizationBuilder::new(&payment_blog_post_rt).build(&default_id, &user_attr)?;
        let _ = auth.check(
            &default_id,
            &user_attr,
            &Permission::Custom("read_post".to_string()),
        )?;

        Ok(())
    }
}
