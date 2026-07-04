use crate::shared::kernel::error::AppError;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum TodoDomainError {
    #[error("title cannot be empty")]
    EmptyTitle,
    #[error("todo is already completed")]
    AlreadyCompleted,
    #[error("todo is not completed")]
    NotCompleted,
}

impl From<TodoDomainError> for AppError {
    fn from(err: TodoDomainError) -> Self {
        match err {
            TodoDomainError::EmptyTitle => AppError::Validation(err.to_string()),
            TodoDomainError::AlreadyCompleted | TodoDomainError::NotCompleted => {
                AppError::Conflict(err.to_string())
            }
        }
    }
}
