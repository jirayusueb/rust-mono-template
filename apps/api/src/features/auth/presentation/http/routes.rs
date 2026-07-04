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
use crate::features::auth::application::commands::login::LoginHandler;
use crate::features::auth::application::commands::logout::LogoutHandler;
use crate::features::auth::application::commands::register::RegisterHandler;
use crate::features::auth::application::dtos::{LoginCommand, LogoutCommand, RegisterCommand};
use crate::features::auth::presentation::http::dtos::{
    LoginRequest, RegisterRequest, SessionPayload, UserResponse,
};
use crate::shared::kernel::error::AppError;

use super::middleware::AuthUser;

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
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .layer(governor)
        .merge(
            // Open routes
            Router::new()
                .route("/auth/logout", post(logout))
                .route("/auth/session", get(session)),
        )
}

async fn register(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, CookieJar, Json<UserResponse>), AppError> {
    req.validate()?;
    let handler = RegisterHandler::new(state.auth_deps());
    let result = handler
        .handle(RegisterCommand {
            email: req.email,
            password: req.password,
            name: req.name,
        })
        .await?;

    let jar = jar.add(build_cookie(&result.token, !state.is_dev));
    Ok((
        StatusCode::CREATED,
        jar,
        Json(UserResponse::from(result.user)),
    ))
}

async fn login(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(req): Json<LoginRequest>,
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

    let handler = LoginHandler::new(state.auth_deps());
    let result = handler
        .handle(LoginCommand {
            email: req.email,
            password: req.password,
            ip_address: Some(ip_address),
            user_agent,
        })
        .await?;

    let jar = jar.add(build_cookie(&result.token, !state.is_dev));
    Ok((StatusCode::OK, jar, Json(UserResponse::from(result.user))))
}

async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(StatusCode, CookieJar), AppError> {
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        let handler = LogoutHandler::new(state.auth_deps());
        handler
            .handle(LogoutCommand {
                token: cookie.value().to_string(),
            })
            .await?;
    }
    let jar = jar.remove(axum_extra::extract::cookie::Cookie::from(
        SESSION_COOKIE.to_string(),
    ));
    Ok((StatusCode::NO_CONTENT, jar))
}

async fn session(AuthUser(user, sess): AuthUser) -> Result<Json<SessionPayload>, AppError> {
    Ok(Json(SessionPayload {
        user: UserResponse::from(user),
        session: sess.into(),
    }))
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
