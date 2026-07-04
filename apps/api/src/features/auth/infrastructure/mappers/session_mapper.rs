use crate::features::auth::domain::Session;
use crate::features::auth::infrastructure::error::AuthInfraError;
use crate::features::auth::infrastructure::schema::session::Model;

/// Maps a database row into a domain `Session`.
pub fn to_domain(row: Model) -> Result<Session, AuthInfraError> {
    Ok(Session::restore(
        row.id,
        row.token,
        row.user_id.into(),
        row.expires_at,
        row.ip_address,
        row.user_agent,
        row.created_at,
        row.updated_at,
    ))
}
