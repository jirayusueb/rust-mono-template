use sea_orm::ActiveValue::Set;

use crate::features::todo::domain::{Status, Title, Todo};
use crate::features::todo::infrastructure::error::TodoInfraError;
use crate::features::todo::infrastructure::schema::todo::{ActiveModel, Model};

/// Maps between database rows and domain `Todo` entities.
pub struct TodoMapper;

impl TodoMapper {
    /// Rehydrates a domain `Todo` from a trusted DB row (bypasses validation).
    pub fn to_domain(row: Model) -> Result<Todo, TodoInfraError> {
        let status = match row.status.as_str() {
            "pending" => Status::Pending,
            "completed" => Status::Completed,
            _ => {
                return Err(TodoInfraError::Decode(format!(
                    "invalid status: {}",
                    row.status
                )))
            }
        };
        Ok(Todo::restore(
            row.id.into(),
            row.user_id.into(),
            Title::restore(row.title),
            status,
            row.created_at,
            row.updated_at,
            row.deleted_at,
        ))
    }

    /// Converts a domain `Todo` into an `ActiveModel` for persistence.
    pub fn to_active_model(todo: &Todo) -> ActiveModel {
        ActiveModel {
            id: Set(todo.id.into()),
            title: Set(todo.title.as_str().to_string()),
            status: Set(todo.status.to_string()),
            created_at: Set(todo.created_at),
            updated_at: Set(todo.updated_at),
            deleted_at: Set(todo.deleted_at),
            user_id: Set(todo.user_id.into()),
        }
    }
}
