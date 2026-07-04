use sea_orm::*;

use crate::features::user::application::ports::user_repository::UserRepository;
use crate::features::user::domain::{Email, User};
use crate::features::user::infrastructure::error::UserInfraError;
use crate::features::user::infrastructure::mappers::UserMapper;
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
        let active = UserMapper::to_active_model(user);
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
            row.map(UserMapper::to_domain)
                .transpose()
                .map_err(AppError::from)
        })
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AppError> {
        let stmt = Entity::find().filter(Column::Email.eq(email.as_str()));
        with_conn!(self, conn, {
            let row = stmt
                .one(&conn)
                .await
                .map_err(|e| AppError::from(UserInfraError::from(e)))?;
            row.map(UserMapper::to_domain)
                .transpose()
                .map_err(AppError::from)
        })
    }
}
