use crate::model::assertion::Assertion;
use crate::model::attribute::Attribute;
use crate::model::parser::FileParser;
use crate::model::permission::ToPermissions;
use serde_json::Map as JsMap;
use serde_json::Value as JsValue;
use std::str::FromStr;
use toml::map::Map;
use toml::{from_str, Value};

const INPUT: &str = r#"{
        "actor": {
            "id": "actor.id",
            "groups": ["writers", "admins"],
            "status": "active"
        },
        "resource": {
            "id": "resource.id",
            "owner": "actor.id",
            "status": "writed",
            "is_revised": false,
            "resource_type": "blog_post"
        },
        "environment": {
            "in_maintainning_mode": false,
        }
    }"#;

const POLICIES: &str = r#"
syntax_version = "0.10"

[[policies]]
resource_type = "blog_post"

[[policies.rules]]
permissions = ["read:30m"]
resource.status = "published"


[[policies.rules]]
permissions = [
    "read:60m",
    "write:1500ms"
]
actor.groups = ["writers"]
resource.owner = actor.id


[[policies.rules]]
permissions = ["revise:1200ms"]
actor.groups $contains ["admins"]


[[policies.rules]]
permissions = ["publish:1s"]
actor.groups = ["admins"]
actor.status = "active"
resource.status = "writed"
resource.is_revised = true
environment.in_maintainning_mode = false

[[policies.rules]]
permissions = ["example:1200ms"]
actor.deep.key = ["admins"]

[[policies]]
resource_id = "SYSTEM.LOGIN.ID"

[[policies.rules]]
permissions = ["login:1200ms"]
failed_attempts >= environment.max_failed_attempts
    "#;

#[test]
fn authorization_works() {
    /*todo!()*/
}

#[test]
fn attribute_path_from_str_works() {
    const TEXT: &str = "parent.child1.child_two.another";

    let attribute_path = Attribute::from_str(TEXT).expect("Error with attribute path parsing");

    assert_eq!(&attribute_path.to_string(), TEXT);
    println!("{attribute_path:?}");
}

#[test]
fn assertion_from_str_works() {
    /*const TEXT: &str = "actor.failed_attempts >= environment.max_failed_attempts";
    let assertion = Assertion::from_str(TEXT).expect("Error with assertion");
    assert_eq!(&assertion.to_string(), TEXT)*/
}

#[test]
fn parse_policies_from_text() {
    let mut parser = FileParser::new(POLICIES);
    let policies = parser.obtain_policies()
        .expect("Error parsing policies from text");
    println!("{} policies captures", policies.len());
    for policy in policies {
        let _= parser.obtain_rules_from_policy(policy)
            .expect("Error obtaining rules");
    }
}
