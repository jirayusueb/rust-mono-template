use std::sync::Arc;

use crate::features::todo::application::dtos::{
    GetTodoOutput, ListTodosInput, ListTodosOutput,
};
use crate::features::todo::application::ports::todo_repository::TodoRepository;
use crate::shared::kernel::error::AppError;

pub struct ListTodosHandler {
    repo: Arc<dyn TodoRepository>,
}

impl ListTodosHandler {
    pub fn new(repo: Arc<dyn TodoRepository>) -> Self {
        Self { repo }
    }

    pub async fn handle(&self, cmd: ListTodosInput) -> Result<ListTodosOutput, AppError> {
        let todos = match &cmd.query {
            Some(q) if !q.is_empty() => self.repo.search(&cmd.user_id, q, 100).await?,
            _ => self.repo.find_all(&cmd.user_id).await?,
        };
        Ok(ListTodosOutput {
            todos: todos.into_iter().map(GetTodoOutput::from).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::todo::application::ports::todo_repository::MockTodoRepository;
    use crate::features::todo::domain::{Title, Todo};
    use crate::shared::kernel::UserId;

    fn make_todo() -> Todo {
        Todo::create(UserId::new(), Title::new("test".into()).unwrap()).unwrap()
    }

    #[tokio::test]
    async fn returns_todos_for_user() {
        let mut mock = MockTodoRepository::new();
        mock.expect_find_all()
            .returning(|_| Ok(vec![make_todo(), make_todo()]));

        let handler = ListTodosHandler::new(Arc::new(mock));
        let cmd = ListTodosInput {
            user_id: UserId::new(),
            query: None,
        };

        let result = handler.handle(cmd).await.unwrap();
        assert_eq!(result.todos.len(), 2);
    }

    #[tokio::test]
    async fn returns_empty_list() {
        let mut mock = MockTodoRepository::new();
        mock.expect_find_all().returning(|_| Ok(vec![]));

        let handler = ListTodosHandler::new(Arc::new(mock));
        let cmd = ListTodosInput {
            user_id: UserId::new(),
            query: None,
        };

        assert!(handler.handle(cmd).await.unwrap().todos.is_empty());
    }
}
