use crate::language::storage::Storage;

use super::to_text_repr::ToTextRepr;

impl ToTextRepr for Storage {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let resources = self.resources().to_text_repr();
        let attr_resources = self.attributed_resources().to_text_repr();

        format!("syntax = 0.16;\n\n\n{resources}{attr_resources}\n")
    }
}
