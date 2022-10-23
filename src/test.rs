#[cfg(test)]
#[cfg(not(feature = "resource_utils"))]
mod std {
    use crate::agent::Agent;
    use crate::authorization::{Permission, Policy};
    use crate::authorization_builder::AuthorizationBuilder;
    use crate::NonEmptyString;
    use crate::resources::Resource;

    pub struct User {
        pub id: NonEmptyString,
        pub alias: String,
        pub status: u8,
        pub groups: Vec<NonEmptyString>,
    }

    impl Agent for User {
        fn id(&self) -> NonEmptyString {
            self.id.clone()
        }

        fn groups(&self) -> Vec<NonEmptyString> {
            self.groups.clone()
        }
    }

    pub struct Group {
        pub id: NonEmptyString,
        pub alias: String,
    }

    fn users_group() -> Group {
        Group {
            id: NonEmptyString::from_str("Users-Group-Id").unwrap(),
            alias: "Users".to_string(),
        }
    }

    fn admin_group() -> Group {
        Group {
            id: NonEmptyString::from_str("Admins-Group-Id").unwrap(),
            alias: "Admins".to_string(),
        }
    }

    fn regular_user() -> User {
        User {
            id: NonEmptyString::from_str("regular-user-id").unwrap(),
            alias: "Regular User".to_string(),
            status: 1,
            groups: vec![users_group().id],
        }
    }

    fn admin_user() -> User {
        User {
            id: NonEmptyString::from_str("admin-user-id").unwrap(),
            alias: "Admin User".to_string(),
            status: 1,
            groups: vec![admin_group().id],
        }
    }

    struct Message {
        id: NonEmptyString,
        resource_type: NonEmptyString,
        owner: NonEmptyString,
        policies: Vec<Policy>,
    }

    impl Resource for Message {
        fn id(&self) -> NonEmptyString {
            self.id.clone()
        }

        fn owner(&self) -> Option<NonEmptyString> {
            Some(self.owner.clone())
        }

        fn policies(&self) -> Vec<Policy> {
            self.policies.clone()
        }

        fn resource_type(&self) -> Option<NonEmptyString> {
            Some(self.resource_type.clone())
        }
    }


    #[test]
    fn authorization_by_user() {
        let user = regular_user();
        let message = Message {
            id: NonEmptyString::from_str("example-message-id").unwrap(),
            resource_type: NonEmptyString::from_str("message").unwrap(),
            owner: user.id(),
            policies: vec![Policy {
                duration: 60,
                by_owner: true,
                groups_ids: None,
                permissions: Permission::crud(),
            }],
        };

        let auth = AuthorizationBuilder::new(&message)
            .build(&user)
            .expect("Error building Authorization");

        let _= auth
            .search_permission(Permission::Create)
            .expect("Error with authorization");
    }

    #[test]
    fn authorization_by_group() {
        let message = Message {
            id: NonEmptyString::from_str("example-message-id").unwrap(),
            resource_type: NonEmptyString::from_str("message").unwrap(),
            owner: regular_user().id(),
            policies: vec![Policy {
                duration: 200,
                by_owner: false,
                groups_ids: Some(vec![admin_group().id]),
                permissions: vec![Permission::Read],
            }],
        };

        let reader_user = admin_user();
        let auth = AuthorizationBuilder::new(&message)
            .build(&reader_user)
            .expect("Error building Authorization");
        let _= auth
            .search_permission(Permission::Read)
            .expect("Error with permission");
    }

    #[test]
    fn unauthorized() {
        let message = Message {
            id: NonEmptyString::from_str("example-message-id").unwrap(),
            resource_type: NonEmptyString::from_str("message").unwrap(),
            owner: admin_group().id,
            policies: vec![Policy {
                duration: 30,
                by_owner: false,
                groups_ids: Some(vec![admin_group().id]),
                permissions: Permission::crud(),
            }],
        };

        let invalid_user = regular_user();
        let builder = AuthorizationBuilder::new(&message);
        let _auth = builder
            .build(&invalid_user)
            .expect_err("Authorization should not be able to be created");
        let auth = builder
            .build(&admin_user())
            .expect("Error building auth");

        let _ = auth
            .check(&message.id.to_string(), &invalid_user, Permission::Read)
            .expect_err("The user should not be able to read the resource");
    }

    #[test]
    fn multi_groups() {
        let message = Message {
            id: NonEmptyString::from_str("example-message-id").unwrap(),
            resource_type: NonEmptyString::from_str("message").unwrap(),
            owner: admin_group().id,
            policies: vec![Policy {
                duration: 30,
                by_owner: false,
                groups_ids: Some(vec![
                    NonEmptyString::from_str("other.group.id").unwrap(),
                    NonEmptyString::from_str("2.group.id").unwrap(),
                    NonEmptyString::from_str("3.group.id").unwrap(),
                    admin_group().id,
                ]),
                permissions: Permission::crud(),
            }],
        };
        let reader_user = admin_user();
        let _ = AuthorizationBuilder::new(&message)
            .build(&reader_user)
            .expect("Error building auth")
            .search_permission(Permission::Read)
            .expect("Error with permission");
    }

    #[test]
    fn multi_permissions() {
        let user = regular_user();
        let message = Message {
            id: NonEmptyString::from_str("example-message-id").unwrap(),
            resource_type: NonEmptyString::from_str("message").unwrap(),
            owner: user.id(),
            policies: vec![Policy {
                duration: 6,
                by_owner: true,
                groups_ids: None,
                permissions: Permission::crud(),
            }],
        };
        let auth = AuthorizationBuilder::new(&message)
            .build(&user)
            .expect("Error building Authorization");

        auth.multi_permissions_check(
            &message.id().to_string(),
            &user,
            &vec![Permission::Read, Permission::Delete],
        )
            .expect("Error with authorization check");

        auth.multi_permissions_check(
            &message.id().to_string(),
            &user,
            &vec![Permission::Read, Permission::from("sign")],
        )
            .expect_err("The authorization check must failed");
    }
}
/*
#[cfg(feature = "jwt")]
#[cfg(test)]
mod jwt_test {
    use crate::authorization::{Authorization, Permission};
    use crate::errors::MinosError;
    use crate::jwt::{AuthorizationClaims, TokenServer};
    use crate::resources::{OwnerType, ResourceType};
    use crate::utils::formatted_datetime_now;
    use chrono::{Duration, Utc};
    use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header};

    #[test]
    fn authorization_as_claims() -> Result<(), MinosError> {
        let resource_type = ResourceType {
            label: "resource".to_string(),
            owner_type: OwnerType::None,
            policies: vec![],
        };

        let auth = Authorization {
            permissions: Permission::crud(),
            agent_id: "user-id".to_string(),
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
            Utc::now()
                .naive_utc()
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



#[cfg(feature = "toml_storage")]
#[cfg(test)]
mod toml_test {
    use crate::authorization::{Permission, Policy};
    use crate::errors::MinosError;
    use crate::group::GroupId;
    use crate::resources::Owner::User;
    use crate::resources::{OwnerType, ResourceType};
    use crate::test::create_temp_file;
    use crate::toml::TomlFile;
    use std::env;
    use std::time::Instant;

    fn create_temp_file(content: &str) -> Result<PathBuf, MinosError> {
        let mut path = env::temp_dir();
        path.push("example.resource.toml");
        let mut file = File::create(&path)?;

        let _ = file.write_all(&content.as_bytes())?;

        Ok(path)
    }

    static FILE_CONTENT: &str = r#"
    label = "example resource"
    owner_type = {user = true, group = false}

    [[policies]]
    duration = 120
    by_owner = true
    permissions = ["create", "read", "update", "delete"]

    [[policies]]
    duration = 300
    by_owner = false
    groups_ids = ["example-group-id-1", "example-group-id-2"]
    permissions = ["read"]"#;

    #[test]
    fn resource_type_by_file() -> Result<(), MinosError> {
        let bench_instant = Instant::now();
        let path = create_temp_file(FILE_CONTENT)?;
        let toml_file = TomlFile::try_from(path)?;
        let resource_type = ResourceType::try_from(toml_file)?;

        println!("{:#?}", resource_type);
        println!(
            "resource type by file benchmark: {:?}",
            bench_instant.elapsed()
        );

        Ok(())
    }

    #[test]
    fn file_by_resource_type() -> Result<(), MinosError> {
        let mut path = env::temp_dir();
        path.push("ref.resource_type.toml");
        let resource_type = ResourceType::new(
            "example resource".to_string(),
            OwnerType::User,
            vec![
                Policy::new(120, true, None, Permission::crud()),
                Policy::new(
                    300,
                    false,
                    Some(vec![
                        GroupId::from("example-group-id-1"),
                        GroupId::from("example-group-id-2"),
                    ]),
                    vec![Permission::Read],
                ),
            ],
        );
        let _ = TomlFile::create(&resource_type, &path)?;
        let path = create_temp_file(FILE_CONTENT)?;
        let toml_file = TomlFile::try_from(path)?;
        let saved_resource_type = ResourceType::try_from(toml_file)?;

        assert_eq!(resource_type, saved_resource_type);

        Ok(())
    }
}
*/
