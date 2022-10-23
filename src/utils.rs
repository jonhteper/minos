#[cfg(feature = "resource_utils")]
pub fn none_as_empty_string<T: ToString>(opt: Option<T>) -> String {
    match opt {
        None => "".to_string(),
        Some(t) => t.to_string(),
    }
}
