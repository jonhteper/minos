use derived::Ctor;

use crate::{language::environment::Environment, text_repr::policy_text_repr::PoliciesFormatter};

use super::to_text_repr::ToTextRepr;

#[derive(Debug, Clone, Ctor)]
pub struct EnvironmentsFormatter<'a, T>
where
    T: IntoIterator<Item = &'a Environment>,
{
    pub envs: T,

    /// To prevent the cloning of the [`Environment`] iterator,
    /// saved its size from construction.
    pub envs_len: usize,
}

impl<'a, T> ToTextRepr for EnvironmentsFormatter<'a, T>
where
    T: IntoIterator<Item = &'a Environment> + Clone,
{
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let mut envs_str = String::new();
        let iter = self.envs.clone().into_iter();
        for (index, env) in iter.enumerate() {
            envs_str.push_str(&env.to_text_repr());

            if index < self.envs_len - 1 {
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
        let policies_vec = self.policies();
        let policies_formatter = PoliciesFormatter::new(policies_vec, policies_vec.len());
        let policies = policies_formatter.to_text_repr();

        format!("{ind}env {identifier} {{\n{policies}{ind}}}\n")
    }
}
