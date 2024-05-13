/// Trait for converting a parsed policies into a text representation.
pub trait ToTextRepr {
    /// Indentation to be used when printing the text representation. Can be an empty str.
    const INDENTATION: &'static str;

    fn to_text_repr(&self) -> String;
}
