use crate::features::todo::domain::Title;
use crate::shared::kernel::UserId;

pub struct CreateTodoCommand {
    pub user_id: UserId,
    pub title: Title,
}
