use async_trait::async_trait;

use crate::features::auth::domain::Account;
use crate::features::auth::domain::Session;
use crate::shared::kernel::error::AppError;
use crate::shared::kernel::UserId;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AuthRepository: Send + Sync {
    // Accounts
    async fn save_credential(&self, account: &Account) -> Result<(), AppError>;
    async fn find_credential_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Option<Account>, AppError>;

    // Sessions
    async fn save_session(&self, session: &Session) -> Result<(), AppError>;
    async fn find_session_by_token(&self, token: &str) -> Result<Option<Session>, AppError>;
    async fn delete_session(&self, token: &str) -> Result<(), AppError>;
    async fn update_session_expiry(&self, session: &Session) -> Result<(), AppError>;
}
