use sea_orm::sea_query::{Expr, OnConflict, Value};
use sea_orm::{ColumnTrait, DatabaseBackend, EntityTrait, Order, QueryFilter, QueryOrder, Statement};
use uuid::Uuid;

use crate::features::todo::application::ports::todo_repository::TodoRepository;
use crate::features::todo::domain::{Todo, TodoId};
use crate::features::todo::infrastructure::error::TodoInfraError;
use crate::features::todo::infrastructure::mappers::TodoMapper;
use crate::features::todo::infrastructure::schema::todo::*;
use crate::shared::infrastructure::database::DbPool;
use crate::shared::kernel::error::AppError;
use crate::shared::kernel::UserId;
use crate::with_conn;

pub struct PostgresTodoRepository {
    pub(super) db: DbPool,
}

impl PostgresTodoRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl TodoRepository for PostgresTodoRepository {
    async fn save(&self, todo: &Todo) -> Result<(), AppError> {
        let active = TodoMapper::to_active_model(todo);
        let stmt = Entity::insert(active).on_conflict(
            OnConflict::column(Column::Id)
                // ponytail: deleted_at intentionally excluded — saves must not un-delete.
                .update_columns([Column::Title, Column::Status, Column::UpdatedAt])
                .to_owned(),
        );
        with_conn!(self, conn, {
            stmt.exec(&conn)
                .await
                .map_err(|e| AppError::from(TodoInfraError::from(e)))?;
            Ok(())
        })
    }

    async fn find_by_id(&self, id: &TodoId, user_id: &UserId) -> Result<Option<Todo>, AppError> {
        let stmt = Entity::find()
            .filter(Column::Id.eq(Uuid::from(*id)))
            .filter(Column::UserId.eq(Uuid::from(*user_id)))
            .filter(Column::DeletedAt.is_null());
        with_conn!(self, conn, {
            let row = stmt
                .one(&conn)
                .await
                .map_err(|e| AppError::from(TodoInfraError::from(e)))?;
            row.map(TodoMapper::to_domain)
                .transpose()
                .map_err(AppError::from)
        })
    }

    async fn find_all(&self, user_id: &UserId) -> Result<Vec<Todo>, AppError> {
        let stmt = Entity::find()
            .filter(Column::UserId.eq(Uuid::from(*user_id)))
            .filter(Column::DeletedAt.is_null())
            .order_by(Column::CreatedAt, Order::Asc);
        with_conn!(self, conn, {
            let rows = stmt
                .all(&conn)
                .await
                .map_err(|e| AppError::from(TodoInfraError::from(e)))?;
            rows.into_iter()
                .map(TodoMapper::to_domain)
                .collect::<Result<Vec<_>, TodoInfraError>>()
                .map_err(AppError::from)
        })
    }

    async fn delete(&self, id: &TodoId, user_id: &UserId) -> Result<(), AppError> {
        let stmt = Entity::update_many()
            .col_expr(Column::DeletedAt, Expr::current_timestamp().into())
            .col_expr(Column::UpdatedAt, Expr::current_timestamp().into())
            .filter(Column::Id.eq(Uuid::from(*id)))
            .filter(Column::UserId.eq(Uuid::from(*user_id)))
            .filter(Column::DeletedAt.is_null());
        with_conn!(self, conn, {
            stmt.exec(&conn)
                .await
                .map_err(|e| AppError::from(TodoInfraError::from(e)))?;
            Ok(())
        })
    }

    // ponytail: BM25 via ts_rank_cd, LIMIT hardcap — add pagination when lists exceed a few hundred
    async fn search(&self, user_id: &UserId, query: &str, limit: u64) -> Result<Vec<Todo>, AppError> {
        let stmt = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"
                SELECT * FROM todos
                WHERE user_id = $1 AND deleted_at IS NULL
                  AND title_tsv @@ plainto_tsquery('english', $2)
                ORDER BY ts_rank_cd(title_tsv, plainto_tsquery('english', $2)) DESC
                LIMIT $3
            "#,
            [
                Value::Uuid(Some(Box::new(Uuid::from(*user_id)))),
                Value::String(Some(Box::new(query.into()))),
                Value::BigInt(Some(limit as i64)),
            ],
        );
        with_conn!(self, conn, {
            let rows = Entity::find()
                .from_raw_sql(stmt)
                .all(&conn)
                .await
                .map_err(|e| AppError::from(TodoInfraError::from(e)))?;
            rows.into_iter()
                .map(TodoMapper::to_domain)
                .collect::<Result<Vec<_>, TodoInfraError>>()
                .map_err(AppError::from)
        })
    }
}
