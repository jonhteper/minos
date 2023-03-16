use crate::model::attribute::{Attribute, Comparator};
use crate::model::permission::Permission;



pub struct Rule {
    left: Attribute,
    comparator: Comparator,
    right: Attribute,
}


pub struct Rules {
    permissions: Vec<Permission>,
    rules: Vec<Rule>,
}