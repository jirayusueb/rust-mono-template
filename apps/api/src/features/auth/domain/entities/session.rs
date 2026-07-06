use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::shared::kernel::UserId;

/// Session expiry: 7 days.
const SESSION_DURATION_DAYS: i64 = 7;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub token: String,
    pub user_id: UserId,
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    /// Factory for a NEW session (generates token, stamps timestamps).
    pub fn create(user_id: UserId, ip_address: Option<String>, user_agent: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7().to_string(),
            token: Uuid::new_v4().to_string(),
            user_id,
            expires_at: now + Duration::days(SESSION_DURATION_DAYS),
            ip_address,
            user_agent,
            created_at: now,
            updated_at: now,
        }
    }

    /// Factory for an EXISTING session (from trusted DB data). Bypasses validation.
    #[allow(clippy::too_many_arguments)]
    pub fn restore(
        id: String,
        token: String,
        user_id: UserId,
        expires_at: DateTime<Utc>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            token,
            user_id,
            expires_at,
            ip_address,
            user_agent,
            created_at,
            updated_at,
        }
    }

    pub fn is_expired_at(&self, now: &DateTime<Utc>) -> bool {
        *now >= self.expires_at
    }

    /// Extend expiry to a fresh window from now.
    pub fn refresh(&mut self) {
        let now = Utc::now();
        self.expires_at = now + Duration::days(SESSION_DURATION_DAYS);
        self.updated_at = now;
    }

    /// True when less than half the session lifetime remains.
    pub fn needs_refresh(&self) -> bool {
        let now = Utc::now();
        let remaining = self.expires_at - now;
        remaining.num_seconds() < (SESSION_DURATION_DAYS * 24 * 3600 / 2)
    }
}
