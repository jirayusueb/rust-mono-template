use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, ts_rs::TS, utoipa::ToSchema)]
#[ts(export, export_to = "../../web/src/lib/dto.ts")]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
