use crate::features::auth::domain::Account;
use crate::features::auth::infrastructure::error::AuthInfraError;
use crate::features::auth::infrastructure::schema::account::Model;

/// Maps a database row into a domain `Account`.
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
