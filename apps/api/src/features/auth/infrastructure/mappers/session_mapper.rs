use sea_orm::ActiveValue::Set;

use crate::features::auth::domain::Session;
use crate::features::auth::infrastructure::error::AuthInfraError;
use crate::features::auth::infrastructure::schema::session::{ActiveModel, Model};

/// Maps between database rows and domain `Session` entities.
pub struct SessionMapper;

impl SessionMapper {
    /// Rehydrates a domain `Session` from a trusted DB row (bypasses validation).
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

    /// Converts a domain `Session` into an `ActiveModel` for persistence.
    pub fn to_active_model(session: &Session) -> ActiveModel {
        ActiveModel {
            id: Set(session.id.clone()),
            token: Set(session.token.clone()),
            user_id: Set(session.user_id.into()),
            expires_at: Set(session.expires_at),
            ip_address: Set(session.ip_address.clone()),
            user_agent: Set(session.user_agent.clone()),
            created_at: Set(session.created_at),
            updated_at: Set(session.updated_at),
        }
    }
}
