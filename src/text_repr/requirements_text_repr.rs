use crate::language::requirements::{
    Assertion, Attribute, ComparableValue, Negation, Requirement, Search, Value,
};

use super::to_text_repr::ToTextRepr;

impl ToTextRepr for Vec<Requirement> {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        let mut repr = String::new();

        for requirement in self {
            repr.push_str(&requirement.to_text_repr());
        }

        repr
    }
}

impl ToTextRepr for Requirement {
    /// 4 tabs of indentation
    const INDENTATION: &'static str = "                ";

    fn to_text_repr(&self) -> String {
        let requirement = match self {
            Requirement::Assertion(assertion) => assertion.to_text_repr(),
            Requirement::Negation(negation) => negation.to_text_repr(),
            Requirement::Search(search) => search.to_text_repr(),
        };

        format!("{}{};\n", Self::INDENTATION, requirement)
    }
}

impl ToTextRepr for Assertion {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        format!("{} = {}", self.left().to_text_repr(), self.right().to_text_repr())
    }
}

impl ToTextRepr for Negation {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        format!(
            "{} != {}",
            self.left().to_text_repr(),
            self.right().to_text_repr()
        )
    }
}

impl ToTextRepr for Search {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        format!(
            "{} *= {}",
            self.left().to_text_repr(),
            self.right().to_text_repr()
        )
    }
}

impl ToTextRepr for Attribute {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        match self {
            Attribute::Actor(attr) => attr.to_string(),
            Attribute::Resource(attr) => attr.to_string(),
        }
    }
}

impl ToTextRepr for Value {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        match self {
            Value::String(v) => format!("\"{}\"", v),
            Value::Array(arr) => format!("{:?}", arr.0),
            Value::Identifier(v) => v.0.to_string(),
        }
    }
}

impl ToTextRepr for ComparableValue {
    const INDENTATION: &'static str = "";

    fn to_text_repr(&self) -> String {
        match self {
            ComparableValue::Attribute(attr) => attr.to_text_repr(),
            ComparableValue::Value(val) => val.to_text_repr(),
        }
    }
}
