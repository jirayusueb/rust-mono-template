use crate::features::todo::domain::TodoId;
use crate::shared::kernel::UserId;

pub struct DeleteTodoInput {
    pub user_id: UserId,
    pub id: TodoId,
}
