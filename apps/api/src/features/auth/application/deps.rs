use std::sync::Arc;

use crate::features::auth::application::ports::auth_repository::AuthRepository;
use crate::features::auth::application::ports::user_port::UserPort;
use crate::shared::application::unit_of_work::UnitOfWork;
use crate::shared::application::utils::password_hasher::PasswordHasher;

/// Bundle of shared deps for auth handlers.
#[derive(Clone)]
pub struct AuthDeps {
    pub user_port: Arc<dyn UserPort>,
    pub auth_repo: Arc<dyn AuthRepository>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub uow: Arc<dyn UnitOfWork>,
}
