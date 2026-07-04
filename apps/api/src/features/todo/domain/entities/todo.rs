use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::shared::kernel::UserId;

use super::super::error::TodoDomainError;
use super::super::values::status::Status;
use super::super::values::title::Title;
use super::super::values::todo_id::TodoId;

#[derive(Debug, Clone, Serialize)]
pub struct Todo {
    pub id: TodoId,
    pub user_id: UserId,
    pub(crate) title: Title,
    pub(crate) status: Status,
    pub created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Todo {
    /// Factory for a NEW todo (runs validation, stamps timestamps).
    pub fn create(user_id: UserId, title: Title) -> Result<Self, TodoDomainError> {
        let now = Utc::now();
        Ok(Self {
            id: TodoId::new(),
            user_id,
            title,
            status: Status::Pending,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        })
    }

    /// Factory for an EXISTING todo (from trusted DB data). Bypasses validation.
    pub fn restore(
        id: TodoId,
        user_id: UserId,
        title: Title,
        status: Status,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            user_id,
            title,
            status,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn complete(&mut self) -> Result<(), TodoDomainError> {
        if self.status == Status::Completed {
            return Err(TodoDomainError::AlreadyCompleted);
        }
        self.status = Status::Completed;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn reopen(&mut self) -> Result<(), TodoDomainError> {
        if self.status != Status::Completed {
            return Err(TodoDomainError::NotCompleted);
        }
        self.status = Status::Pending;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn rename(&mut self, title: Title) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    /// Soft-delete: stamp `deleted_at` instead of removing the row.
    pub fn delete(&mut self) {
        let now = Utc::now();
        self.deleted_at = Some(now);
        self.updated_at = now;
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_todo() -> Todo {
        Todo::create(UserId::new(), Title::new("test".into()).unwrap()).unwrap()
    }

    #[test]
    fn new_todo_starts_pending() {
        let todo = make_todo();
        assert_eq!(todo.status, Status::Pending);
    }

    #[test]
    fn complete_transitions_to_completed() {
        let mut todo = make_todo();
        assert!(todo.complete().is_ok());
        assert_eq!(todo.status, Status::Completed);
    }

    #[test]
    fn complete_when_already_completed_errors() {
        let mut todo = make_todo();
        todo.complete().unwrap();
        assert_eq!(
            todo.complete().unwrap_err(),
            TodoDomainError::AlreadyCompleted
        );
    }

    #[test]
    fn reopen_transitions_to_pending() {
        let mut todo = make_todo();
        todo.complete().unwrap();
        assert!(todo.reopen().is_ok());
        assert_eq!(todo.status, Status::Pending);
    }

    #[test]
    fn reopen_when_pending_errors() {
        let mut todo = make_todo();
        assert_eq!(todo.reopen().unwrap_err(), TodoDomainError::NotCompleted);
    }

    #[test]
    fn delete_stamps_deleted_at_and_marks_deleted() {
        let mut todo = make_todo();
        assert!(!todo.is_deleted());
        assert!(todo.deleted_at.is_none());

        let before = Utc::now();
        todo.delete();

        assert!(todo.is_deleted());
        let stamped = todo.deleted_at.expect("deleted_at should be set");
        assert!(stamped >= before);
    }
}
