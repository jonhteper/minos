use std::collections::HashMap;
use std::env;

use pest::Parser;

use crate::authorization::{self, Actor, Authorizator};
use crate::minos::container::Container;
use crate::minos::environment::Environment;
use crate::minos::file::File;
use crate::minos::parser::tokens::{FileVersion, SingleValueAttribute, SingleValueOperator, Token};
use crate::minos::parser::v0_14::{MinosParserV0_14, Rule};
use crate::minos::policy::Policy;
use crate::minos::requirements::Requirement;
use crate::minos::resource::Resource;
use crate::minos::rule;
use crate::{errors::MinosResult, minos::parser::MinosParser};

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
        vec![rule::Rule::new(vec![Requirement::SingleValue {
            attribute: SingleValueAttribute::Type,
            operator: SingleValueOperator::Equal,
            value: "RootUser".to_string(),
        }])],
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
    let authorizator = Authorizator::new(&envs);

    let actor = Actor::new(
        "RootUser".to_string(),
        "actor.id".to_string(),
        vec![],
        vec![],
    );

    let product = authorization::Resource::new(
        "Product".to_string(),
        Some("example.product.id".to_string()),
    );

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
