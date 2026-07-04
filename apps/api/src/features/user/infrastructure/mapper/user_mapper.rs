use crate::features::user::domain::Email;
use crate::features::user::domain::User;
use crate::features::user::infrastructure::error::UserInfraError;
use crate::features::user::infrastructure::schema::user::Model;

/// Maps a database row into a domain `User`.
pub fn to_domain(row: Model) -> Result<User, UserInfraError> {
    Ok(User {
        id: row.id.into(),
        email: Email::new(row.email).map_err(|_| UserInfraError::Decode("invalid email".into()))?,
        email_verified: row.email_verified,
        name: row.name,
        image: row.image,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}
