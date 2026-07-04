use crate::features::auth::application::ports::session_info::SessionInfo;
use crate::features::auth::application::ports::user_port::AuthUserInfo;
use crate::features::auth::presentation::http::dtos::{SessionResponse, UserResponse};

impl From<AuthUserInfo> for UserResponse {
    fn from(user: AuthUserInfo) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email,
            email_verified: user.email_verified,
            name: user.name,
            image: user.image,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<SessionInfo> for SessionResponse {
    fn from(s: SessionInfo) -> Self {
        Self {
            expires_at: s.expires_at,
            ip_address: s.ip_address,
            user_agent: s.user_agent,
            created_at: s.created_at,
        }
    }
}
