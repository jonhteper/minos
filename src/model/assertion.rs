use serde_json::{Map, Value};

enum Operator {
    /// =
    Equal,
    /// !=
    Distinct,
    /// <
    Minor,
    /// >
    Major,
    /// <=
    MinorOrEqual,
    /// >=
    MajorOrEqual,
}

/// Always must be true
pub struct Assertion {
    left: Map<String, Value>,
    operator: Operator,
    right: String,
}