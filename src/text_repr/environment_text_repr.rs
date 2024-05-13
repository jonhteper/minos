use std::collections::HashMap;

use crate::{
    language::environment::Environment, parser::tokens::Identifier,
    text_repr::policy_text_repr::PoliciesFormatter,
};

use super::to_text_repr::ToTextRepr;

impl ToTextRepr for HashMap<Identifier, Environment> {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let mut envs_str = String::new();
        for (index, env) in self.values().enumerate() {
            envs_str.push_str(&env.to_text_repr());

            if index < self.len() - 1 {
                envs_str.push('\n');
            }
        }

        envs_str
    }
}

impl ToTextRepr for Environment {
    /// one tab of indentation
    const INDENTATION: &'static str = "    ";

    fn to_text_repr(&self) -> String {
        let ind = Self::INDENTATION;
        let identifier = &self.identifier().0;
        let policies_formatter = PoliciesFormatter::new(self.policies());
        let policies = policies_formatter.to_text_repr();

        format!("{ind}env {identifier} {{\n{policies}{ind}}}\n")
    }
}
