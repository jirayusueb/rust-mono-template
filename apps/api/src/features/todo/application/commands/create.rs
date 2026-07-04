use std::sync::Arc;

use crate::features::todo::application::dtos::CreateTodoCommand;
use crate::features::todo::application::ports::todo_repository::TodoRepository;
use crate::features::todo::domain::TodoId;
use crate::shared::kernel::error::AppError;

pub struct CreateTodoHandler {
    repo: Arc<dyn TodoRepository>,
}

impl CreateTodoHandler {
    pub fn new(repo: Arc<dyn TodoRepository>) -> Self {
        Self { repo }
    }

    pub async fn handle(&self, cmd: CreateTodoCommand) -> Result<TodoId, AppError> {
        let todo = crate::features::todo::domain::Todo::new(cmd.user_id, cmd.title)?;
        let id = todo.id;
        self.repo.save(&todo).await?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::todo::application::ports::todo_repository::MockTodoRepository;
    use crate::features::todo::domain::Title;
    use crate::shared::kernel::UserId;

    fn make_cmd() -> CreateTodoCommand {
        CreateTodoCommand {
            user_id: UserId::new(),
            title: Title::new("buy milk".into()).unwrap(),
        }
    }

    #[tokio::test]
    async fn creates_todo_returns_id_when_save_succeeds() {
        let mut mock = MockTodoRepository::new();
        mock.expect_save().returning(|_| Ok(()));

        let handler = CreateTodoHandler::new(Arc::new(mock));
        let cmd = make_cmd();

        let result = handler.handle(cmd).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn save_failure_propagates_internal_error() {
        let mut mock = MockTodoRepository::new();
        mock.expect_save()
            .returning(|_| Err(AppError::Internal("db down".into())));

        let handler = CreateTodoHandler::new(Arc::new(mock));
        let result = handler.handle(make_cmd()).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Internal(_)));
    }
}
