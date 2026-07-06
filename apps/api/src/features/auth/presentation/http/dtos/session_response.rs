use serde::Serialize;

use super::{SessionDetails, UserResponse};

/// Full response for `GET /auth/session` — wraps user info + session metadata.
#[derive(Debug, Serialize, ts_rs::TS, utoipa::ToSchema)]
#[ts(export, export_to = "../../web/src/lib/dto.ts")]
pub struct SessionResponse {
    pub user: UserResponse,
    pub session: SessionDetails,
}
