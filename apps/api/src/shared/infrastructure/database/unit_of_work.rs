use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{DatabaseTransaction, TransactionTrait};
use tokio::sync::Mutex;

use crate::shared::application::unit_of_work::UnitOfWork;
use crate::shared::infrastructure::database::tx::CURRENT_TX;
use crate::shared::infrastructure::database::DbPool;
use crate::shared::kernel::error::AppError;

/// SeaORM implementation of UnitOfWork.
///
/// Begins a DB transaction, stores it in a task-local slot (so repositories
/// transparently pick it up via `with_conn!`), runs the work, then commits
/// on Ok or drops (auto-rollback) on Err.
pub struct SeaOrmUnitOfWork {
    db: DbPool,
}

impl SeaOrmUnitOfWork {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UnitOfWork for SeaOrmUnitOfWork {
    async fn run_in_transaction(
        &self,
        work: Pin<Box<dyn Future<Output = Result<(), AppError>> + Send>>,
    ) -> Result<(), AppError> {
        let tx: DatabaseTransaction = self
            .db
            .begin()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let slot: Arc<Mutex<Option<DatabaseTransaction>>> = Arc::new(Mutex::new(Some(tx)));

        let slot_clone = slot.clone();
        let result = CURRENT_TX.scope(slot, work).await;

        let mut guard = slot_clone.lock().await;
        let tx = guard
            .take()
            .expect("transaction taken before commit — concurrent UoW on same task?");

        match result {
            Ok(()) => {
                tx.commit()
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?;
                Ok(())
            }
            Err(e) => {
                // Drop rolls back. SeaORM's DatabaseTransaction starts rollback on Drop.
                drop(tx);
                Err(e)
            }
        }
    }
}
