use crate::features::todo::application::dtos::{
    CreateTodoInput, CreateTodoOutput, GetTodoOutput, UpdateTodoInput,
};
use crate::features::todo::domain::{Title, TodoId};
use crate::features::todo::presentation::http::dtos::{
    CreateTodoRequest, CreateTodoResponse, TodoResponse, UpdateTodoRequest,
};
use crate::shared::kernel::error::AppError;
use crate::shared::kernel::UserId;

/// Maps between transport DTOs and application-layer types for the todo feature.
pub struct TodoMapper;

impl TodoMapper {
    pub fn to_create_input(
        req: CreateTodoRequest,
        user_id: UserId,
    ) -> Result<CreateTodoInput, AppError> {
        Ok(CreateTodoInput {
            user_id,
            title: Title::new(req.title)?,
        })
    }

    pub fn to_update_input(
        req: UpdateTodoRequest,
        user_id: UserId,
        id: TodoId,
    ) -> Result<UpdateTodoInput, AppError> {
        Ok(UpdateTodoInput {
            user_id,
            id,
            title: match req.title {
                Some(raw) => Some(Title::new(raw)?),
                None => None,
            },
            status: req.status,
        })
    }

    pub fn from_create_output(output: CreateTodoOutput) -> CreateTodoResponse {
        CreateTodoResponse { id: output.id }
    }

    pub fn from_get_output(output: GetTodoOutput) -> TodoResponse {
        TodoResponse {
            id: output.id,
            title: output.title,
            status: output.status,
            created_at: output.created_at,
            updated_at: output.updated_at,
        }
    }
}
