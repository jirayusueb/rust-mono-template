use crate::features::auth::application::deps::AuthDeps;
use crate::features::auth::application::dtos::SignOutCommand;
use crate::shared::kernel::error::AppError;

pub struct SignOutHandler {
    deps: AuthDeps,
}

impl SignOutHandler {
    pub fn new(deps: AuthDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, cmd: SignOutCommand) -> Result<(), AppError> {
        self.deps.auth_repo.delete_session(&cmd.token).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::auth::application::deps::AuthDeps;
    use crate::features::auth::application::ports::auth_repository::MockAuthRepository;
    use crate::features::auth::application::ports::user_port::MockUserPort;
    use crate::shared::application::unit_of_work::NoopUnitOfWork;
    use crate::shared::application::utils::password_hasher::MockPasswordHasher;
    use std::sync::Arc;

    fn make_deps(auth_repo: MockAuthRepository) -> AuthDeps {
        AuthDeps {
            user_port: Arc::new(MockUserPort::new()),
            auth_repo: Arc::new(auth_repo),
            password_hasher: Arc::new(MockPasswordHasher::new()),
            uow: Arc::new(NoopUnitOfWork),
        }
    }

    #[tokio::test]
    async fn deletes_session() {
        let mut auth_repo = MockAuthRepository::new();
        auth_repo.expect_delete_session().returning(|_| Ok(()));

        let handler = SignOutHandler::new(make_deps(auth_repo));
        let cmd = SignOutCommand {
            token: "session-token".into(),
        };

        assert!(handler.handle(cmd).await.is_ok());
    }

    #[tokio::test]
    async fn delete_failure_propagates() {
        let mut auth_repo = MockAuthRepository::new();
        auth_repo
            .expect_delete_session()
            .returning(|_| Err(AppError::Internal("db error".into())));

        let handler = SignOutHandler::new(make_deps(auth_repo));
        let cmd = SignOutCommand {
            token: "session-token".into(),
        };

        assert!(matches!(
            handler.handle(cmd).await,
            Err(AppError::Internal(_))
        ));
    }
}
