use crate::model::rules::Rule;

pub struct Policies {
    resource_type: Option<String>,
    resource_id: Option<String>,
    rules: Vec<Rule>,
}
