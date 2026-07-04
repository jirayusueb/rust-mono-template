use crate::features::auth::application::dtos::{AuthDeps, AuthResult, SignInCommand};
use crate::features::auth::domain::Session;
use crate::shared::kernel::error::AppError;

pub struct SignInHandler {
    deps: AuthDeps,
}

impl SignInHandler {
    pub fn new(deps: AuthDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, cmd: SignInCommand) -> Result<AuthResult, AppError> {
        // Find user by email
        let user = self
            .deps
            .user_port
            .find_by_email(&cmd.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("invalid credentials".into()))?;

        // Find credential
        let account = self
            .deps
            .auth_repo
            .find_credential_by_user_id(&user.id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("invalid credentials".into()))?;

        let hash = account
            .password
            .ok_or_else(|| AppError::Unauthorized("invalid credentials".into()))?;

        // Verify password
        if !self.deps.password_hasher.verify(&cmd.password, &hash)? {
            return Err(AppError::Unauthorized("invalid credentials".into()));
        }

        // Create session
        let session = Session::create(user.id, cmd.ip_address, cmd.user_agent);
        self.deps.auth_repo.save_session(&session).await?;

        Ok(AuthResult {
            user,
            token: session.token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::auth::application::ports::auth_repository::MockAuthRepository;
    use crate::features::auth::application::ports::user_port::{AuthUserInfo, MockUserPort};
    use crate::features::auth::domain::Account;
    use crate::shared::application::unit_of_work::NoopUnitOfWork;
    use crate::shared::application::utils::password_hasher::MockPasswordHasher;
    use crate::shared::kernel::UserId;
    use chrono::Utc;
    use rstest::rstest;
    use std::sync::Arc;

    fn make_user() -> AuthUserInfo {
        AuthUserInfo {
            id: UserId::new(),
            email: "user@example.com".into(),
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

    fn make_cmd() -> SignInCommand {
        SignInCommand {
            email: "user@example.com".into(),
            password: "password123".into(),
            ip_address: None,
            user_agent: None,
        }
    }

    #[tokio::test]
    async fn successful_sign_in() {
        let user = make_user();
        let user_id = user.id;

        let mut user_port = MockUserPort::new();
        user_port
            .expect_find_by_email()
            .returning(move |_| Ok(Some(user.clone())));

        let mut auth_repo = MockAuthRepository::new();
        auth_repo
            .expect_find_credential_by_user_id()
            .returning(move |_| Ok(Some(Account::create(user_id, "hashed".into()))));
        auth_repo.expect_save_session().returning(|_| Ok(()));

        let mut hasher = MockPasswordHasher::new();
        hasher.expect_verify().returning(|_, _| Ok(true));

        let handler = SignInHandler::new(make_deps(user_port, auth_repo, hasher));
        let result = handler.handle(make_cmd()).await.unwrap();

        assert_eq!(result.user.id, user_id);
        assert!(!result.token.is_empty());
    }

    #[rstest]
    #[case::user_not_found("user_not_found")]
    #[case::credential_not_found("credential_not_found")]
    #[case::password_mismatch("password_mismatch")]
    #[tokio::test]
    async fn sign_in_failure_paths_return_unauthorized(#[case] case: &str) {
        let user = make_user();
        let user_id = user.id;

        let mut user_port = MockUserPort::new();
        let mut auth_repo = MockAuthRepository::new();
        let mut hasher = MockPasswordHasher::new();

        match case {
            "user_not_found" => {
                user_port.expect_find_by_email().returning(|_| Ok(None));
            }
            "credential_not_found" => {
                user_port
                    .expect_find_by_email()
                    .returning(move |_| Ok(Some(user.clone())));
                auth_repo
                    .expect_find_credential_by_user_id()
                    .returning(|_| Ok(None));
            }
            "password_mismatch" => {
                user_port
                    .expect_find_by_email()
                    .returning(move |_| Ok(Some(user.clone())));
                auth_repo
                    .expect_find_credential_by_user_id()
                    .returning(move |_| Ok(Some(Account::create(user_id, "hashed".into()))));
                hasher.expect_verify().returning(|_, _| Ok(false));
            }
            _ => unreachable!(),
        }

        let handler = SignInHandler::new(make_deps(user_port, auth_repo, hasher));
        assert!(matches!(
            handler.handle(make_cmd()).await,
            Err(AppError::Unauthorized(_))
        ));
    }
}
