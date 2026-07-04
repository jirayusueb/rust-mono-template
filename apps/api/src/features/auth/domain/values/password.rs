use crate::features::auth::domain::error::AuthDomainError;

/// Password value object — validates minimum length at construction.
/// The raw password is intentionally not stored; callers extract it for hashing.
#[derive(Debug)]
pub struct Password(String);

impl Password {
    pub fn new(value: String) -> Result<Self, AuthDomainError> {
        if value.len() < 8 {
            return Err(AuthDomainError::WeakPassword);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_password_of_min_length() {
        assert!(Password::new("12345678".into()).is_ok());
    }

    #[test]
    fn rejects_short_password() {
        assert_eq!(
            Password::new("1234567".into()).unwrap_err(),
            AuthDomainError::WeakPassword
        );
    }
}
