use crate::features::auth::application::ports::user_port::AuthUserInfo;

/// Output of sign-in: the authenticated user + the session token to set as cookie.
pub struct SignInOutput {
    pub user: AuthUserInfo,
    pub token: String,
}
