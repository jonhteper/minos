use std::collections::HashMap;
use std::{env, fs};

use pest::Parser;

use crate::minos::requirements::Requirement;
use crate::minos::{self, rule};
use crate::minos::environment::Environment;
use crate::minos::file::File;
use crate::minos::lang::{Token, FileVersion, SingleValueAttribute, SingleValueOperator};
use crate::minos::parser::v0_14::{MinosParserV0_14, Rule};
use crate::minos::policy::Policy;
use crate::minos::resource::Resource;
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
        vec![
            rule::Rule::new(vec![Requirement::SingleValue { attribute: SingleValueAttribute::Type, operator: SingleValueOperator::Equal, value: "RootUser".to_string() }]),
        ]

    );
    let resource = Resource::new(
        "Product".to_string(),
        None,
        vec![policy]
    );
    let mut resources = HashMap::new();
    resources.insert((resource.name().clone(), resource.id().clone()), resource);
    let environment = Environment::new(
        "TestEnv".to_string(),
        resources,
    );
    let mut environments = HashMap::new();
    environments.insert(environment.name().clone(), environment);
    
    File::new(FileVersion::V0_14, environments)
}

#[test]
fn file_from_tokens_works() -> MinosResult<()> {
    let file_builtin = file_builtin();
    let file_parsed = MinosParserV0_14::parse_file_content(V0_14_MINOS_CONTENT)?;

    assert_eq!(file_builtin, file_parsed);

    Ok(())
}



#[test]
fn parse_file_works() -> MinosResult<()> {
    let mut path = env::current_dir()?;
    path.push("assets/test.minos");

    let file = MinosParser::parse_file(&path)?;

    Ok(())
}
