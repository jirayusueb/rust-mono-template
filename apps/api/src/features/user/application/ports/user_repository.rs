use async_trait::async_trait;

use crate::features::user::domain::Email;
use crate::features::user::domain::User;
use crate::shared::kernel::error::AppError;
use crate::shared::kernel::UserId;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), AppError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AppError>;
}
