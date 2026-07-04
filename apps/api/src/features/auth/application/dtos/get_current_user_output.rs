use crate::features::auth::application::ports::session_info::SessionInfo;
use crate::features::auth::application::ports::user_port::AuthUserInfo;

/// Output of get-current-user: the authenticated user + their session metadata.
pub struct GetCurrentUserOutput {
    pub user: AuthUserInfo,
    pub session: SessionInfo,
}
