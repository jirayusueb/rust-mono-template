use crate::shared::kernel::error::AppError;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum AuthDomainError {
    #[error("password must be at least 8 characters")]
    WeakPassword,
}

impl From<AuthDomainError> for AppError {
    fn from(err: AuthDomainError) -> Self {
        match err {
            AuthDomainError::WeakPassword => AppError::Validation(err.to_string()),
        }
    }
}
