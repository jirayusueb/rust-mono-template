use std::sync::Arc;

use crate::features::todo::application::dtos::{CreateTodoInput, CreateTodoOutput};
use crate::features::todo::application::ports::todo_repository::TodoRepository;
use crate::shared::kernel::error::AppError;

pub struct CreateTodoHandler {
    repo: Arc<dyn TodoRepository>,
}

impl CreateTodoHandler {
    pub fn new(repo: Arc<dyn TodoRepository>) -> Self {
        Self { repo }
    }

    pub async fn handle(&self, cmd: CreateTodoInput) -> Result<CreateTodoOutput, AppError> {
        let todo = crate::features::todo::domain::Todo::create(cmd.user_id, cmd.title)?;
        let id = todo.id;
        self.repo.save(&todo).await?;
        Ok(CreateTodoOutput {
            id: id.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::todo::application::ports::todo_repository::MockTodoRepository;
    use crate::features::todo::domain::Title;
    use crate::shared::kernel::UserId;

    fn make_input() -> CreateTodoInput {
        CreateTodoInput {
            user_id: UserId::new(),
            title: Title::new("buy milk".into()).unwrap(),
        }
    }

    #[tokio::test]
    async fn creates_todo_returns_id_when_save_succeeds() {
        let mut mock = MockTodoRepository::new();
        mock.expect_save().returning(|_| Ok(()));

        let handler = CreateTodoHandler::new(Arc::new(mock));
        let input = make_input();

        let result = handler.handle(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn save_failure_propagates_internal_error() {
        let mut mock = MockTodoRepository::new();
        mock.expect_save()
            .returning(|_| Err(AppError::Internal("db down".into())));

        let handler = CreateTodoHandler::new(Arc::new(mock));
        let result = handler.handle(make_input()).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Internal(_)));
    }
}
