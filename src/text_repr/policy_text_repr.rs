use crate::language::policy::{Permission, Policy};

use super::to_text_repr::ToTextRepr;

impl ToTextRepr for Vec<Policy> {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let mut policies_str = String::new();
        for policy in self {
            policies_str.push_str(&policy.to_text_repr());
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
