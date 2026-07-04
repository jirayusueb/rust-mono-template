use std::sync::Arc;

use crate::features::auth::application::dtos::AuthDeps;
use crate::features::auth::application::ports::auth_repository::AuthRepository;
use crate::features::auth::application::ports::user_port::UserPort;
use crate::features::todo::application::ports::todo_repository::TodoRepository;
use crate::features::user::application::ports::user_repository::UserRepository;
use crate::shared::application::unit_of_work::UnitOfWork;
use crate::shared::application::utils::password_hasher::PasswordHasher;

/// Composition root: wires all feature ports to their concrete adapters.
/// Lives in bootstrap/ (not shared/kernel/) because it knows about features.
#[derive(Clone)]
pub struct AppState {
    pub todo_repo: Arc<dyn TodoRepository>,
    pub user_repo: Arc<dyn UserRepository>,
    pub auth_repo: Arc<dyn AuthRepository>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub user_port: Arc<dyn UserPort>,
    pub uow: Arc<dyn UnitOfWork>,
    pub is_dev: bool,
}

impl AppState {
    pub fn auth_deps(&self) -> AuthDeps {
        AuthDeps {
            user_port: self.user_port.clone(),
            auth_repo: self.auth_repo.clone(),
            password_hasher: self.password_hasher.clone(),
            uow: self.uow.clone(),
        }
    }
}
