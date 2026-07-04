use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::features::todo::domain::Status;

// --- Request DTOs ---

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTodoRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodoRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    // status is an enum — serde rejects bad values before validator runs
    pub status: Option<Status>,
}

// --- Response DTOs ---

#[derive(Debug, Serialize)]
pub struct TodoResponse {
    pub id: String,
    pub title: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CreateTodoResponse {
    pub id: String,
}
