use std::borrow::Cow;
use std::collections::HashMap;
use std::env;

use pest::Parser;

use crate::engine::container::Container;
use crate::engine::{self, Actor, AsActor, Engine, IntoActor, IntoResource};
use crate::errors::MinosResult;
use crate::language::environment::Environment;
use crate::language::file::File;
use crate::language::policy::Policy;
use crate::language::requirements::{AttributesComparationRequirement, SingleValueRequirement};
use crate::language::resource::Resource;
use crate::language::rule;
use crate::parser::tokens::{
    ActorSingleValueAttribute, FileVersion, ResourceAttribute, SingleValueOperator, Token,
};
use crate::parser::v0_14::{MinosParserV0_14, Rule};
use crate::parser::v0_15::MinosParserV0_15;
use crate::parser::MinosParser;

const V0_14_MINOS_CONTENT: &str = r#"
sintaxis=0.14;

env TestEnv {
    resource Product {
        policy {
            allow = ["create", "delete"];

            rule {
                actor.type = RootUser;
            }
        }
    }
}
"#;

const V0_15_MINOS_CONTENT: &str = r#"
sintaxis=0.15;

env TestEnv {
    resource User {
        policy {
            allow = ["create", "delete"];

            rule {
                actor.type = resource.type;
            }
        }
    }
}
"#;

#[test]
pub fn parser_test() -> MinosResult<()> {
    let pairs = MinosParserV0_14::parse(Rule::file, &V0_14_MINOS_CONTENT)?
        .next()
        .unwrap();
    let file_token = MinosParserV0_14::parse_token(pairs)?;

    match file_token {
        Token::File(_) => {}
        _ => panic!("Expect Token::File"),
    }

    Ok(())
}

fn file_builtin() -> File {
    let policy = Policy::new(
        vec!["create".to_string(), "delete".to_string()],
        vec![rule::Rule::new(vec![SingleValueRequirement::new(
            ActorSingleValueAttribute::Type,
            SingleValueOperator::Equal,
            "RootUser".to_string(),
        )
        .into()])],
    );
    let resource = Resource::new("Product".to_string(), None, vec![policy]);
    let mut resources = HashMap::new();
    resources.insert(resource.name().clone(), resource);
    let environment = Environment::new("TestEnv".to_string(), resources, HashMap::new());
    let mut environments = HashMap::new();
    environments.insert(environment.name().clone(), environment);

    File::new(FileVersion::V0_14, environments)
}

#[test]
fn file_from_tokens_works() -> MinosResult<()> {
    let file_builtin = file_builtin();
    let file_parsed = MinosParserV0_14::parse_file_content(V0_14_MINOS_CONTENT)?;

    assert_eq!(file_builtin.environments(), &file_parsed);

    Ok(())
}

#[test]
fn parse_file_works() -> MinosResult<()> {
    let mut path = env::current_dir()?;
    path.push("assets/test.minos");

    let _environments = MinosParser::parse_file(&path)?;

    Ok(())
}

#[test]
fn parse_dir_works() -> MinosResult<()> {
    let mut path = env::current_dir()?;
    path.push("assets");

    let _environments = MinosParser::parse_dir(&path)?;

    Ok(())
}

#[test]
fn authorizator_works() -> MinosResult<()> {
    let envs = MinosParser::parse_str(FileVersion::V0_14, V0_14_MINOS_CONTENT)?;
    let authorizator = Engine::new(&envs);

    let actor = Actor::new(
        Cow::from("RootUser"),
        Cow::from("actor.id"),
        Cow::from(vec![]),
        Cow::from(vec![]),
    );

    let product =
        engine::Resource::new(Cow::from("Product"), Some(Cow::from("example.product.id")));

    let permissions = authorizator.authorize(&"TestEnv".to_string(), &actor, &product)?;
    assert_eq!(
        permissions,
        vec!["create".to_string(), "delete".to_string()]
    );

    Ok(())
}

#[test]
fn container_works() -> MinosResult<()> {
    let mut path = env::current_dir()?;
    path.push("assets");

    let container = Container::new(
        "TestContainer".to_string(),
        "Container used in tests".to_string(),
        vec![path],
    )
    .load()?;

    assert!(container.environments().len() > 0);

    Ok(())
}

fn v0_15_file_builtin() -> File {
    let policy = Policy::new(
        vec!["create".to_string(), "delete".to_string()],
        vec![rule::Rule::new(vec![
            AttributesComparationRequirement::new(
                ActorSingleValueAttribute::Type,
                SingleValueOperator::Equal,
                ResourceAttribute::Type,
            )
            .into(),
        ])],
    );
    let resource = Resource::new("User".to_string(), None, vec![policy]);
    let mut resources = HashMap::new();
    resources.insert(resource.name().clone(), resource);
    let environment = Environment::new("TestEnv".to_string(), resources, HashMap::new());
    let mut environments = HashMap::new();
    environments.insert(environment.name().clone(), environment);

    File::new(FileVersion::V0_14, environments)
}

#[test]
fn v0_15_file_from_tokens_works() -> MinosResult<()> {
    let file_builtin = v0_15_file_builtin();
    let file_parsed = MinosParserV0_15::parse_file_content(V0_15_MINOS_CONTENT)?;

    assert_eq!(file_builtin.environments(), &file_parsed);

    Ok(())
}

#[test]
fn attributes_comparation_rules_works() -> MinosResult<()> {
    let envs = MinosParser::parse_str(FileVersion::V0_15, V0_15_MINOS_CONTENT)?;
    let auth = Engine::new(&envs);

    let resource = engine::Resource::new(Cow::from("User"), Some(Cow::from("Example.Id")));
    let actor = Actor::new(
        Cow::from("User"),
        Cow::from("Example.Id"),
        Cow::from(vec![]),
        Cow::from(vec![]),
    );

    auth.find_permissions(
        &"TestEnv".to_string(),
        &actor,
        &resource,
        &vec!["create".to_string(), "delete".to_string()],
    )?;

    Ok(())
}

#[derive(Debug, Clone)]
struct User {
    pub id: String,
    pub roles: Vec<String>,
}

impl AsActor for User {
    fn as_actor(&self) -> Actor {
        Actor::new(
            Cow::from("RootUser"),
            Cow::from(&self.id),
            Cow::from(vec![]),
            Cow::from(&self.roles),
        )
    }
}

impl IntoResource for User {
    fn into_resource<'a>(self) -> engine::Resource<'a> {
        engine::Resource::new(Cow::from("RootUser"), Some(Cow::from(self.id)))
    }
}

#[test]
fn actor_traits_tests() -> MinosResult<()> {
    let user = User {
        id: "UserId".to_string(),
        roles: vec!["SpecialUser".to_string()],
    };

    let envs = MinosParser::parse_str(FileVersion::V0_15, V0_15_MINOS_CONTENT)?;
    let auth = Engine::new(&envs);

    auth.find_permission(
        &"TestEnv".to_string(),
        &user.as_actor(),
        &user.clone().into_resource(),
        &"delete".to_string(),
    )?;



    Ok(())
}
