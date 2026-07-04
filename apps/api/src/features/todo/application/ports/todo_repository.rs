use async_trait::async_trait;

use crate::features::todo::domain::Todo;
use crate::features::todo::domain::TodoId;
use crate::shared::kernel::error::AppError;
use crate::shared::kernel::UserId;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn save(&self, todo: &Todo) -> Result<(), AppError>;
    async fn find_by_id(&self, id: &TodoId, user_id: &UserId) -> Result<Option<Todo>, AppError>;
    async fn find_all(&self, user_id: &UserId) -> Result<Vec<Todo>, AppError>;
    async fn delete(&self, id: &TodoId, user_id: &UserId) -> Result<(), AppError>;
}
