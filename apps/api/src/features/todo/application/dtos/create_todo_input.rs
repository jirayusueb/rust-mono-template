use crate::features::todo::domain::Title;
use crate::shared::kernel::UserId;

pub struct CreateTodoInput {
    pub user_id: UserId,
    pub title: Title,
}
