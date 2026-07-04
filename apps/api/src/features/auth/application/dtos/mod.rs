use std::sync::Arc;

use crate::features::auth::application::ports::auth_repository::AuthRepository;
use crate::features::auth::application::ports::user_port::{AuthUserInfo, UserPort};
use crate::shared::application::unit_of_work::UnitOfWork;
use crate::shared::application::utils::password_hasher::PasswordHasher;

pub struct RegisterCommand {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

pub struct LoginCommand {
    pub email: String,
    pub password: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

pub struct LogoutCommand {
    pub token: String,
}

pub struct GetCurrentUserQuery {
    pub token: String,
}

/// Bundle of shared deps for auth handlers.
#[derive(Clone)]
pub struct AuthDeps {
    pub user_port: Arc<dyn UserPort>,
    pub auth_repo: Arc<dyn AuthRepository>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub uow: Arc<dyn UnitOfWork>,
}

/// Result of register/login — the handler needs both the user and session token.
pub struct AuthResult {
    pub user: AuthUserInfo,
    pub token: String,
}
