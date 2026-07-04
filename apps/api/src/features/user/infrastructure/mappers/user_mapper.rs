use sea_orm::ActiveValue::Set;

use crate::features::user::domain::{Email, User};
use crate::features::user::infrastructure::error::UserInfraError;
use crate::features::user::infrastructure::schema::user::{ActiveModel, Model};

/// Maps between database rows and domain `User` entities.
pub struct UserMapper;

impl UserMapper {
    /// Rehydrates a domain `User` from a trusted DB row (bypasses validation).
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

    /// Converts a domain `User` into an `ActiveModel` for persistence.
    pub fn to_active_model(user: &User) -> ActiveModel {
        ActiveModel {
            id: Set(user.id.into()),
            email: Set(user.email.as_str().to_string()),
            email_verified: Set(user.email_verified),
            name: Set(user.name.clone()),
            image: Set(user.image.clone()),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        }
    }
}
