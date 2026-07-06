use crate::shared::kernel::UserId;

pub struct ListTodosInput {
    pub user_id: UserId,
    pub query: Option<String>,
}
