use crate::model::assertion::Assertion;
use crate::model::attribute::AttributePath;
use crate::model::parser::JsonParser;
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
        syntax_version = "0.6"

        [[policies]]
        resource_type = "blog_post"

        [[policies.rules]]
        permissions = [["read", "30m"]]
        resource.status = "published"


        [[policies.rules]]
        permissions = [
            ["read", "60m"],
            ["write", "1500ms"]
        ]
        actor.groups = ["writers"]
        by_owner = true


        [[policies.rules]]
        permissions = [["revise", "1200ms"]]
        actor.groups = ["admins"]


        [[policies.rules]]
        permissions = [["publish", "1s"]]
        actor.groups = ["admins"]
        actor.status = "active"
        resource.status = "writed"
        resource.is_revised = true
        environment.in_maintainning_mode = false

        [[policies.rules]]
        permissions = [["login", "1200ms"]]
        assertions = ["actor.failed_attempts >= environment.max_failed_attempts"]
    "#;

#[test]
fn authorization_works() {
    todo!()
}

#[test]
fn attribute_path_from_str_works() {
    const TEXT: &str = "parent.child1.child_two.another";

    let attribute_path = AttributePath::from_str(TEXT).expect("Error with attribute path parsing");

    assert_eq!(&attribute_path.to_string(), TEXT);
    println!("{attribute_path:?}");
}

#[test]
fn assertion_from_str_works() {
    const TEXT: &str = "actor.failed_attempts >= environment.max_failed_attempts";
    let assertion = Assertion::from_str(TEXT).expect("Error with assertion");
    assert_eq!(&assertion.to_string(), TEXT)
}

#[test]
fn parse_toml_file1() {
    let file: Map<String, Value> = from_str(POLICIES).expect("Error decoding policies");

    //println!("{:?}", file);

    let sintaxis = file.get("sintaxis").unwrap().to_string();
    println!("sintaxis: {sintaxis}");

    let policies = file.get("policies").unwrap().as_array().unwrap();

    for policy in policies {
        let policy = policy.as_table().unwrap();
        let rules = policy.get("rules").unwrap().as_array().unwrap();
        for rule in rules {
            let rule = rule.as_table().unwrap();
            let permissions = rule.get("permissions").unwrap().as_array().unwrap();
            println!("permissions: {permissions:?}");
            for (key, value) in rule {
                if key == "permissions" {
                    continue;
                }
                println!("{key}: {value}")
            }
            println!();
        }
    }
}

#[test]
fn parse_toml_file2() {
    let file_str = tsu::convert_toml_to_json(POLICIES).expect("Error in file conversion");

    let file: JsMap<String, JsValue> = serde_json::from_str(&file_str).expect("Error parsing file");

    let syntax_version = file.get("syntax_version").unwrap().to_string();
    println!("syntax_version: {syntax_version}");

    let policies = file.get("policies").unwrap().as_array().unwrap();

    for policy in policies {
        let policy = policy.as_object().unwrap();
        let rules = policy.get("rules").unwrap().as_array().unwrap();
        for rule in rules {
            let rule = rule.as_object().unwrap();
            let permissions = rule.get("permissions").unwrap().as_array().unwrap();
            /*let parsed_permissions = JsonParser::vec_to_permissions(permissions)
                .expect("Error parsing permissions");
            println!("permissions: {parsed_permissions:?}");*/
            for (key, value) in rule {
                if key == "permissions" {
                    continue;
                }
                println!("{key}: {value}")
            }
            println!();
        }
    }
}
