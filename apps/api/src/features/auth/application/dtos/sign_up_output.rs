use crate::features::auth::application::ports::user_port::AuthUserInfo;

/// Output of sign-up: the new user + the session token to set as cookie.
pub struct SignUpOutput {
    pub user: AuthUserInfo,
    pub token: String,
}
