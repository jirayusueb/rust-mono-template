use crate::features::user::domain::Email;
use crate::features::user::domain::User;
use crate::features::user::infrastructure::error::UserInfraError;
use crate::features::user::infrastructure::schema::user::Model;

/// Maps a database row into a domain `User`.
pub fn to_domain(row: Model) -> Result<User, UserInfraError> {
    Ok(User::restore(
        row.id.into(),
        Email::restore(row.email),
        row.email_verified,
        row.name,
        row.image,
        row.created_at,
        row.updated_at,
    ))
}
