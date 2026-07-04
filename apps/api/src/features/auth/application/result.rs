use crate::features::auth::application::ports::user_port::AuthUserInfo;

/// Result of sign-up/sign-in — the handler needs both the user and session token.
pub struct AuthResult {
    pub user: AuthUserInfo,
    pub token: String,
}
