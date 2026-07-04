use crate::features::todo::domain::{Status, Title, TodoId};
use crate::shared::kernel::UserId;

// --- Command DTOs ---

pub struct CreateTodo {
    pub user_id: UserId,
    pub title: Title,
}

pub struct UpdateTodo {
    pub user_id: UserId,
    pub id: TodoId,
    pub title: Option<Title>,
    pub status: Option<Status>,
}

pub struct DeleteTodo {
    pub user_id: UserId,
    pub id: TodoId,
}

// --- Query DTOs ---

pub struct GetTodo {
    pub user_id: UserId,
    pub id: TodoId,
}

pub struct ListTodos {
    pub user_id: UserId,
}
