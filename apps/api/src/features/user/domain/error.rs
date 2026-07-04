use crate::shared::kernel::error::AppError;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum UserDomainError {
    #[error("invalid email format")]
    InvalidEmail,
}

impl From<UserDomainError> for AppError {
    fn from(err: UserDomainError) -> Self {
        match err {
            UserDomainError::InvalidEmail => AppError::Validation(err.to_string()),
        }
    }
}
