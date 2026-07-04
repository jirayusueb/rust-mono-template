use std::sync::Arc;

use crate::features::todo::application::dtos::DeleteTodo;
use crate::features::todo::application::ports::todo_repository::TodoRepository;
use crate::shared::kernel::error::AppError;

pub struct DeleteTodoHandler {
    repo: Arc<dyn TodoRepository>,
}

impl DeleteTodoHandler {
    pub fn new(repo: Arc<dyn TodoRepository>) -> Self {
        Self { repo }
    }

    pub async fn handle(&self, cmd: DeleteTodo) -> Result<(), AppError> {
        self.repo
            .find_by_id(&cmd.id, &cmd.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("todo not found".into()))?;

        self.repo.delete(&cmd.id, &cmd.user_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::todo::application::ports::todo_repository::MockTodoRepository;
    use crate::features::todo::domain::{Title, Todo, TodoId};
    use crate::shared::kernel::UserId;

    fn make_todo() -> Todo {
        Todo::new(UserId::new(), Title::new("test".into()).unwrap()).unwrap()
    }

    #[tokio::test]
    async fn deletes_existing_todo() {
        let todo = make_todo();
        let cmd = DeleteTodo {
            user_id: todo.user_id,
            id: todo.id,
        };

        let mut mock = MockTodoRepository::new();
        mock.expect_find_by_id()
            .returning(|_, _| Ok(Some(make_todo())));
        mock.expect_delete().returning(|_, _| Ok(()));

        let handler = DeleteTodoHandler::new(Arc::new(mock));
        assert!(handler.handle(cmd).await.is_ok());
    }

    #[tokio::test]
    async fn not_found_when_todo_missing() {
        let mut mock = MockTodoRepository::new();
        mock.expect_find_by_id().returning(|_, _| Ok(None));

        let handler = DeleteTodoHandler::new(Arc::new(mock));
        let cmd = DeleteTodo {
            user_id: UserId::new(),
            id: TodoId::new(),
        };

        assert!(matches!(
            handler.handle(cmd).await.unwrap_err(),
            AppError::NotFound(_)
        ));
    }
}
