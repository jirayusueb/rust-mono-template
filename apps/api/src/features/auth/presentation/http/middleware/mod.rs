use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;

use crate::bootstrap::AppState;
use crate::features::auth::application::dtos::GetCurrentUserQuery;
use crate::features::auth::application::ports::session_info::SessionInfo;
use crate::features::auth::application::ports::user_port::AuthUserInfo;
use crate::features::auth::application::queries::get_current_user::GetCurrentUserHandler;
use crate::shared::kernel::error::AppError;

/// Extractor that validates the session cookie and loads the authenticated user + session.
pub struct AuthUser(pub AuthUserInfo, pub SessionInfo);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Internal("failed to read cookies".into()))?;

        let token = jar
            .get("session")
            .ok_or_else(|| AppError::Unauthorized("not authenticated".into()))?
            .value();

        let handler = GetCurrentUserHandler::new(state.auth_deps());
        let (user, session) = handler
            .handle(GetCurrentUserQuery {
                token: token.to_string(),
            })
            .await?;

        Ok(AuthUser(user, session))
    }
}
