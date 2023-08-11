use lazy_static::lazy_static;

use crate::{
    engine::{AsActor, AsResource, FindPermissionRequest},
    language::storage::Storage,
    parser::tokens::FileVersion,
    Actor, Engine, MinosParser,
};

const ADVANCED_MINOS_FILE_CONTENT: &str =
    include_str!("../../assets/simulation/simulation_v0_16M.minos");
lazy_static! {
    static ref STORAGE: Storage =
        MinosParser::parse_str(FileVersion::V0_16M, ADVANCED_MINOS_FILE_CONTENT).unwrap();
    static ref ENGINE: Engine<'static> = Engine::new(&STORAGE);
}

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
    ) -> Result<Self, String> {
        let user = User {
            id: values.id,
            name: values.name,
            status: "active".into(),
            roles: values.roles,
        };

        engine
            .has_permission(FindPermissionRequest {
                env_name: Some(env),
                actor,
                resource: &user.as_resource(),
                permission: "create".into(),
            })
            .map_err(|e| e.to_string())?;

        Ok(user)
    }

    fn read_status(&self, engine: &Engine, actor: &Actor, env: &str) -> Result<&str, String> {
        engine
            .has_permission(FindPermissionRequest {
                env_name: Some(env),
                actor,
                resource: &self.as_resource(),
                permission: "read_status".into(),
            })
            .map_err(|e| e.to_string())?;

        Ok(&self.status)
    }

    fn update_status(
        &mut self,
        engine: &Engine,
        actor: &Actor,
        env: &str,
        status: &'a str,
    ) -> Result<(), String> {
        engine
            .has_permission(FindPermissionRequest {
                env_name: Some(env),
                actor,
                resource: &self.as_resource(),
                permission: "update_status".into(),
            })
            .map_err(|e| e.to_string())?;

        self.status = status;

        Ok(())
    }

    fn delete(&mut self, engine: &Engine, actor: &Actor, env: &str) -> Result<(), String> {
        engine
            .has_permission(FindPermissionRequest {
                env_name: Some(env),
                actor,
                resource: &self.as_resource(),
                permission: "delete".into(),
            })
            .map_err(|e| e.to_string())?;

        self.id = "";
        self.name = "";
        self.status = "";
        self.roles = vec![];

        Ok(())
    }

    fn sudo(&self, engine: &Engine, env: &str) -> Result<SuperUser, String> {
        engine
          .has_permission(FindPermissionRequest {
                env_name: Some(env),
                actor: &self.as_actor(),
                resource: &self.as_resource(),
                permission: "sudo".into(),
            })
          .map_err(|e| e.to_string())?;

        Ok(SuperUser {
            id: self.id,
        })
    }
}

struct SuperUser<'a> {
    id: &'a str,
}

impl <'a> AsActor for SuperUser<'a> {
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
fn user_simulation_works() {
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

    let operation_result = jonh_user.read_status(&ENGINE, &jonh_user.as_actor(), "STD");
    assert_eq!(operation_result, Ok("Active"));

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

    let mut mary_user = operation_result.unwrap();
    let super_user = jane_user.sudo(&ENGINE, "STD").expect("jane must be a super user");
    let operation_result = mary_user.delete(&ENGINE, &super_user.as_actor(), "ROOT");
    assert!(operation_result.is_ok());
    assert!(mary_user.id.is_empty());

}
