use sea_orm::*;

use crate::features::user::application::ports::user_repository::UserRepository;
use crate::features::user::domain::{Email, User};
use crate::features::user::infrastructure::error::UserInfraError;
use crate::features::user::infrastructure::mapper::user_mapper::to_domain;
use crate::features::user::infrastructure::schema::user::*;
use crate::shared::infrastructure::database::DbPool;
use crate::shared::kernel::error::AppError;
use crate::shared::kernel::UserId;
use crate::with_conn;
use uuid::Uuid;

pub struct PostgresUserRepository {
    pub(super) db: DbPool,
}

impl PostgresUserRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: &User) -> Result<(), AppError> {
        let active = ActiveModel {
            id: Set(user.id.into()),
            email: Set(user.email.as_str().to_string()),
            email_verified: Set(user.email_verified),
            name: Set(user.name.clone()),
            image: Set(user.image.clone()),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        };
        let stmt = Entity::insert(active).on_conflict(
            sea_query::OnConflict::column(Column::Id)
                .update_columns([
                    Column::Email,
                    Column::EmailVerified,
                    Column::Name,
                    Column::Image,
                    Column::UpdatedAt,
                ])
                .to_owned(),
        );
        with_conn!(self, conn, {
            stmt.exec(&conn)
                .await
                .map_err(|e| AppError::from(UserInfraError::from(e)))?;
            Ok(())
        })
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, AppError> {
        let stmt = Entity::find().filter(Column::Id.eq(Uuid::from(*id)));
        with_conn!(self, conn, {
            let row = stmt
                .one(&conn)
                .await
                .map_err(|e| AppError::from(UserInfraError::from(e)))?;
            row.map(to_domain).transpose().map_err(AppError::from)
        })
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AppError> {
        let stmt = Entity::find().filter(Column::Email.eq(email.as_str()));
        with_conn!(self, conn, {
            let row = stmt
                .one(&conn)
                .await
                .map_err(|e| AppError::from(UserInfraError::from(e)))?;
            row.map(to_domain).transpose().map_err(AppError::from)
        })
    }
}
