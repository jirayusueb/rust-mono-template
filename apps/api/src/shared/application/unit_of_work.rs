use std::future::Future;
use std::pin::Pin;

use async_trait::async_trait;

use crate::shared::kernel::error::AppError;

/// Allows the application layer to define a transaction boundary
/// without knowing how it is implemented (Dependency Rule).
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UnitOfWork: Send + Sync {
    /// Run `work` inside a transaction. Commits on `Ok`, rolls back on `Err`.
    async fn run_in_transaction(
        &self,
        work: Pin<Box<dyn Future<Output = Result<(), AppError>> + Send>>,
    ) -> Result<(), AppError>;
}

/// No-op implementation for tests — runs work without a real transaction.
pub struct NoopUnitOfWork;

#[async_trait]
impl UnitOfWork for NoopUnitOfWork {
    async fn run_in_transaction(
        &self,
        work: Pin<Box<dyn Future<Output = Result<(), AppError>> + Send>>,
    ) -> Result<(), AppError> {
        work.await
    }
}
