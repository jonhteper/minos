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
        permissions = [["read", "30 min"]]
        resource.status = "published"


        [[policies.rules]]
        permissions = [
            ["read", "60min"],
            ["write", "1500ms"]
        ]
        actor.groups = ["writers"]
        by_owner = true


        [[policies.rules]]
        permissions = [["revise", "1200 ms"]]
        actor.groups = ["admins"]


        [[policies.rules]]
        permissions = [["publish", "1s"]]
        actor.groups = ["admins"]
        actor.status = "active"
        resource.status = "writed"
        resource.is_revised = true
        environment.in_maintainning_mode = false
    "#;

#[test]
fn authorization_works() {
    todo!()
}