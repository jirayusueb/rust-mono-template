use crate::shared::kernel::error::AppError;

/// Hashes and verifies passwords. Implemented by Argon2PasswordHasher.
#[cfg_attr(test, mockall::automock)]
pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, AppError>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError>;
}
