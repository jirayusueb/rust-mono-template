use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::shared::kernel::UserId;

use super::super::values::email::Email;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub email_verified: bool,
    pub name: Option<String>,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: Email, name: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            email,
            email_verified: false,
            name,
            image: None,
            created_at: now,
            updated_at: now,
        }
    }
}
