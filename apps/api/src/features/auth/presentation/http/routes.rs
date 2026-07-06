use axum::extract::{ConnectInfo, State};
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::GovernorLayer;
use validator::Validate;

use crate::bootstrap::AppState;
use crate::features::auth::application::commands::sign_in::SignInHandler;
use crate::features::auth::application::commands::sign_out::SignOutHandler;
use crate::features::auth::application::commands::sign_up::SignUpHandler;
use crate::features::auth::application::dtos::{GetCurrentUserInput, SignOutInput};
use crate::features::auth::application::queries::current_user::GetCurrentUserHandler;
use crate::features::auth::presentation::http::dtos::{
    SessionResponse, SignInRequest, SignUpRequest, UserResponse,
};
use crate::shared::kernel::error::AppError;

use super::mappers::AuthMapper;

const SESSION_COOKIE: &str = "session";

pub fn auth_routes() -> Router<AppState> {
    let governor = GovernorLayer::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .finish()
            .unwrap(),
    );

    // Rate-limited routes (brute-force targets)
    Router::new()
        .route("/auth/sign-up", post(sign_up))
        .route("/auth/sign-in", post(sign_in))
        .layer(governor)
        .merge(
            // Open routes
            Router::new()
                .route("/auth/sign-out", post(sign_out))
                .route("/auth/session", get(session)),
        )
}

#[utoipa::path(
    post,
    path = "/auth/sign-up",
    tag = "auth",
    operation_id = "signUp",
    request_body = SignUpRequest,
    responses((status = 201, body = UserResponse, description = "user created")),
)]
async fn sign_up(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(req): Json<SignUpRequest>,
) -> Result<(StatusCode, CookieJar, Json<UserResponse>), AppError> {
    req.validate()?;
    let handler = SignUpHandler::new(state.auth_deps());
    let result = handler
        .handle(AuthMapper::to_sign_up_input(req))
        .await?;

    let jar = jar.add(build_cookie(&result.token, !state.is_dev));
    Ok((
        StatusCode::CREATED,
        jar,
        Json(AuthMapper::to_user_response(result.user)),
    ))
}

#[utoipa::path(
    post,
    path = "/auth/sign-in",
    tag = "auth",
    operation_id = "signIn",
    request_body = SignInRequest,
    responses((status = 200, body = UserResponse, description = "user signed in")),
)]
async fn sign_in(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(req): Json<SignInRequest>,
) -> Result<(StatusCode, CookieJar, Json<UserResponse>), AppError> {
    req.validate()?;

    // ponytail: X-Forwarded-For first (behind proxy), fallback to direct connect IP
    let ip_address = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string());
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    let handler = SignInHandler::new(state.auth_deps());
    let result = handler
        .handle(AuthMapper::to_sign_in_input(
            req,
            Some(ip_address),
            user_agent,
        ))
        .await?;

    let jar = jar.add(build_cookie(&result.token, !state.is_dev));
    Ok((
        StatusCode::OK,
        jar,
        Json(AuthMapper::to_user_response(result.user)),
    ))
}

#[utoipa::path(
    post,
    path = "/auth/sign-out",
    tag = "auth",
    operation_id = "signOut",
    responses((status = 204, description = "signed out")),
)]
async fn sign_out(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(StatusCode, CookieJar), AppError> {
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        let handler = SignOutHandler::new(state.auth_deps());
        handler
            .handle(SignOutInput {
                token: cookie.value().to_string(),
            })
            .await?;
    }
    let jar = jar.remove(axum_extra::extract::cookie::Cookie::from(
        SESSION_COOKIE.to_string(),
    ));
    Ok((StatusCode::NO_CONTENT, jar))
}

#[utoipa::path(
    get,
    path = "/auth/session",
    tag = "auth",
    operation_id = "session",
    responses((status = 200, body = Option<SessionResponse>, description = "current session or null")),
)]
async fn session(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<Option<SessionResponse>>, AppError> {
    let Some(cookie) = jar.get(SESSION_COOKIE) else {
        return Ok(Json(None));
    };

    let handler = GetCurrentUserHandler::new(state.auth_deps());
    let result = handler
        .handle(GetCurrentUserInput {
            token: cookie.value().to_string(),
        })
        .await;

    match result {
        Ok(r) => Ok(Json(Some(AuthMapper::to_session_response(r.user, r.session)))),
        // Expired / invalid / not-found → null, not 401
        Err(_) => Ok(Json(None)),
    }
}

fn build_cookie(token: &str, secure: bool) -> axum_extra::extract::cookie::Cookie<'static> {
    axum_extra::extract::cookie::Cookie::build((SESSION_COOKIE.to_string(), token.to_string()))
        .path("/")
        .http_only(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .secure(secure)
        .max_age(time::Duration::days(7))
        .build()
}
