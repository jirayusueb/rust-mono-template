use serde::{Deserialize, Serialize};

use super::super::error::TodoDomainError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct Title(String);

impl Title {
    pub fn new(value: String) -> Result<Self, TodoDomainError> {
        if value.trim().is_empty() {
            return Err(TodoDomainError::EmptyTitle);
        }
        Ok(Self(value))
    }

    /// Factory for EXISTING data (from trusted DB source). Bypasses validation.
    pub fn restore(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Title {
    type Error = TodoDomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
