use crate::features::todo::application::dtos::{CreateTodoCommand, UpdateTodoCommand};
use crate::features::todo::domain::{Title, Todo, TodoId};
use crate::features::todo::presentation::http::dtos::{
    CreateTodoRequest, CreateTodoResponse, TodoResponse, UpdateTodoRequest,
};
use crate::shared::kernel::error::AppError;
use crate::shared::kernel::UserId;

/// Maps between transport DTOs and application-layer types for the todo feature.
pub struct TodoMapper;

impl TodoMapper {
    pub fn to_create_command(
        req: CreateTodoRequest,
        user_id: UserId,
    ) -> Result<CreateTodoCommand, AppError> {
        Ok(CreateTodoCommand {
            user_id,
            title: Title::new(req.title)?,
        })
    }

    pub fn to_update_command(
        req: UpdateTodoRequest,
        user_id: UserId,
        id: TodoId,
    ) -> Result<UpdateTodoCommand, AppError> {
        Ok(UpdateTodoCommand {
            user_id,
            id,
            title: match req.title {
                Some(raw) => Some(Title::new(raw)?),
                None => None,
            },
            status: req.status,
        })
    }

    pub fn to_todo_response(todo: Todo) -> TodoResponse {
        TodoResponse {
            id: todo.id.to_string(),
            title: todo.title.as_str().to_string(),
            status: todo.status,
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }

    pub fn to_create_response(id: TodoId) -> CreateTodoResponse {
        CreateTodoResponse { id: id.to_string() }
    }
}
