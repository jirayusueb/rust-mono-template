use chrono::{DateTime, Utc};

use crate::features::todo::domain::{Status, Todo};

/// Flat, persistence-agnostic view of a single todo. Shared by Get and List outputs.
#[derive(Debug)]
pub struct GetTodoOutput {
    pub id: String,
    pub title: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Todo> for GetTodoOutput {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id.to_string(),
            title: todo.title.as_str().to_string(),
            status: todo.status,
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }
}
