use derived::Ctor;

use crate::language::policy::{Permission, Policy};

use super::to_text_repr::ToTextRepr;

#[derive(Debug, Clone, Ctor)]
pub struct PoliciesFormatter<'a, T>
where
    T: ExactSizeIterator<Item = &'a Policy>,
{
    pub policies: T,
}

impl<'a, T> ToTextRepr for PoliciesFormatter<'a, T>
where
    T: ExactSizeIterator<Item = &'a Policy> + Clone,
{
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let mut policies_str = String::new();
        for (index, policy) in self.policies.clone().enumerate() {
            policies_str.push_str(&policy.to_text_repr());

            if index < self.policies.len() - 1 {
                policies_str.push('\n');
            }
        }

        policies_str
    }
}

impl ToTextRepr for Policy {
    /// 2 tabs of indentation
    const INDENTATION: &'static str = "        ";
    fn to_text_repr(&self) -> String {
        let ind = Self::INDENTATION;
        let allow = self.permissions().to_text_repr();
        let rules = self.rules().to_text_repr();

        format!("{ind}policy {{\n{allow}{rules}{ind}}}\n")
    }
}

impl ToTextRepr for Vec<Permission> {
    /// 3 tabs of identation
    const INDENTATION: &'static str = "            ";

    fn to_text_repr(&self) -> String {
        let permissions_str = self.iter().map(|p| p.0.as_ref()).collect::<Vec<&str>>();

        format!("{}allow = {:?};\n\n", Self::INDENTATION, permissions_str)
    }
}
