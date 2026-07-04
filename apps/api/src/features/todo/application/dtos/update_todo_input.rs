use crate::features::todo::domain::{Status, Title, TodoId};
use crate::shared::kernel::UserId;

pub struct UpdateTodoInput {
    pub user_id: UserId,
    pub id: TodoId,
    pub title: Option<Title>,
    pub status: Option<Status>,
}
