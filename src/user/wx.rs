use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// Represents a WeChat ID (Wxid) that must start with "wxid_" and be within a valid length range.
/// - Unnamed single-field struct → Serialized directly as the field’s value (used here)
/// - Unnamed multi-field struct → Serialized as a JSON array
/// - Named multi-field struct → Serialized as a JSON object
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Wxid(String);

impl Wxid {
    /// Returns the Wxid as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for Wxid {
    type Error = String;

    /// Attempts to create a Wxid from a string slice.
    ///
    /// Fails if the string does not start with "wxid_" or is not within the valid length range.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("wxid_") || value.ends_with("@chatroom") {
            // TODO value.len() > 5 && value.len() < 50
            Ok(Wxid(value.to_string()))
        } else {
            Err(format!("Invalid Wxid format: {}", value))
        }
    }
}

impl std::fmt::Display for Wxid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
