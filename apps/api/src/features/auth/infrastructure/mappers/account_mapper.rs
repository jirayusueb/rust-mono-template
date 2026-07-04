use sea_orm::ActiveValue::Set;

use crate::features::auth::domain::Account;
use crate::features::auth::infrastructure::error::AuthInfraError;
use crate::features::auth::infrastructure::schema::account::{ActiveModel, Model};

/// Maps between database rows and domain `Account` entities.
pub struct AccountMapper;

impl AccountMapper {
    /// Rehydrates a domain `Account` from a trusted DB row (bypasses validation).
    pub fn to_domain(row: Model) -> Result<Account, AuthInfraError> {
        Ok(Account::restore(
            row.id.into(),
            row.user_id.into(),
            row.provider_id,
            row.account_id,
            row.password,
            row.created_at,
            row.updated_at,
        ))
    }

    /// Converts a domain `Account` into an `ActiveModel` for persistence.
    pub fn to_active_model(account: &Account) -> ActiveModel {
        ActiveModel {
            id: Set(account.id.into()),
            user_id: Set(account.user_id.into()),
            account_id: Set(account.account_id.clone()),
            provider_id: Set(account.provider_id.clone()),
            password: Set(account.password.clone()),
            access_token: Set(None),
            refresh_token: Set(None),
            id_token: Set(None),
            access_token_expires_at: Set(None),
            refresh_token_expires_at: Set(None),
            scope: Set(None),
            created_at: Set(account.created_at),
            updated_at: Set(account.updated_at),
        }
    }
}
