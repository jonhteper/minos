use std::sync::Arc;

use crate::language::rule::Rule;

use super::to_text_repr::ToTextRepr;

impl ToTextRepr for Vec<Arc<Rule>> {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let mut rules_str = String::new();

        for rule in self {
            rules_str.push_str(&rule.to_text_repr());
        }

        rules_str
    }
}

impl ToTextRepr for Rule {
    const INDENTATION: &'static str = "\t\t\t";

    fn to_text_repr(&self) -> String {
        let ind = Self::INDENTATION;
        let requirements = self.requirements().to_text_repr();

        format!("{ind}rule {{\n{requirements}{ind}}}\n\n")
    }
}
