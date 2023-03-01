use crate::core::rule::Rule;

struct Policies {
    resource_type: Option<String>,
    resource_id: Option<String>,
    rules: Vec<Rule>,
}
