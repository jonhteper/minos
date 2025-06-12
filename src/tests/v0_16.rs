use std::{env, sync::LazyLock};

use anyhow::anyhow;
use chrono::Utc;
use parse_display_derive::{Display, FromStr};

use crate::{
    engine::{
        Actor, AsActor, AsResource, AuthorizeRequest, Engine, FindPermissionRequest,
        FindPermissionsRequest, Resource, TryIntoActor,
    },
    language::{environment::DEFAULT_ENV_IDENTIFIER, storage::Storage},
    text_repr::to_text_repr::ToTextRepr,
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
    let storage = MinosParser::easy_parse_str(MINOS_V0_16_FILE_CONTENT)?;
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

#[derive(Debug, Display, FromStr, Clone, Copy)]
enum SimplePermissions {
    #[display("create")]
    Create,

    #[display("read")]
    Read,

    #[display("update")]
    Update,

    #[display("delete")]
    Delete,

    #[display("install")]
    Install,

    #[display("uninstall")]
    Uninstall,

    #[display("execute")]
    Execute,
}

/// Test to verify that the authorization works correctly.
#[test]
fn simple_authorize_works() -> MinosResult<()> {
    let storage = MinosParser::easy_parse_str(MINOS_V0_16_FILE_CONTENT)?;
    let user = Actor {
        id: "Example.user.id".into(),
        type_: "User".into(),
        groups: vec![],
        roles: vec![],
        status: None,
    };
    let resource = Resource {
        id: Some("Example.user.id".into()),
        type_: "User".into(),
        owner: None,
        status: None,
    };
    let engine = Engine::new(&storage);
    let permissions = engine.authorize(AuthorizeRequest {
        env_name: None,
        resource: &resource,
        actor: &user,
    })?;

    assert_eq!(
        permissions.as_ref(),
        &[
            SimplePermissions::Create.to_string(),
            SimplePermissions::Read.to_string(),
            SimplePermissions::Update.to_string(),
            SimplePermissions::Delete.to_string(),
        ]
    );

    Ok(())
}

#[test]
fn simple_find_permission_works() {
    let storage = MinosParser::easy_parse_str(MINOS_V0_16_FILE_CONTENT).unwrap();
    let user = Actor {
        id: "Example.user.id".into(),
        type_: "User".into(),
        groups: vec![],
        roles: vec![],
        status: None,
    };
    let resource = Resource {
        id: Some("Example.user.id".into()),
        type_: "User".into(),
        owner: None,
        status: None,
    };
    let engine = Engine::new(&storage);
    let result = engine.actor_has_permission(FindPermissionRequest {
        env_name: None,
        resource: &resource,
        actor: &user,
        permission: SimplePermissions::Create.to_string(),
    });

    assert!(result.is_ok());
}

#[test]
fn simple_find_permissions_works() {
    let storage = MinosParser::easy_parse_str(MINOS_V0_16_FILE_CONTENT).unwrap();
    let user = Actor {
        id: "Example.user.id".into(),
        type_: "User".into(),
        groups: vec![],
        roles: vec![],
        status: None,
    };
    let resource = Resource {
        id: Some("Example.user.id".into()),
        type_: "User".into(),
        owner: None,
        status: None,
    };
    let engine = Engine::new(&storage);
    let result = engine.actor_has_permissions(FindPermissionsRequest {
        env_name: None,
        resource: &resource,
        actor: &user,
        permissions: vec![
            SimplePermissions::Create.to_string(),
            SimplePermissions::Read.to_string(),
        ],
    });

    assert!(result.is_ok());
}

const ADVANCED_MINOS_V_0_16_TEXT: &str = include_str!("../../assets/simulation/simulation_v0_16.minos");

static STORAGE_V_0_16: LazyLock<Storage> =
    LazyLock::new(|| MinosParser::easy_parse_str(ADVANCED_MINOS_V_0_16_TEXT).unwrap());
static ENGINE_V0_16: LazyLock<Engine<'static>> = LazyLock::new(|| Engine::new(&STORAGE_V_0_16));

#[test]
fn file_simulation_works() -> MinosResult<()> {
    let user2 = Actor {
        id: "user2".into(),
        type_: "User".into(),
        groups: vec!["File".into()],
        roles: vec![],
        status: None,
    };
    let config_file = Resource {
        id: Some("app.conf".into()),
        type_: "File".into(),
        owner: Some("user1".into()),
        status: None,
    };

    let operation_result = ENGINE_V0_16.actor_has_permission(FindPermissionRequest {
        env_name: None,
        actor: &user2,
        resource: &config_file,
        permission: SimplePermissions::Read.to_string(),
    });
    assert_eq!(operation_result, Ok(true));

    let user1 = Actor {
        id: "user1".into(),
        type_: "User".into(),
        groups: vec![],
        roles: vec!["admin".into()],
        status: None,
    };
    let operation_result = ENGINE_V0_16.actor_has_permission(FindPermissionRequest {
        env_name: None,
        actor: &user1,
        resource: &config_file,
        permission: SimplePermissions::Delete.to_string(),
    });
    assert_eq!(operation_result, Ok(true));

    let guest_user = Actor {
        id: "GUEST.USER".into(),
        type_: "User".into(),
        groups: vec![],
        roles: vec!["guest".into()],
        status: None,
    };
    let operation_result = ENGINE_V0_16.actor_has_permission(FindPermissionRequest {
        env_name: None,
        actor: &guest_user,
        resource: &config_file,
        permission: SimplePermissions::Read.to_string(),
    });
    assert_eq!(operation_result, Ok(false));

    let operation_result = ENGINE_V0_16.actor_has_permission(FindPermissionRequest {
        env_name: Some("TEST"),
        actor: &guest_user,
        resource: &config_file,
        permission: SimplePermissions::Delete.to_string(),
    });
    assert_eq!(operation_result, Ok(true));

    let permissions = &ENGINE_V0_16.authorize(AuthorizeRequest {
        env_name: Some("TEST"),
        actor: &guest_user,
        resource: &config_file,
    })?;
    assert_eq!(permissions.len(), 4);

    Ok(())
}

struct User {
    pub id: String,
    pub is_sudoer: bool,
    pub roles: Vec<String>,
}

impl User {
    fn sudo(&self) -> Result<SuperUser, &'static str> {
        if !self.is_sudoer {
            Err("the user cannot be a superuser")?;
        }

        Ok(SuperUser {
            id: self.id.clone(),
            valid_until: Utc::now().timestamp() + 2000,
        })
    }
}

impl AsActor for User {
    fn as_actor(&self) -> Actor {
        Actor {
            id: self.id.clone(),
            type_: "User".to_string(),
            groups: vec![],
            roles: self.roles.clone(),
            status: None,
        }
    }
}

struct SuperUser {
    pub id: String,
    pub valid_until: i64,
}

impl TryIntoActor for SuperUser {
    type Error = &'static str;
    fn try_into_actor(self) -> Result<Actor, Self::Error> {
        if self.valid_until < Utc::now().timestamp() {
            Err("the superuser has expired")?;
        }

        Ok(Actor {
            id: self.id.to_string(),
            type_: "SuperUser".to_string(),
            groups: vec![],
            roles: vec![],
            status: None,
        })
    }
}

struct Application {
    pub id: String,
    pub name: String,
    pub path: String,
    pub executing_environment: String,
    pub is_installed: bool,
}

impl Application {
    fn install(&mut self, minos_engine: &Engine, actor: &Actor) -> anyhow::Result<()> {
        if !minos_engine.actor_has_permission(FindPermissionRequest {
            env_name: Some(&self.executing_environment),
            actor,
            resource: &self.as_resource(),
            permission: SimplePermissions::Install.to_string(),
        })? {
            Err(anyhow!(
                "the actor does not have the permission to install {}",
                &self.name
            ))?
        }
        println!("Installing application {} in {}...", &self.name, &self.path);
        self.is_installed = true;

        Ok(())
    }

    fn execute(&self, minos_engine: &Engine, actor: &Actor) -> anyhow::Result<()> {
        if !minos_engine.actor_has_permission(FindPermissionRequest {
            env_name: Some(&self.executing_environment),
            actor,
            resource: &self.as_resource(),
            permission: SimplePermissions::Execute.to_string(),
        })? {
            Err(anyhow!(
                "the actor does not have permission to execute {}",
                &self.name
            ))?
        }
        println!("Executing application {}...", &self.name);

        Ok(())
    }

    fn uninstall(&mut self, minos_engine: &Engine, actor: &Actor) -> anyhow::Result<()> {
        if !minos_engine.actor_has_permission(FindPermissionRequest {
            env_name: Some(&self.executing_environment),
            actor,
            resource: &self.as_resource(),
            permission: SimplePermissions::Uninstall.to_string(),
        })? {
            Err(anyhow!(
                "the actor does not have permission to uninstall {}",
                &self.name
            ))?
        }
        println!("Uninstalling application {} from {}...", &self.name, &self.path);
        self.is_installed = false;

        Ok(())
    }

    fn update(&self, minos_engine: &Engine, actor: &Actor) -> anyhow::Result<()> {
        if !minos_engine.actor_has_permission(FindPermissionRequest {
            env_name: Some(&self.executing_environment),
            actor,
            resource: &self.as_resource(),
            permission: SimplePermissions::Update.to_string(),
        })? {
            Err(anyhow!(
                "the actor does not have permission to update {}",
                &self.name
            ))?
        }
        println!("Updating application {}...", &self.name);

        Ok(())
    }
}

impl AsResource for Application {
    fn as_resource(&self) -> Resource {
        Resource {
            id: Some(self.id.as_str().into()),
            type_: "Application".into(),
            owner: Some("OS".into()),
            status: match self.is_installed {
                true => Some("installed".into()),
                false => Some("no-installed".into()),
            },
        }
    }
}

#[test]
fn application_simulation_works() {
    let john_user = User {
        id: "John".to_string(),
        is_sudoer: true,
        roles: vec![],
    };

    let jane_user = User {
        id: "Jane".to_string(),
        is_sudoer: false,
        roles: vec!["application manager".to_string()],
    };

    let mut chromium = Application {
        id: "app.chromium".to_string(),
        name: "Chromium".to_string(),
        path: "/usr/bin/chromium".to_string(),
        executing_environment: "STD".to_string(),
        is_installed: true,
    };

    let operation_result = chromium.execute(&ENGINE_V0_16, &john_user.as_actor());
    assert!(operation_result.is_ok());

    let mut firefox = Application {
        id: "app.firefox".to_string(),
        name: "Firefox".to_string(),
        path: "/usr/bin/firefox".to_string(),
        executing_environment: "STD".to_string(),
        is_installed: false,
    };

    let operation_result = firefox.install(&ENGINE_V0_16, &jane_user.as_actor());
    assert!(operation_result.is_ok());
    let operation_result = firefox.execute(&ENGINE_V0_16, &jane_user.as_actor());
    assert!(operation_result.is_ok());

    chromium.executing_environment = "ROOT".to_string();
    let john_sudo = john_user.sudo().unwrap();
    let operation_result = chromium.uninstall(&ENGINE_V0_16, &john_sudo.try_into_actor().unwrap());
    dbg!(&operation_result);
    assert!(operation_result.is_ok());

    let mut application_store = Application {
        id: "app.application-store".to_string(),
        name: "Market".to_string(),
        path: "/usr/bin/store".to_string(),
        executing_environment: "STD".to_string(),
        is_installed: true,
    };

    let operation_result = application_store.uninstall(&ENGINE_V0_16, &jane_user.as_actor());
    dbg!(&operation_result);
    assert!(operation_result.is_err());

    let john_sudo = john_user.sudo().unwrap();
    let operation_result = application_store.update(&ENGINE_V0_16, &john_sudo.try_into_actor().unwrap());
    assert!(operation_result.is_ok());

    let operation_result = application_store.execute(&ENGINE_V0_16, &john_user.as_actor());
    assert!(operation_result.is_ok());
}

const FORMATTED_MINOS_FILE: &str = include_str!("../../assets/v0_16.formatted.minos");

#[test]
fn text_repr_works() -> MinosResult<()> {
    let storage = MinosParser::easy_parse_str(FORMATTED_MINOS_FILE)?;
    let text_repr = storage.to_text_repr();

    println!("{}", text_repr);
    assert_eq!(FORMATTED_MINOS_FILE, &text_repr);

    Ok(())
}
