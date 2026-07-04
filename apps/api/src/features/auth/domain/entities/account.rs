use chrono::{DateTime, Utc};

use crate::shared::kernel::UserId;

use super::super::values::account_id::AccountId;

/// An authentication method linked to a User.
/// For credential auth: provider_id = "credential", password = Some(hash).
/// For OAuth: provider_id = provider name, password = None, token fields populated.
#[derive(Debug, Clone)]
pub struct Account {
    pub id: AccountId,
    pub user_id: UserId,
    pub provider_id: String,
    pub account_id: String,
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Account {
    pub fn new_credential(user_id: UserId, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: AccountId::new(),
            user_id,
            provider_id: "credential".to_string(),
            account_id: user_id.to_string(),
            password: Some(password_hash),
            created_at: now,
            updated_at: now,
        }
    }
}
