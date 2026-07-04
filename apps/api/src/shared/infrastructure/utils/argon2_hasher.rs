use argon2::password_hash::{PasswordHash, PasswordHasher as _, PasswordVerifier as _, SaltString};
use argon2::Argon2;
use rand::rngs::OsRng;

use crate::shared::application::utils::password_hasher::PasswordHasher;
use crate::shared::kernel::error::AppError;

pub struct Argon2PasswordHasher;

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Argon2PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("password hash error: {e}")))?;
        Ok(hash.to_string())
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed = PasswordHash::new(hash)
            .map_err(|e| AppError::Internal(format!("password hash parse error: {e}")))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok())
    }
}
