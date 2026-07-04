use std::sync::Arc;

use crate::features::auth::application::dtos::{AuthDeps, AuthResult, RegisterCommand};
use crate::features::auth::domain::{Account, Password, Session};
use crate::shared::kernel::error::AppError;

pub struct RegisterHandler {
    deps: AuthDeps,
}

impl RegisterHandler {
    pub fn new(deps: AuthDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, cmd: RegisterCommand) -> Result<AuthResult, AppError> {
        let password = Password::new(cmd.password)?;

        // Check email not taken (read-only, outside transaction)
        if self
            .deps
            .user_port
            .find_by_email(&cmd.email)
            .await?
            .is_some()
        {
            return Err(AppError::Conflict("email already registered".into()));
        }

        // Capture result from inside the transaction closure
        let result_slot = Arc::new(std::sync::Mutex::new(None::<AuthResult>));
        let slot_clone = result_slot.clone();

        let user_port = self.deps.user_port.clone();
        let auth_repo = self.deps.auth_repo.clone();
        let hasher = self.deps.password_hasher.clone();
        let email = cmd.email;
        let name = cmd.name;
        let pw = password;

        self.deps
            .uow
            .run_in_transaction(Box::pin(async move {
                // Create user
                let user = user_port.create(email, name).await?;

                // Hash password + create credential account
                let hash = hasher.hash(pw.as_str())?;
                let account = Account::new_credential(user.id, hash);
                auth_repo.save_credential(&account).await?;

                // Create session
                let session = Session::new(user.id, None, None);
                auth_repo.save_session(&session).await?;

                *slot_clone.lock().unwrap() = Some(AuthResult {
                    token: session.token,
                    user,
                });
                Ok(())
            }))
            .await?;

        let mut guard = result_slot.lock().unwrap();
        guard
            .take()
            .ok_or_else(|| AppError::Internal("transaction produced no result".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::auth::application::dtos::AuthDeps;
    use crate::features::auth::application::ports::auth_repository::MockAuthRepository;
    use crate::features::auth::application::ports::user_port::{AuthUserInfo, MockUserPort};
    use crate::shared::application::unit_of_work::NoopUnitOfWork;
    use crate::shared::application::utils::password_hasher::MockPasswordHasher;
    use crate::shared::kernel::UserId;
    use chrono::Utc;
    use std::sync::Arc;

    fn make_user() -> AuthUserInfo {
        AuthUserInfo {
            id: UserId::new(),
            email: "new@example.com".into(),
            email_verified: false,
            name: None,
            image: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn make_deps(
        user_port: MockUserPort,
        auth_repo: MockAuthRepository,
        password_hasher: MockPasswordHasher,
    ) -> AuthDeps {
        AuthDeps {
            user_port: Arc::new(user_port),
            auth_repo: Arc::new(auth_repo),
            password_hasher: Arc::new(password_hasher),
            uow: Arc::new(NoopUnitOfWork),
        }
    }

    #[tokio::test]
    async fn successful_registration() {
        let user = make_user();
        let user_id = user.id;

        let mut user_port = MockUserPort::new();
        user_port.expect_find_by_email().returning(|_| Ok(None));
        user_port
            .expect_create()
            .returning(move |_, _| Ok(user.clone()));

        let mut auth_repo = MockAuthRepository::new();
        auth_repo.expect_save_credential().returning(|_| Ok(()));
        auth_repo.expect_save_session().returning(|_| Ok(()));

        let mut hasher = MockPasswordHasher::new();
        hasher.expect_hash().returning(|_| Ok("hashed".into()));

        let handler = RegisterHandler::new(make_deps(user_port, auth_repo, hasher));
        let cmd = RegisterCommand {
            email: "new@example.com".into(),
            password: "strongpass".into(),
            name: None,
        };

        let result = handler.handle(cmd).await.unwrap();
        assert_eq!(result.user.id, user_id);
        assert!(!result.token.is_empty());
    }

    #[tokio::test]
    async fn email_taken_conflict() {
        let mut user_port = MockUserPort::new();
        user_port
            .expect_find_by_email()
            .returning(|_| Ok(Some(make_user())));

        let handler = RegisterHandler::new(make_deps(
            user_port,
            MockAuthRepository::new(),
            MockPasswordHasher::new(),
        ));
        let cmd = RegisterCommand {
            email: "taken@example.com".into(),
            password: "strongpass".into(),
            name: None,
        };

        assert!(matches!(
            handler.handle(cmd).await,
            Err(AppError::Conflict(_))
        ));
    }

    #[tokio::test]
    async fn weak_password_validation() {
        let handler = RegisterHandler::new(make_deps(
            MockUserPort::new(),
            MockAuthRepository::new(),
            MockPasswordHasher::new(),
        ));
        let cmd = RegisterCommand {
            email: "new@example.com".into(),
            password: "short".into(),
            name: None,
        };

        assert!(matches!(
            handler.handle(cmd).await,
            Err(AppError::Validation(_))
        ));
    }
}
