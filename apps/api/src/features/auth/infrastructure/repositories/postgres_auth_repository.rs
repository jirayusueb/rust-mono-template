use sea_orm::*;
use uuid::Uuid;

use crate::features::auth::application::ports::auth_repository::AuthRepository;
use crate::features::auth::domain::{Account, Session};
use crate::features::auth::infrastructure::error::AuthInfraError;
use crate::features::auth::infrastructure::mapper::account_mapper::to_domain as account_to_domain;
use crate::features::auth::infrastructure::mapper::session_mapper::to_domain as session_to_domain;
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
    async fn save_credential(&self, account_model: &Account) -> Result<(), AppError> {
        let active = account::ActiveModel {
            id: Set(account_model.id.into()),
            user_id: Set(account_model.user_id.into()),
            account_id: Set(account_model.account_id.clone()),
            provider_id: Set(account_model.provider_id.clone()),
            password: Set(account_model.password.clone()),
            access_token: Set(None),
            refresh_token: Set(None),
            id_token: Set(None),
            access_token_expires_at: Set(None),
            refresh_token_expires_at: Set(None),
            scope: Set(None),
            created_at: Set(account_model.created_at),
            updated_at: Set(account_model.updated_at),
        };
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
            row.map(account_to_domain)
                .transpose()
                .map_err(AppError::from)
        })
    }

    async fn save_session(&self, session_model: &Session) -> Result<(), AppError> {
        let active = session::ActiveModel {
            id: Set(session_model.id.clone()),
            token: Set(session_model.token.clone()),
            user_id: Set(session_model.user_id.into()),
            expires_at: Set(session_model.expires_at),
            ip_address: Set(session_model.ip_address.clone()),
            user_agent: Set(session_model.user_agent.clone()),
            created_at: Set(session_model.created_at),
            updated_at: Set(session_model.updated_at),
        };
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
            row.map(session_to_domain)
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
