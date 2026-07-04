use crate::features::todo::domain::TodoId;
use crate::shared::kernel::UserId;

pub struct DeleteTodoCommand {
    pub user_id: UserId,
    pub id: TodoId,
}
