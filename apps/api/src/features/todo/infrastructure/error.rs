use crate::shared::kernel::error::AppError;

#[derive(Debug, thiserror::Error)]
pub enum TodoInfraError {
    #[error("database error")]
    Database(#[from] sea_orm::DbErr),
    #[error("connection pool error: {0}")]
    Pool(String),
    #[error("decode error: {0}")]
    Decode(String),
}

impl From<TodoInfraError> for AppError {
    fn from(err: TodoInfraError) -> Self {
        AppError::Internal(err.to_string())
    }
}
