use std::fmt;

use serde::{Deserialize, Serialize};

use super::super::error::UserDomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Result<Self, UserDomainError> {
        if !is_valid(&value) {
            return Err(UserDomainError::InvalidEmail);
        }
        Ok(Self(value.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Email {
    type Error = UserDomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Minimal validation: must contain exactly one `@` with text on both sides.
/// ponytail: not RFC 5322 compliant — good enough for a starter; tighten if bounce rates matter.
fn is_valid(s: &str) -> bool {
    let parts: Vec<&str> = s.split('@').collect();
    parts.len() == 2 && !parts[0].is_empty() && parts[1].contains('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_email() {
        let email = Email::new("user@example.com".into()).unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn normalizes_to_lowercase() {
        let email = Email::new("User@Example.COM".into()).unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn rejects_no_at() {
        assert_eq!(
            Email::new("userexample.com".into()).unwrap_err(),
            UserDomainError::InvalidEmail
        );
    }

    #[test]
    fn rejects_empty_local() {
        assert_eq!(
            Email::new("@example.com".into()).unwrap_err(),
            UserDomainError::InvalidEmail
        );
    }

    #[test]
    fn rejects_no_dot_in_domain() {
        assert_eq!(
            Email::new("user@example".into()).unwrap_err(),
            UserDomainError::InvalidEmail
        );
    }
}
