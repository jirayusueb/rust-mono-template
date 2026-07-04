use crate::features::todo::domain::Todo;

use super::dtos::TodoResponse;

impl From<Todo> for TodoResponse {
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
