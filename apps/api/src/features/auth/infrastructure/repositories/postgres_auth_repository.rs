use sea_orm::*;
use uuid::Uuid;

use crate::features::auth::application::ports::auth_repository::AuthRepository;
use crate::features::auth::domain::{Account, Session};
use crate::features::auth::infrastructure::error::AuthInfraError;
use crate::features::auth::infrastructure::mappers::{AccountMapper, SessionMapper};
use crate::features::auth::infrastructure::schema::{account, session};
use crate::shared::infrastructure::database::DbPool;
use crate::shared::kernel::error::AppError;
use crate::shared::kernel::UserId;
use crate::with_conn;

pub struct PostgresAuthRepository {
    pub(super) db: DbPool,
}

impl PostgresAuthRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl AuthRepository for PostgresAuthRepository {
    async fn save_credential(&self, account: &Account) -> Result<(), AppError> {
        let active = AccountMapper::to_active_model(account);
        let stmt = account::Entity::insert(active);
        with_conn!(self, conn, {
            stmt.exec(&conn)
                .await
                .map_err(|e| AppError::from(AuthInfraError::from(e)))?;
            Ok(())
        })
    }

    async fn find_credential_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Option<Account>, AppError> {
        let stmt = account::Entity::find()
            .filter(account::Column::UserId.eq(Uuid::from(*user_id)))
            .filter(account::Column::ProviderId.eq("credential"));
        with_conn!(self, conn, {
            let row = stmt
                .one(&conn)
                .await
                .map_err(|e| AppError::from(AuthInfraError::from(e)))?;
            row.map(AccountMapper::to_domain)
                .transpose()
                .map_err(AppError::from)
        })
    }

    async fn save_session(&self, session: &Session) -> Result<(), AppError> {
        let active = SessionMapper::to_active_model(session);
        let stmt = session::Entity::insert(active);
        with_conn!(self, conn, {
            stmt.exec(&conn)
                .await
                .map_err(|e| AppError::from(AuthInfraError::from(e)))?;
            Ok(())
        })
    }

    async fn find_session_by_token(&self, token: &str) -> Result<Option<Session>, AppError> {
        let stmt = session::Entity::find().filter(session::Column::Token.eq(token));
        with_conn!(self, conn, {
            let row = stmt
                .one(&conn)
                .await
                .map_err(|e| AppError::from(AuthInfraError::from(e)))?;
            row.map(SessionMapper::to_domain)
                .transpose()
                .map_err(AppError::from)
        })
    }

    async fn delete_session(&self, token: &str) -> Result<(), AppError> {
        let stmt = session::Entity::delete_many().filter(session::Column::Token.eq(token));
        with_conn!(self, conn, {
            stmt.exec(&conn)
                .await
                .map_err(|e| AppError::from(AuthInfraError::from(e)))?;
            Ok(())
        })
    }

    async fn update_session_expiry(&self, session: &Session) -> Result<(), AppError> {
        let stmt = session::Entity::update_many()
            .filter(session::Column::Token.eq(session.token.clone()))
            .col_expr(
                session::Column::ExpiresAt,
                sea_query::Expr::value(session.expires_at),
            )
            .col_expr(
                session::Column::UpdatedAt,
                sea_query::Expr::value(session.updated_at),
            );
        with_conn!(self, conn, {
            stmt.exec(&conn)
                .await
                .map_err(|e| AppError::from(AuthInfraError::from(e)))?;
            Ok(())
        })
    }
}
