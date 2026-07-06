use chrono::{DateTime, Utc};
use serde::Serialize;

/// Session metadata returned inside the full `SessionResponse`.
#[derive(Debug, Serialize, ts_rs::TS, utoipa::ToSchema)]
#[ts(export, export_to = "../../web/src/lib/dto.ts")]
pub struct SessionDetails {
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}
