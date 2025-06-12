use std::sync::LazyLock;

use anyhow::anyhow;

use crate::{
    engine::{AsActor, AsResource, FindPermissionRequest},
    language::storage::Storage,
    Actor, Engine, MinosParser,
};

const ADVANCED_MINOS_FILE_CONTENT: &str =
    include_str!("../../assets/simulation/simulation_v0_16M.minos");

static STORAGE: LazyLock<Storage> =
    LazyLock::new(|| MinosParser::easy_parse_str(ADVANCED_MINOS_FILE_CONTENT).unwrap());
static ENGINE: LazyLock<Engine<'static>> = LazyLock::new(|| Engine::new(&STORAGE));

#[derive(Debug)]
struct User<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub status: &'a str,
    pub roles: Vec<&'a str>,
}

impl<'a> AsActor for User<'a> {
    fn as_actor(&self) -> Actor {
        Actor {
            id: self.id.into(),
            type_: "User".into(),
            status: Some(self.status.into()),
            groups: vec![],
            roles: self.roles.iter().map(|r| r.to_string()).collect(),
        }
    }
}

impl<'a> AsResource for User<'a> {
    fn as_resource(&self) -> crate::Resource {
        crate::Resource {
            id: Some(self.id.into()),
            type_: "User".into(),
            owner: None,
            status: Some(self.status.into()),
        }
    }
}

struct UserValues<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub roles: Vec<&'a str>,
}

impl<'a> User<'a> {
    fn create(
        engine: &Engine,
        actor: &Actor,
        env: &str,
        values: UserValues<'a>,
    ) -> anyhow::Result<Self> {
        let user = User {
            id: values.id,
            name: values.name,
            status: "active".into(),
            roles: values.roles,
        };

        if !engine.actor_has_permission(FindPermissionRequest {
            env_name: Some(env),
            actor,
            resource: &user.as_resource(),
            permission: "create".into(),
        })? {
            Err(anyhow!("actor does not have create permission"))?
        }

        Ok(user)
    }

    fn read_status(&self, engine: &Engine, actor: &Actor, env: &str) -> anyhow::Result<&str> {
        if !engine.actor_has_permission(FindPermissionRequest {
            env_name: Some(env),
            actor,
            resource: &self.as_resource(),
            permission: "read_status".into(),
        })? {
            Err(anyhow!("actor does not have read_status permission"))?
        }

        Ok(&self.status)
    }

    fn update_status(
        &mut self,
        engine: &Engine,
        actor: &Actor,
        env: &str,
        status: &'a str,
    ) -> anyhow::Result<()> {
        if !engine.actor_has_permission(FindPermissionRequest {
            env_name: Some(env),
            actor,
            resource: &self.as_resource(),
            permission: "update_status".into(),
        })? {
            Err(anyhow!("actor does not have update_status permission"))?
        }

        self.status = status;

        Ok(())
    }

    fn delete(&mut self, engine: &Engine, actor: &Actor, env: &str) -> anyhow::Result<()> {
        if !engine.actor_has_permission(FindPermissionRequest {
            env_name: Some(env),
            actor,
            resource: &self.as_resource(),
            permission: "delete".into(),
        })? {
            Err(anyhow!("actor does not have delete permission"))?
        }

        self.id = "";
        self.name = "";
        self.status = "";
        self.roles = vec![];

        Ok(())
    }

    fn sudo(&self, engine: &Engine, env: &str) -> anyhow::Result<SuperUser> {
        if !engine.actor_has_permission(FindPermissionRequest {
            env_name: Some(env),
            actor: &self.as_actor(),
            resource: &self.as_resource(),
            permission: "sudo".into(),
        })? {
            Err(anyhow!("actor cannot be superuser"))?
        }

        Ok(SuperUser { id: self.id })
    }
}

struct SuperUser<'a> {
    id: &'a str,
}

impl<'a> AsActor for SuperUser<'a> {
    fn as_actor(&self) -> Actor {
        Actor {
            id: self.id.into(),
            type_: "SuperUser".into(),
            status: None,
            groups: vec![],
            roles: vec![],
        }
    }
}

#[test]
fn user_simulation_works() -> anyhow::Result<()> {
    let mut jonh_user = User {
        id: "1",
        name: "John Doe",
        status: "Banned",
        roles: vec![],
    };

    let jane_user = User {
        id: "2",
        name: "Jane Green",
        status: "Active",
        roles: vec!["admin"],
    };

    let operation_result = jonh_user.update_status(&ENGINE, &jonh_user.as_actor(), "STD", "Active");
    assert!(operation_result.is_err());

    let operation_result = jonh_user.update_status(&ENGINE, &jane_user.as_actor(), "STD", "Active");
    assert!(operation_result.is_ok());

    let user_status = jonh_user.read_status(&ENGINE, &jonh_user.as_actor(), "STD")?;
    assert_eq!(user_status, "Active");

    let operation_result = User::create(
        &ENGINE,
        &jane_user.as_actor(),
        "STD",
        UserValues {
            id: "3",
            name: "Mary J.",
            roles: vec![],
        },
    );
    assert!(operation_result.is_ok());

    let mut mary_user = operation_result?;
    let super_user = jane_user.sudo(&ENGINE, "STD")?;
    let operation_result = mary_user.delete(&ENGINE, &super_user.as_actor(), "ROOT");
    assert!(operation_result.is_ok());
    assert!(mary_user.id.is_empty());

    Ok(())
}
