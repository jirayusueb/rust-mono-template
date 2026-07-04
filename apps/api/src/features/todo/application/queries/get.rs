use std::sync::Arc;

use crate::features::todo::application::dtos::{GetTodoInput, GetTodoOutput};
use crate::features::todo::application::ports::todo_repository::TodoRepository;
use crate::shared::kernel::error::AppError;

pub struct GetTodoHandler {
    repo: Arc<dyn TodoRepository>,
}

impl GetTodoHandler {
    pub fn new(repo: Arc<dyn TodoRepository>) -> Self {
        Self { repo }
    }

    pub async fn handle(&self, cmd: GetTodoInput) -> Result<GetTodoOutput, AppError> {
        let todo = self
            .repo
            .find_by_id(&cmd.id, &cmd.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("todo not found".into()))?;
        Ok(todo.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::todo::application::ports::todo_repository::MockTodoRepository;
    use crate::features::todo::domain::{Title, Todo, TodoId};
    use crate::shared::kernel::UserId;

    fn make_todo() -> Todo {
        Todo::create(UserId::new(), Title::new("test".into()).unwrap()).unwrap()
    }

    #[tokio::test]
    async fn returns_todo_when_found() {
        let mut mock = MockTodoRepository::new();
        mock.expect_find_by_id()
            .returning(|_, _| Ok(Some(make_todo())));

        let handler = GetTodoHandler::new(Arc::new(mock));
        let cmd = GetTodoInput {
            user_id: UserId::new(),
            id: TodoId::new(),
        };

        assert!(handler.handle(cmd).await.is_ok());
    }

    #[tokio::test]
    async fn not_found_when_missing() {
        let mut mock = MockTodoRepository::new();
        mock.expect_find_by_id().returning(|_, _| Ok(None));

        let handler = GetTodoHandler::new(Arc::new(mock));
        let cmd = GetTodoInput {
            user_id: UserId::new(),
            id: TodoId::new(),
        };

        assert!(matches!(
            handler.handle(cmd).await.unwrap_err(),
            AppError::NotFound(_)
        ));
    }
}
