use std::env;

use crate::{
    language::environment::DEFAULT_ENV_IDENTIFIER, parser::tokens::FileVersion, Container, MinosParser,
    MinosResult,
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
    assert_eq!(rules.len(), 2);
    
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
