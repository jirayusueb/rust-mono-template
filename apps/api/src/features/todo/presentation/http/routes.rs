use std::str::FromStr;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use validator::Validate;

use crate::bootstrap::AppState;
use crate::features::auth::presentation::http::middleware::AuthUser;
use crate::features::todo::application::commands::create::CreateTodoHandler;
use crate::features::todo::application::commands::delete::DeleteTodoHandler;
use crate::features::todo::application::commands::update::UpdateTodoHandler;
use crate::features::todo::application::dtos::{
    DeleteTodoCommand, GetTodoQuery, ListTodosQuery,
};
use crate::features::todo::application::queries::get::GetTodoHandler;
use crate::features::todo::application::queries::list::ListTodosHandler;
use crate::features::todo::domain::TodoId;
use crate::shared::kernel::error::AppError;

use super::dtos::{CreateTodoRequest, CreateTodoResponse, TodoResponse, UpdateTodoRequest};
use super::mappers::TodoMapper;

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
}

async fn create_todo(
    AuthUser(auth, _): AuthUser,
    State(state): State<AppState>,
    Json(req): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<CreateTodoResponse>), AppError> {
    req.validate()?;
    let handler = CreateTodoHandler::new(state.todo_repo.clone());
    let id = handler
        .handle(TodoMapper::to_create_command(req, auth.id)?)
        .await?;
    Ok((
        StatusCode::CREATED,
        Json(TodoMapper::to_create_response(id)),
    ))
}

async fn list_todos(
    AuthUser(auth, _): AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    let handler = ListTodosHandler::new(state.todo_repo.clone());
    let todos = handler.handle(ListTodosQuery { user_id: auth.id }).await?;
    Ok(Json(
        todos
            .into_iter()
            .map(TodoMapper::to_todo_response)
            .collect(),
    ))
}

async fn get_todo(
    AuthUser(auth, _): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TodoResponse>, AppError> {
    let id = TodoId::from_str(&id)?;
    let handler = GetTodoHandler::new(state.todo_repo.clone());
    let todo = handler
        .handle(GetTodoQuery {
            user_id: auth.id,
            id,
        })
        .await?;
    Ok(Json(TodoMapper::to_todo_response(todo)))
}

async fn update_todo(
    AuthUser(auth, _): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTodoRequest>,
) -> Result<StatusCode, AppError> {
    req.validate()?;
    let id = TodoId::from_str(&id)?;
    let handler = UpdateTodoHandler::new(state.todo_repo.clone());
    handler
        .handle(TodoMapper::to_update_command(req, auth.id, id)?)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_todo(
    AuthUser(auth, _): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let id = TodoId::from_str(&id)?;
    let handler = DeleteTodoHandler::new(state.todo_repo.clone());
    handler
        .handle(DeleteTodoCommand {
            user_id: auth.id,
            id,
        })
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
