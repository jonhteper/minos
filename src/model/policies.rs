use crate::model::rule::Rule;

struct Policies<'r> {
    resource_type: Option<String>,
    resource_id: Option<String>,
    rules: Vec<Rule<'r>>,
}
