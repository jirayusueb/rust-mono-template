use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::features::todo::domain::Status;

#[derive(Debug, Serialize, ts_rs::TS)]
#[ts(export, export_to = "../../web/src/lib/contract.ts")]
pub struct TodoResponse {
    pub id: String,
    pub title: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
