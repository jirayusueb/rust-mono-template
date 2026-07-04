use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::shared::kernel::error::AppError;
use crate::shared::kernel::UserId;

/// Auth's view of a user — owned by auth, no dependency on the user feature.
#[derive(Debug, Clone)]
pub struct AuthUserInfo {
    pub id: UserId,
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Port: what auth needs from the user context.
/// The adapter (in infrastructure) bridges to the user feature's UserRepository.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserPort: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<AuthUserInfo>, AppError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<AuthUserInfo>, AppError>;
    async fn create(&self, email: String, name: Option<String>) -> Result<AuthUserInfo, AppError>;
}
