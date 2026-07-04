use chrono::{DateTime, Utc};

/// Auth's view of a session — no token (that stays in the cookie).
#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}
