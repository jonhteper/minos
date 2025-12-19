use std::{borrow::Cow, collections::HashMap, marker::PhantomData, sync::LazyLock};

use parse_display::{Display, FromStr};

use crate::{
    engine::{AsActor, AsResource, FindPermissionRequest},
    Actor,
};

const MINOS_FILE_REFERENCE: &str = include_str!("../../assets/v0_17.minos");

#[test]
fn v0_17_works() {
    crate::MinosParser::easy_parse_str(MINOS_FILE_REFERENCE).expect("Error decoding MINOS file");
}

static MINOS_ENGINE: LazyLock<crate::Engine> = LazyLock::new(|| {
    crate::MinosParser::parse_str(MINOS_FILE_REFERENCE).expect("Error decoding MINOS file")
});

#[derive(Debug, Display, FromStr, Copy, Clone, Eq, PartialEq)]
enum FileStatus {
    Created,
    InTrash,
}

#[derive(Debug, Clone)]
struct File {
    name: &'static str,
    owner: Cow<'static, str>,
    status: FileStatus,
}

impl AsResource for File {
    fn as_resource(&self) -> crate::Resource {
        crate::Resource {
            id: Some(self.name.to_string()),
            type_: "File".into(),
            owner: Some(self.owner.to_string()),
            status: Some(self.status.to_string()),
        }
    }
}

struct FileSystem {
    authorizator: &'static crate::Engine<'static>,
    files: HashMap<&'static str, File>,
}

impl FileSystem {
    fn new(authorizator: &'static crate::Engine<'static>) -> Self {
        Self {
            authorizator,
            files: HashMap::new(),
        }
    }

    fn create_file(&mut self, actor: &impl AsActor, name: &'static str) -> Result<File, String> {
        let actor = actor.as_actor();

        let file = File {
            name,
            owner: Cow::Owned(actor.id.clone()),
            status: FileStatus::Created,
        };

        let resource = file.as_resource();

        let has_permission = self
            .authorizator
            .actor_has_permission(FindPermissionRequest {
                env_name: None,
                actor: &actor,
                resource: &resource,
                permission: "create".to_string(),
            })
            .map_err(|e| e.to_string())?;

        if !has_permission {
            return Err("Permission denied")?;
        }

        self.files.insert(name, file.clone());

        Ok(file)
    }

    fn read_file(&self, actor: &impl AsActor, name: &'static str) -> Result<File, String> {
        let actor = actor.as_actor();

        let file = self.files.get(name).ok_or("File not found")?;

        let resource = file.as_resource();

        let has_permission = self
            .authorizator
            .actor_has_permission(FindPermissionRequest {
                env_name: None,
                actor: &actor,
                resource: &resource,
                permission: "read".to_string(),
            })
            .map_err(|e| e.to_string())?;

        if !has_permission {
            return Err("Permission denied")?;
        }

        if file.status == FileStatus::InTrash {
            return Err("File is in trash")?;
        }

        Ok(file.clone())
    }

    fn delete_file(&mut self, actor: &impl AsActor, name: &'static str) -> Result<(), String> {
        let actor = actor.as_actor();

        let file = self.files.get(name).ok_or("File not found")?;

        let resource = file.as_resource();

        let has_permission = self
            .authorizator
            .actor_has_permission(FindPermissionRequest {
                env_name: None,
                actor: &actor,
                resource: &resource,
                permission: "delete".to_string(),
            })
            .map_err(|e| e.to_string())?;

        if !has_permission {
            return Err("Permission denied")?;
        }

        self.files.remove(name);

        Ok(())
    }

    fn trash_file(&mut self, actor: &impl AsActor, name: &'static str) -> Result<(), String> {
        let actor = actor.as_actor();

        let mut file = self.files.get(name).ok_or("File not found")?.clone();

        let resource = file.as_resource();

        let has_permission = self
            .authorizator
            .actor_has_permission(FindPermissionRequest {
                env_name: None,
                actor: &actor,
                resource: &resource,
                permission: "trash".to_string(),
            })
            .map_err(|e| e.to_string())?;

        if !has_permission {
            return Err("Permission denied")?;
        }

        file.status = FileStatus::InTrash;

        self.files.insert(name, file);

        Ok(())
    }
}

struct Guest;
struct Regular;
struct Root;

struct User<Ty = Regular> {
    name: &'static str,
    groups: Vec<String>,
    roles: Vec<String>,
    is_banned: bool,
    _ty: PhantomData<Ty>,
}

impl User {
    fn new(name: &'static str, groups: Vec<String>, roles: Vec<String>) -> Self {
        User {
            name,
            groups,
            roles,
            is_banned: false,
            _ty: PhantomData,
        }
    }

    fn new_guest() -> User<Guest> {
        User {
            name: "Guest",
            groups: vec![],
            roles: vec![],
            is_banned: false,
            _ty: PhantomData,
        }
    }

    fn new_root() -> User<Root> {
        User {
            name: "Root",
            groups: vec![],
            roles: vec![],
            is_banned: false,
            _ty: PhantomData,
        }
    }

    fn ban(&mut self) {
        self.is_banned = true;
    }

    fn unban(&mut self) {
        self.is_banned = false;
    }
}

impl AsActor for User<Regular> {
    fn as_actor(&self) -> crate::Actor {
        let status = if self.is_banned {
            Some("Banned".to_string())
        } else {
            Some("Active".to_string())
        };

        crate::Actor {
            id: self.name.to_string(),
            type_: "User".to_string(),
            status,
            groups: self.groups.clone(),
            roles: self.roles.clone(),
        }
    }
}

impl AsActor for User<Guest> {
    fn as_actor(&self) -> crate::Actor {
        crate::Actor {
            id: self.name.to_string(),
            type_: "Guest".to_string(),
            status: Some("Active".to_string()),
            groups: vec![],
            roles: vec![],
        }
    }
}

impl AsActor for User<Root> {
    fn as_actor(&self) -> crate::Actor {
        crate::Actor {
            id: self.name.to_string(),
            type_: "Root".to_string(),
            status: Some("Active".to_string()),
            groups: vec![],
            roles: vec![],
        }
    }
}

#[test]
fn auth_process_works() {
    let mut file_system = FileSystem::new(&MINOS_ENGINE);

    let mut user = User::new("Jonh Doe", vec!["sudo".to_string()], vec![]);

    let file = file_system
        .create_file(&user, "hello world.txt")
        .expect("Error creating file");

    user.roles.push("supervised".to_string());

    file_system
        .delete_file(&user, "hello world.txt")
        .expect("Error deleting file");
}
