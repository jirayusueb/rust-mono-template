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
    CreateTodo, DeleteTodo, GetTodo, ListTodos, UpdateTodo,
};
use crate::features::todo::application::queries::get::GetTodoHandler;
use crate::features::todo::application::queries::list::ListTodosHandler;
use crate::features::todo::domain::{Title, TodoId};
use crate::shared::kernel::error::AppError;

use super::dtos::{CreateTodoRequest, CreateTodoResponse, TodoResponse, UpdateTodoRequest};

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
    let title = Title::new(req.title)?;
    let handler = CreateTodoHandler::new(state.todo_repo.clone());
    let id = handler
        .handle(CreateTodo {
            user_id: auth.id,
            title,
        })
        .await?;
    Ok((
        StatusCode::CREATED,
        Json(CreateTodoResponse { id: id.to_string() }),
    ))
}

async fn list_todos(
    AuthUser(auth, _): AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    let handler = ListTodosHandler::new(state.todo_repo.clone());
    let todos = handler.handle(ListTodos { user_id: auth.id }).await?;
    Ok(Json(todos.into_iter().map(TodoResponse::from).collect()))
}

async fn get_todo(
    AuthUser(auth, _): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TodoResponse>, AppError> {
    let id = TodoId::from_str(&id)?;
    let handler = GetTodoHandler::new(state.todo_repo.clone());
    let todo = handler
        .handle(GetTodo {
            user_id: auth.id,
            id,
        })
        .await?;
    Ok(Json(TodoResponse::from(todo)))
}

async fn update_todo(
    AuthUser(auth, _): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTodoRequest>,
) -> Result<StatusCode, AppError> {
    req.validate()?;
    let id = TodoId::from_str(&id)?;
    let title = match req.title {
        Some(raw) => Some(Title::new(raw)?),
        None => None,
    };
    let handler = UpdateTodoHandler::new(state.todo_repo.clone());
    handler
        .handle(UpdateTodo {
            user_id: auth.id,
            id,
            title,
            status: req.status,
        })
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
        .handle(DeleteTodo {
            user_id: auth.id,
            id,
        })
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
