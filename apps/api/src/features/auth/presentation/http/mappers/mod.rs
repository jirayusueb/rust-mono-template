use crate::features::auth::application::dtos::{SignInCommand, SignUpCommand};
use crate::features::auth::application::ports::session_info::SessionInfo;
use crate::features::auth::application::ports::user_port::AuthUserInfo;
use crate::features::auth::presentation::http::dtos::{
    SessionDetails, SessionResponse, SignInRequest, SignUpRequest, UserResponse,
};

/// Maps between transport DTOs and application-layer types for the auth feature.
pub struct AuthMapper;

impl AuthMapper {
    pub fn to_sign_up_command(req: SignUpRequest) -> SignUpCommand {
        SignUpCommand {
            email: req.email,
            password: req.password,
            name: req.name,
        }
    }

    pub fn to_sign_in_command(
        req: SignInRequest,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> SignInCommand {
        SignInCommand {
            email: req.email,
            password: req.password,
            ip_address,
            user_agent,
        }
    }

    pub fn to_user_response(user: AuthUserInfo) -> UserResponse {
        UserResponse {
            id: user.id.to_string(),
            email: user.email,
            email_verified: user.email_verified,
            name: user.name,
            image: user.image,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }

    pub fn to_session_details(sess: SessionInfo) -> SessionDetails {
        SessionDetails {
            expires_at: sess.expires_at,
            ip_address: sess.ip_address,
            user_agent: sess.user_agent,
            created_at: sess.created_at,
        }
    }

    pub fn to_session_response(user: AuthUserInfo, sess: SessionInfo) -> SessionResponse {
        SessionResponse {
            user: Self::to_user_response(user),
            session: Self::to_session_details(sess),
        }
    }
}
