use serde::Serialize;

use super::{SessionDetails, UserResponse};

/// Full response for `GET /auth/session` — wraps user info + session metadata.
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub user: UserResponse,
    pub session: SessionDetails,
}
