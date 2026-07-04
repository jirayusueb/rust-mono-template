use std::sync::Arc;

use crate::features::todo::application::dtos::ListTodos;
use crate::features::todo::application::ports::todo_repository::TodoRepository;
use crate::features::todo::domain::Todo;
use crate::shared::kernel::error::AppError;

pub struct ListTodosHandler {
    repo: Arc<dyn TodoRepository>,
}

impl ListTodosHandler {
    pub fn new(repo: Arc<dyn TodoRepository>) -> Self {
        Self { repo }
    }

    pub async fn handle(&self, cmd: ListTodos) -> Result<Vec<Todo>, AppError> {
        self.repo.find_all(&cmd.user_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::todo::application::ports::todo_repository::MockTodoRepository;
    use crate::features::todo::domain::Title;
    use crate::shared::kernel::UserId;

    fn make_todo() -> Todo {
        Todo::new(UserId::new(), Title::new("test".into()).unwrap()).unwrap()
    }

    #[tokio::test]
    async fn returns_todos_for_user() {
        let mut mock = MockTodoRepository::new();
        mock.expect_find_all()
            .returning(|_| Ok(vec![make_todo(), make_todo()]));

        let handler = ListTodosHandler::new(Arc::new(mock));
        let cmd = ListTodos {
            user_id: UserId::new(),
        };

        let result = handler.handle(cmd).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn returns_empty_list() {
        let mut mock = MockTodoRepository::new();
        mock.expect_find_all().returning(|_| Ok(vec![]));

        let handler = ListTodosHandler::new(Arc::new(mock));
        let cmd = ListTodos {
            user_id: UserId::new(),
        };

        assert!(handler.handle(cmd).await.unwrap().is_empty());
    }
}
