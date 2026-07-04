use chrono::Utc;

use crate::features::auth::application::deps::AuthDeps;
use crate::features::auth::application::dtos::GetCurrentUserQuery;
use crate::features::auth::application::ports::session_info::SessionInfo;
use crate::features::auth::application::ports::user_port::AuthUserInfo;
use crate::shared::kernel::error::AppError;

pub struct GetCurrentUserHandler {
    deps: AuthDeps,
}

impl GetCurrentUserHandler {
    pub fn new(deps: AuthDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        query: GetCurrentUserQuery,
    ) -> Result<(AuthUserInfo, SessionInfo), AppError> {
        let mut session = self
            .deps
            .auth_repo
            .find_session_by_token(&query.token)
            .await?
            .ok_or_else(|| AppError::Unauthorized("not authenticated".into()))?;

        if session.is_expired_at(&Utc::now()) {
            return Err(AppError::Unauthorized("session expired".into()));
        }

        // Sliding refresh: extend expiry when less than half the lifetime remains.
        if session.needs_refresh() {
            session.refresh();
            self.deps.auth_repo.update_session_expiry(&session).await?;
        }

        let user = self
            .deps
            .user_port
            .find_by_id(&session.user_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("user not found".into()))?;

        let session_info = SessionInfo {
            expires_at: session.expires_at,
            ip_address: session.ip_address,
            user_agent: session.user_agent,
            created_at: session.created_at,
        };

        Ok((user, session_info))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::auth::application::deps::AuthDeps;
    use crate::features::auth::application::ports::auth_repository::MockAuthRepository;
    use crate::features::auth::application::ports::user_port::{AuthUserInfo, MockUserPort};
    use crate::features::auth::domain::Session;
    use crate::shared::application::unit_of_work::NoopUnitOfWork;
    use crate::shared::application::utils::password_hasher::MockPasswordHasher;
    use crate::shared::kernel::UserId;
    use chrono::{Duration, Utc};
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

    fn make_deps(user_port: MockUserPort, auth_repo: MockAuthRepository) -> AuthDeps {
        AuthDeps {
            user_port: Arc::new(user_port),
            auth_repo: Arc::new(auth_repo),
            password_hasher: Arc::new(MockPasswordHasher::new()),
            uow: Arc::new(NoopUnitOfWork),
        }
    }

    #[tokio::test]
    async fn returns_user_and_session_when_valid() {
        let session = Session::create(UserId::new(), None, None);
        let session_user_id = session.user_id;

        let mut auth_repo = MockAuthRepository::new();
        auth_repo
            .expect_find_session_by_token()
            .returning(move |_| Ok(Some(Session::create(session_user_id, None, None))));

        let mut user_port = MockUserPort::new();
        user_port
            .expect_find_by_id()
            .returning(|_| Ok(Some(make_user())));

        let handler = GetCurrentUserHandler::new(make_deps(user_port, auth_repo));
        let cmd = GetCurrentUserQuery {
            token: "valid-token".into(),
        };

        let (user, session_info) = handler.handle(cmd).await.unwrap();
        assert_eq!(user.email, "user@example.com");
        assert!(session_info.expires_at > Utc::now());
    }

    #[tokio::test]
    async fn refreshes_session_when_past_half_life() {
        let user_id = UserId::new();

        let mut auth_repo = MockAuthRepository::new();
        auth_repo
            .expect_find_session_by_token()
            .returning(move |_| {
                let mut s = Session::create(user_id, None, None);
                s.expires_at = Utc::now() + Duration::days(1); // < 3.5 days remaining
                Ok(Some(s))
            });
        auth_repo
            .expect_update_session_expiry()
            .returning(|_| Ok(()));

        let mut user_port = MockUserPort::new();
        user_port
            .expect_find_by_id()
            .returning(|_| Ok(Some(make_user())));

        let handler = GetCurrentUserHandler::new(make_deps(user_port, auth_repo));
        let cmd = GetCurrentUserQuery {
            token: "valid-token".into(),
        };

        let (_user, session_info) = handler.handle(cmd).await.unwrap();
        assert!(session_info.expires_at > Utc::now() + Duration::days(6));
    }

    #[rstest]
    #[case::session_not_found("session_not_found")]
    #[case::session_expired("session_expired")]
    #[case::user_not_found("user_not_found")]
    #[tokio::test]
    async fn failure_paths_return_unauthorized(#[case] case: &str) {
        let mut auth_repo = MockAuthRepository::new();
        let mut user_port = MockUserPort::new();

        match case {
            "session_not_found" => {
                auth_repo
                    .expect_find_session_by_token()
                    .returning(|_| Ok(None));
            }
            "session_expired" => {
                auth_repo.expect_find_session_by_token().returning(|_| {
                    let mut s = Session::create(UserId::new(), None, None);
                    s.expires_at = Utc::now() - Duration::days(1);
                    Ok(Some(s))
                });
            }
            "user_not_found" => {
                auth_repo
                    .expect_find_session_by_token()
                    .returning(|_| Ok(Some(Session::create(UserId::new(), None, None))));
                user_port.expect_find_by_id().returning(|_| Ok(None));
            }
            _ => unreachable!(),
        }

        let handler = GetCurrentUserHandler::new(make_deps(user_port, auth_repo));
        let cmd = GetCurrentUserQuery {
            token: "some-token".into(),
        };

        assert!(matches!(
            handler.handle(cmd).await,
            Err(AppError::Unauthorized(_))
        ));
    }
}
