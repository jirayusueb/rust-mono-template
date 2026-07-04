use std::sync::Arc;

use crate::features::auth::application::ports::user_port::{AuthUserInfo, UserPort};
use crate::features::user::application::ports::user_repository::UserRepository;
use crate::features::user::domain::Email;
use crate::features::user::domain::User;
use crate::shared::kernel::error::AppError;
use crate::shared::kernel::UserId;

/// Adapter: bridges auth's UserPort to the user feature's UserRepository.
/// This is the ONLY file in auth that imports from the user module.
pub struct UserPortAdapter {
    repo: Arc<dyn UserRepository>,
}

impl UserPortAdapter {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

fn map(user: User) -> AuthUserInfo {
    AuthUserInfo {
        id: user.id,
        email: user.email.as_str().to_string(),
        email_verified: user.email_verified,
        name: user.name,
        image: user.image,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}

#[async_trait::async_trait]
impl UserPort for UserPortAdapter {
    async fn find_by_email(&self, email: &str) -> Result<Option<AuthUserInfo>, AppError> {
        let email = Email::new(email.to_string())?;
        Ok(self.repo.find_by_email(&email).await?.map(map))
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<AuthUserInfo>, AppError> {
        Ok(self.repo.find_by_id(id).await?.map(map))
    }

    async fn create(&self, email: String, name: Option<String>) -> Result<AuthUserInfo, AppError> {
        let email = Email::new(email)?;
        let user = User::new(email, name);
        self.repo.save(&user).await?;
        Ok(map(user))
    }
}
