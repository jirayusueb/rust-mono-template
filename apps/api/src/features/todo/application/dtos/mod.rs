use crate::features::todo::domain::{Status, Title, TodoId};
use crate::shared::kernel::UserId;

// --- Command DTOs ---

pub struct CreateTodoCommand {
    pub user_id: UserId,
    pub title: Title,
}

pub struct UpdateTodoCommand {
    pub user_id: UserId,
    pub id: TodoId,
    pub title: Option<Title>,
    pub status: Option<Status>,
}

pub struct DeleteTodoCommand {
    pub user_id: UserId,
    pub id: TodoId,
}

// --- Query DTOs ---

pub struct GetTodoQuery {
    pub user_id: UserId,
    pub id: TodoId,
}

pub struct ListTodosQuery {
    pub user_id: UserId,
}
