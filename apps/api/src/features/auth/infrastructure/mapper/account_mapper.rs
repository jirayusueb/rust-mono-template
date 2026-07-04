use crate::features::auth::domain::Account;
use crate::features::auth::infrastructure::error::AuthInfraError;
use crate::features::auth::infrastructure::schema::account::Model;

/// Maps a database row into a domain `Account`.
pub fn to_domain(row: Model) -> Result<Account, AuthInfraError> {
    Ok(Account {
        id: row.id.into(),
        user_id: row.user_id.into(),
        account_id: row.account_id,
        provider_id: row.provider_id,
        password: row.password,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}
