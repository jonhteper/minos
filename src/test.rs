use toml::map::Map;
use toml::{Value, from_str};
use serde_json::Value as JsValue;
use serde_json::Map as JsMap;
use crate::model::parser::Parser;
use crate::model::permission::ParsePermissions;

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

const POLICIES:&str = r#"
        sintaxis = "0.6"

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
fn parse_toml_file1() {
    let file: Map<String, Value> = from_str(POLICIES)
        .expect("Error decoding policies");

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
                if key == "permissions"{
                    continue
                }
                println!("{key}: {value}")
            }
            println!();
        }
    }

}

#[test]
fn parse_toml_file2() {
    let file_str = tsu::convert_toml_to_json(POLICIES)
        .expect("Error in file conversion");

    let file: JsMap<String, JsValue> = serde_json::from_str(&file_str)
        .expect("Error parsing file");


    let sintaxis = file.get("sintaxis").unwrap().to_string();
    println!("sintaxis: {sintaxis}");

    let policies = file.get("policies").unwrap().as_array().unwrap();

    for policy in policies {
        let policy = policy.as_object().unwrap();
        let rules = policy.get("rules").unwrap().as_array().unwrap();
        for rule in rules {
            let rule = rule.as_object().unwrap();
            let permissions = rule.get("permissions").unwrap().as_array().unwrap();
            let parser = Parser;
            let parsed_permissions = parser.to_permissions(permissions)
                .expect("Error parsing permissions");
            println!("permissions: {parsed_permissions:?}");
            for (key, value) in rule {
                if key == "permissions"{
                    continue
                }
                println!("{key}: {value}")
            }
            println!();
        }
    }
}