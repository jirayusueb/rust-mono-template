use crate::features::auth::domain::Session;
use crate::features::auth::infrastructure::error::AuthInfraError;
use crate::features::auth::infrastructure::schema::session::Model;

/// Maps a database row into a domain `Session`.
pub fn to_domain(row: Model) -> Result<Session, AuthInfraError> {
    Ok(Session {
        id: row.id,
        token: row.token,
        user_id: row.user_id.into(),
        expires_at: row.expires_at,
        ip_address: row.ip_address,
        user_agent: row.user_agent,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}
