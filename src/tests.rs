use std::env;

use lazy_static::lazy_static;

use crate::{
    engine::{Actor, AuthorizeRequest, Engine, FindPermissionRequest, FindPermissionsRequest, Resource},
    language::{environment::DEFAULT_ENV_IDENTIFIER, policy::Permission, storage::Storage},
    parser::tokens::FileVersion,
    Container, MinosParser, MinosResult,
};

const MINOS_V0_16_FILE_CONTENT: &str = r#"
syntax = 0.16;
/* Example Commentary */
resource User {
    env DEFAULT {
        policy {
            allow = ["create", "read", "update", "delete"];

            rule {
                actor.type = RootUser;
            }

            rule {
                actor.type = resource.type;
                actor.id = resource.id;
            }

            rule {
                resource.owner = actor.id;
            }
        }
    }
}

"#;

/// Test to verify that the parsing from file content, works as expected.
#[test]
fn parse_file_content_works() -> MinosResult<()> {
    let storage = MinosParser::parse_str(FileVersion::V0_16, MINOS_V0_16_FILE_CONTENT)?;
    let resources = storage.resources();
    assert_eq!(resources.len(), 1);
    let environments = resources.get(&"User".into()).unwrap().environments();
    assert_eq!(environments.len(), 1);
    let policies = environments
        .get(&DEFAULT_ENV_IDENTIFIER.into())
        .unwrap()
        .policies();
    assert_eq!(policies.len(), 1);
    let rules = policies.first().unwrap().rules();
    assert_eq!(rules.len(), 3);

    Ok(())
}

/// Test to verify that the Container construction works as expected.
#[test]
fn build_container_works() -> MinosResult<()> {
    let mut path = env::current_dir()?;
    path.push("assets");
    let container = Container::new("1".to_string(), "Test Container".to_string(), vec![path]).load()?;

    assert!(container.storage().resources().len() > 0);

    Ok(())
}

/// Test to verify that the authorization works correctly.
#[test]
fn simple_authorize_works() -> MinosResult<()> {
    let storage = MinosParser::parse_str(FileVersion::V0_16, MINOS_V0_16_FILE_CONTENT)?;
    let user = Actor::new("User".into(), "Example.user.id".into(), vec![], vec![]);
    let resource = Resource::new(Some("Example.user.id".into()), "User".into(), None);
    let engine = Engine::new(&storage);
    let permissions = engine.authorize(AuthorizeRequest {
        env_name: None,
        resource: &resource,
        actor: &user,
    })?;

    assert_eq!(
        permissions,
        vec![
            Permission::from("create"),
            Permission::from("read"),
            Permission::from("update"),
            Permission::from("delete")
        ]
    );

    Ok(())
}

#[test]
fn simple_find_permission_works() {
    let storage = MinosParser::parse_str(FileVersion::V0_16, MINOS_V0_16_FILE_CONTENT).unwrap();
    let user = Actor::new("User".into(), "Example.user.id".into(), vec![], vec![]);
    let resource = Resource::new(Some("Example.user.id".into()), "User".into(), None);
    let engine = Engine::new(&storage);
    let result = engine.find_permission(FindPermissionRequest {
        env_name: None,
        resource: &resource,
        actor: &user,
        permission: Permission::from("create"),
    });

    assert!(result.is_ok());
}

#[test]
fn simple_find_permissions_works() {
    let storage = MinosParser::parse_str(FileVersion::V0_16, MINOS_V0_16_FILE_CONTENT).unwrap();
    let user = Actor::new("User".into(), "Example.user.id".into(), vec![], vec![]);
    let resource = Resource::new(Some("Example.user.id".into()), "User".into(), None);
    let engine = Engine::new(&storage);
    let result = engine.find_permissions(FindPermissionsRequest {
        env_name: None,
        resource: &resource,
        actor: &user,
        permissions: &[Permission::from("create"), Permission::from("read")],
    });

    assert!(result.is_ok());
}

const ADVANCED_MINOS_V_0_16_TEXT: &str = include_str!("../assets/simulation/simulation_v0_16.minos");
lazy_static! {
    static ref STORAGE_V_0_16: Storage =
        MinosParser::parse_str(FileVersion::V0_16, ADVANCED_MINOS_V_0_16_TEXT).unwrap();
    static ref ENGINE_V0_16: Engine<'static> = Engine::new(&STORAGE_V_0_16);
}

#[test]
fn file_simulation_works() -> MinosResult<()> {
    let user2 = Actor::new("User".into(), "user2".into(), vec!["File".into()], vec![]);
    let config_file = Resource::new(Some("app.conf".into()), "File".into(), Some("user1".into()));

    let operation_result = &ENGINE_V0_16.find_permission(FindPermissionRequest {
        env_name: None,
        actor: &user2,
        resource: &config_file,
        permission: Permission::from("read"),
    });
    assert!(operation_result.is_ok());

    let user1 = Actor::new("User".into() , "user1".into(), vec![], vec!["admin".into()]);
    let operation_result = &ENGINE_V0_16.find_permission(FindPermissionRequest {
        env_name: None,
        actor: &user1,
        resource: &config_file,
        permission: Permission::from("delete"),
    });
    assert!(operation_result.is_ok());

    let guest_user = Actor::new("User".into(), "GUEST.USER".into(), vec![], vec!["guest".into()]);
    let operation_result = &ENGINE_V0_16.find_permission(FindPermissionRequest {
        env_name: None,
        actor: &guest_user,
        resource: &config_file,
        permission: Permission::from("read"),
    });
    assert!(operation_result.is_err());

    let operation_result = &ENGINE_V0_16.find_permission(FindPermissionRequest {
        env_name: Some("TEST"),
        actor: &guest_user,
        resource: &config_file,
        permission: Permission::from("delete"),
    });
    assert!(operation_result.is_ok());

    let permissions = &ENGINE_V0_16.authorize(AuthorizeRequest {
        env_name: Some("TEST"),
        actor: &guest_user,
        resource: &config_file,
    })?;
    assert_eq!(permissions.len(), 4);

    Ok(())
}
