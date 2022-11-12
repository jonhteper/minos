#[cfg(test)]
#[cfg(not(feature = "custom_authorization"))]
mod std {
    use std::num::NonZeroU64;
    use crate::actor::Actor;
    use crate::authorization::{AuthorizationMode, Permission, Policy};
    use crate::authorization_builder::AuthorizationBuilder;
    use crate::resources::Resource;
    use crate::NonEmptyString;

    pub struct User {
        pub id: NonEmptyString,
        pub alias: String,
        pub status: u8,
        pub groups: Vec<NonEmptyString>,
    }

    impl Actor for User {
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
                duration: NonZeroU64::new(60).unwrap(),
                auth_mode: AuthorizationMode::Owner,
                groups_ids: None,
                permissions: Permission::crud(),
            }],
        };

        let auth = AuthorizationBuilder::new(&message)
            .build(&user)
            .expect("Error building Authorization");

        auth.search_permission(Permission::Create)
            .expect("Error with authorization");
    }

    #[test]
    fn authorization_by_group() {
        let message = Message {
            id: NonEmptyString::from_str("example-message-id").unwrap(),
            resource_type: NonEmptyString::from_str("message").unwrap(),
            owner: regular_user().id(),
            policies: vec![Policy {
                duration: NonZeroU64::new(200).unwrap(),
                auth_mode: AuthorizationMode::SingleGroup,
                groups_ids: Some(vec![admin_group().id]),
                permissions: vec![Permission::Read],
            }],
        };

        let reader_user = admin_user();
        let auth = AuthorizationBuilder::new(&message)
            .build(&reader_user)
            .expect("Error building Authorization");
        auth.search_permission(Permission::Read)
            .expect("Error with permission");
    }

    #[test]
    fn unauthorized() {
        let message = Message {
            id: NonEmptyString::from_str("example-message-id").unwrap(),
            resource_type: NonEmptyString::from_str("message").unwrap(),
            owner: admin_group().id,
            policies: vec![Policy {
                duration: NonZeroU64::new(30).unwrap(),
                auth_mode: AuthorizationMode::SingleGroup,
                groups_ids: Some(vec![admin_group().id]),
                permissions: Permission::crud(),
            }],
        };

        let invalid_user = regular_user();
        let builder = AuthorizationBuilder::new(&message);
        let _auth = builder
            .build(&invalid_user)
            .expect_err("Authorization should not be able to be created");
        let auth = builder.build(&admin_user()).expect("Error building auth");

        auth.check(&message.id.to_string(), &invalid_user, Permission::Read)
            .expect_err("The user should not be able to read the resource");
    }


    #[test]
    fn multi_permissions() {
        let user = regular_user();
        let message = Message {
            id: NonEmptyString::from_str("example-message-id").unwrap(),
            resource_type: NonEmptyString::from_str("message").unwrap(),
            owner: user.id(),
            policies: vec![Policy {
                duration: NonZeroU64::new(6).unwrap(),
                auth_mode: AuthorizationMode::Owner,
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

    #[test]
    fn multi_group() {
        let message = Message {
            id: NonEmptyString::from_str("example-message-id").unwrap(),
            resource_type: NonEmptyString::from_str("message").unwrap(),
            owner: admin_group().id,
            policies: vec![Policy {
                duration: NonZeroU64::new(30).unwrap(),
                auth_mode: AuthorizationMode::SingleGroup,
                groups_ids: Some(vec![
                    NonEmptyString::from_str("other.group.id").unwrap(),
                    NonEmptyString::from_str("2.group.id").unwrap(),
                    NonEmptyString::from_str("3.group.id").unwrap(),
                    admin_group().id,
                ]),
                permissions: Permission::crud(),
            }],
        };
        let mut reader_user = admin_user();
        reader_user.groups.push(NonEmptyString::from_str("other.group.id").unwrap());
        reader_user.groups.push(NonEmptyString::from_str("2.group.id").unwrap());
        reader_user.groups.push(NonEmptyString::from_str("3.group.id").unwrap());

        AuthorizationBuilder::new(&message)
            .build(&reader_user)
            .expect("Error building auth")
            .search_permission(Permission::Read)
            .expect("Error with permission");
    }

    fn owner_single_group() {
        let message = Message {
            id: NonEmptyString::from_str("example-message-id").unwrap(),
            resource_type: NonEmptyString::from_str("message").unwrap(),
            owner: admin_group().id,
            policies: vec![Policy {
                duration: NonZeroU64::new(30).unwrap(),
                auth_mode: AuthorizationMode::OwnerSingleGroup,
                groups_ids: Some(vec![
                    admin_group().id,
                ]),
                permissions: vec![Permission::Read],
            }],
        };
        let reader_user = admin_user();
         AuthorizationBuilder::new(&message)
            .build(&reader_user)
            .expect("Error building auth")
            .search_permission(Permission::Read)
            .expect("Error with permission");
    }

    fn owner_multi_group() {
        let message = Message {
            id: NonEmptyString::from_str("example-message-id").unwrap(),
            resource_type: NonEmptyString::from_str("message").unwrap(),
            owner: admin_group().id,
            policies: vec![Policy {
                duration: NonZeroU64::new(30).unwrap(),
                auth_mode: AuthorizationMode::OwnerMultiGroup,
                groups_ids: Some(vec![
                    NonEmptyString::from_str("other.group.id").unwrap(),
                    admin_group().id,
                ]),
                permissions: vec![Permission::Read],
            }],
        };
        let mut reader_user = admin_user();
        reader_user.groups.push(NonEmptyString::from_str("other.group.id").unwrap());

        AuthorizationBuilder::new(&message)
            .build(&reader_user)
            .expect("Error building auth")
            .search_permission(Permission::Read)
            .expect("Error with permission");
    }

}

#[cfg(feature = "jwt")]
#[cfg(not(feature = "custom_authorization"))]
#[cfg(test)]
mod jwt_test {
    use crate::actor::Actor;
    use crate::authorization::{Authorization, AuthorizationMode, Permission, Policy};
    use crate::errors::MinosError;
    use crate::jwt::{AuthorizationClaims, TokenServer};
    use crate::prelude::Resource;
    use crate::NonEmptyString;
    use chrono::Utc;
    use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header};
    use std::num::NonZeroU64;

    struct Foo;

    impl Resource for Foo {
        fn id(&self) -> NonEmptyString {
            NonEmptyString::from_str("resource-id").unwrap()
        }

        fn owner(&self) -> Option<NonEmptyString> {
            NonEmptyString::from_str("user-id")
        }

        fn policies(&self) -> Vec<Policy> {
            vec![Policy::new(NonZeroU64::new(1).unwrap(), AuthorizationMode::Owner, None, Permission::crud())]
        }

        fn resource_type(&self) -> Option<NonEmptyString> {
            NonEmptyString::from_str("foo")
        }
    }

    #[test]
    fn authorization_as_claims() -> Result<(), MinosError> {
        let resource = Foo;

        let auth = Authorization {
            permissions: Permission::crud(),
            agent_id: resource.owner().unwrap(),
            resource_id: resource.id(),
            resource_type: resource.resource_type(),
            expiration: Utc::now().timestamp() as u64,
        };

        let generate_claims = AuthorizationClaims::from(auth.clone());
        let generated_auth = generate_claims
            .as_authorization()
            .expect("Error creating authorization from claims");

        assert_eq!(&auth, &generated_auth);
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

        let auth_claims = AuthorizationClaims::new(
            vec![Permission::Read.to_string(), Permission::Update.to_string()],
            NonEmptyString::from_str("user-id").unwrap(),
            NonEmptyString::from_str("resource-id").unwrap(),
            "example.resource".to_string(),
            (Utc::now().timestamp() + 30) as u64,
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
#[cfg(not(feature = "custom_authorization"))]
#[cfg(test)]
mod toml_test {
    use crate::actor::Actor;
    use crate::authorization::{Authorization, AuthorizationMode, Permission, Policy};
    use crate::errors::MinosError;
    use crate::resources::AsResource;
    use crate::resources::Resource;
    use crate::toml::{StoredManifest, TomlFile};
    use crate::NonEmptyString;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::num::NonZeroU64;

    static FILE_CONTENT: &str = r#"
    resource_type = "example resource"
    owner = true

    [[policies]]
    duration = 120
    auth_mode = "owner"
    permissions = ["create", "read", "update", "delete"]

    [[policies]]
    duration = 300
    auth_mode = "single group"
    groups_ids = ["example-group-id-1", "example-group-id-2"]
    permissions = ["read"]"#;

    fn create_temp_file(content: &str) -> Result<PathBuf, MinosError> {
        let mut path = env::temp_dir();
        path.push("example.resource");
        let mut file = File::create(&path)?;

        file.write_all(&content.as_bytes())?;

        Ok(path)
    }

    #[derive(Debug, PartialEq)]
    struct GenericResource {
        pub id: NonEmptyString,
        pub owner: Option<NonEmptyString>,
        pub policies: Vec<Policy>,
        pub resource_type: Option<NonEmptyString>,
    }

    impl Resource for GenericResource {
        fn id(&self) -> NonEmptyString {
            self.id.clone()
        }

        fn owner(&self) -> Option<NonEmptyString> {
            self.owner.clone()
        }

        fn policies(&self) -> Vec<Policy> {
            self.policies.clone()
        }

        fn resource_type(&self) -> Option<NonEmptyString> {
            self.resource_type.clone()
        }
    }

    struct ResourceBuilder {
        pub stored_resource: StoredManifest,
        pub id: NonEmptyString,
        pub owner: Option<NonEmptyString>,
    }

    impl AsResource<GenericResource> for ResourceBuilder {
        type Error = MinosError;
        fn as_resource(&mut self) -> Result<GenericResource, MinosError> {
            Ok(GenericResource {
                id: self.id.clone(),
                owner: self.owner.clone(),
                policies: self.stored_resource.policies(),
                resource_type: self.stored_resource.resource_type(),
            })
        }
    }

    fn resource_from_toml_file(
        file: TomlFile,
        id: NonEmptyString,
        owner: Option<NonEmptyString>,
    ) -> Result<GenericResource, MinosError> {
        let stored_resource = StoredManifest::try_from(file)?;
        let mut builder = ResourceBuilder {
            stored_resource,
            id,
            owner,
        };

        builder.as_resource()
    }

    #[test]
    fn resource_by_file() -> Result<(), MinosError> {
        let path = create_temp_file(FILE_CONTENT)?;
        let toml_file = TomlFile::try_from(&path)?;
        let resource = resource_from_toml_file(
            toml_file,
            NonEmptyString::from_str("example-resource-id").unwrap(),
            NonEmptyString::from_str("example-user-id"),
        )?;

        println!("{:#?}", resource);

        Ok(())
    }

    #[test]
    fn file_by_resource() -> Result<(), MinosError> {
        let mut path = env::temp_dir();
        path.push("ref.resource");
        let resource = GenericResource {
            id: NonEmptyString::from_str("example-resource-id").unwrap(),
            owner: NonEmptyString::from_str("example-user-id"),
            resource_type: NonEmptyString::from_str("example resource"),
            policies: vec![
                Policy::new(NonZeroU64::new(120).unwrap(), AuthorizationMode::Owner, None, Permission::crud()),
                Policy::new(
                    NonZeroU64::new(300).unwrap(),
                    AuthorizationMode::SingleGroup,
                    Some(vec![
                        NonEmptyString::from_str("example-group-id-1").unwrap(),
                        NonEmptyString::from_str("example-group-id-2").unwrap(),
                    ]),
                    vec![Permission::Read],
                ),
            ],
        };

        TomlFile::create(&resource, &path)?;
        let path = create_temp_file(FILE_CONTENT)?;
        let toml_file = TomlFile::try_from(&path)?;
        let saved_resource = resource_from_toml_file(
            toml_file,
            NonEmptyString::from_str("example-resource-id").unwrap(),
            NonEmptyString::from_str("example-user-id"),
        )?;

        assert_eq!(resource, saved_resource);

        Ok(())
    }
}
