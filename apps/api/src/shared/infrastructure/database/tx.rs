use sea_orm::{
    ConnectionTrait, DatabaseBackend, DatabaseTransaction, DbErr, ExecResult, QueryResult,
    Statement,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::shared::infrastructure::database::DbPool;

/// Unified connection reference — either the pooled connection or an active transaction.
/// Both variants implement `ConnectionTrait`, so queries work the same either way.
pub enum Conn<'a> {
    Pool(&'a DbPool),
    Tx(&'a DatabaseTransaction),
}

#[async_trait::async_trait]
impl ConnectionTrait for Conn<'_> {
    fn get_database_backend(&self) -> DatabaseBackend {
        match self {
            Conn::Pool(c) => c.get_database_backend(),
            Conn::Tx(c) => c.get_database_backend(),
        }
    }

    async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
        match self {
            Conn::Pool(c) => c.execute(stmt).await,
            Conn::Tx(c) => c.execute(stmt).await,
        }
    }

    async fn execute_unprepared(&self, sql: &str) -> Result<ExecResult, DbErr> {
        match self {
            Conn::Pool(c) => c.execute_unprepared(sql).await,
            Conn::Tx(c) => c.execute_unprepared(sql).await,
        }
    }

    async fn query_one(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr> {
        match self {
            Conn::Pool(c) => c.query_one(stmt).await,
            Conn::Tx(c) => c.query_one(stmt).await,
        }
    }

    async fn query_all(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr> {
        match self {
            Conn::Pool(c) => c.query_all(stmt).await,
            Conn::Tx(c) => c.query_all(stmt).await,
        }
    }
}

// Task-local slot holding the active transaction (if any).
//
// Rust equivalent of Node's AsyncLocalStorage — set by `SeaOrmUnitOfWork`,
// checked by every repository via the `with_conn!` macro.
tokio::task_local! {
    /// `None` = no active txn (use pooled connection); `Some` = use this txn.
    pub static CURRENT_TX: Arc<Mutex<Option<DatabaseTransaction>>>;
}

/// Runs `body` against the active transaction if one exists, otherwise against
/// the pooled connection.
///
/// The `MutexGuard` must live across the `.await` inside `$body`, so this is a
/// macro — returning a borrow from behind the guard would not compile.
#[macro_export]
macro_rules! with_conn {
    ($self:ident, $conn:ident, $body:expr) => {
        if let Ok(slot) = $crate::shared::infrastructure::database::tx::CURRENT_TX.try_get() {
            let guard = slot.lock().await;
            if let Some(ref tx) = *guard {
                let $conn = $crate::shared::infrastructure::database::tx::Conn::Tx(tx);
                $body
            } else {
                let $conn = $crate::shared::infrastructure::database::tx::Conn::Pool(&$self.db);
                $body
            }
        } else {
            let $conn = $crate::shared::infrastructure::database::tx::Conn::Pool(&$self.db);
            $body
        }
    };
}
