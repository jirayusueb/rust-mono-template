use std::sync::Arc;

use crate::features::todo::application::dtos::UpdateTodoCommand;
use crate::features::todo::application::ports::todo_repository::TodoRepository;
use crate::features::todo::domain::Status;
use crate::shared::kernel::error::AppError;

pub struct UpdateTodoHandler {
    repo: Arc<dyn TodoRepository>,
}

impl UpdateTodoHandler {
    pub fn new(repo: Arc<dyn TodoRepository>) -> Self {
        Self { repo }
    }

    pub async fn handle(&self, cmd: UpdateTodoCommand) -> Result<(), AppError> {
        let mut todo = self
            .repo
            .find_by_id(&cmd.id, &cmd.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("todo not found".into()))?;

        if let Some(title) = cmd.title {
            todo.rename(title);
        }
        if let Some(status) = cmd.status {
            match status {
                Status::Completed => todo.complete()?,
                Status::Pending => todo.reopen()?,
            }
        }

        self.repo.save(&todo).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::todo::application::ports::todo_repository::MockTodoRepository;
    use crate::features::todo::domain::{Title, Todo};
    use crate::shared::kernel::UserId;
    use rstest::rstest;

    fn make_todo_with_status(status: Status) -> Todo {
        let mut todo = Todo::new(UserId::new(), Title::new("test".into()).unwrap()).unwrap();
        todo.status = status;
        todo
    }

    fn make_cmd(
        title: Option<&str>,
        status: Option<Status>,
        initial_status: Status,
    ) -> (UpdateTodoCommand, Todo) {
        let todo = make_todo_with_status(initial_status);
        let cmd = UpdateTodoCommand {
            user_id: todo.user_id,
            id: todo.id,
            title: title.map(|t| Title::new(t.into()).unwrap()),
            status,
        };
        (cmd, todo)
    }

    #[rstest]
    #[case::title_only(Some("new"), None, Status::Pending)]
    #[case::complete(None, Some(Status::Completed), Status::Pending)]
    #[case::reopen(None, Some(Status::Pending), Status::Completed)]
    #[case::both(Some("new"), Some(Status::Completed), Status::Pending)]
    #[case::neither(None, None, Status::Pending)]
    #[tokio::test]
    async fn update_combinations_succeed(
        #[case] title: Option<&str>,
        #[case] status: Option<Status>,
        #[case] initial_status: Status,
    ) {
        let (cmd, _todo) = make_cmd(title, status, initial_status);
        let todo_for_mock = make_todo_with_status(initial_status);

        let mut mock = MockTodoRepository::new();
        mock.expect_find_by_id()
            .returning(move |_, _| Ok(Some(todo_for_mock.clone())));
        mock.expect_save().returning(|_| Ok(()));

        let handler = UpdateTodoHandler::new(Arc::new(mock));
        assert!(handler.handle(cmd).await.is_ok());
    }

    #[tokio::test]
    async fn not_found_when_todo_missing() {
        let mut mock = MockTodoRepository::new();
        mock.expect_find_by_id().returning(|_, _| Ok(None));

        let handler = UpdateTodoHandler::new(Arc::new(mock));
        let (cmd, _) = make_cmd(None, None, Status::Pending);

        assert!(matches!(
            handler.handle(cmd).await.unwrap_err(),
            AppError::NotFound(_)
        ));
    }

    #[tokio::test]
    async fn complete_already_completed_conflict() {
        let mut mock = MockTodoRepository::new();
        mock.expect_find_by_id()
            .returning(|_, _| Ok(Some(make_todo_with_status(Status::Completed))));

        let handler = UpdateTodoHandler::new(Arc::new(mock));
        let (cmd, _) = make_cmd(None, Some(Status::Completed), Status::Completed);

        assert!(matches!(
            handler.handle(cmd).await.unwrap_err(),
            AppError::Conflict(_)
        ));
    }

    #[tokio::test]
    async fn reopen_pending_conflict() {
        let mut mock = MockTodoRepository::new();
        mock.expect_find_by_id()
            .returning(|_, _| Ok(Some(make_todo_with_status(Status::Pending))));

        let handler = UpdateTodoHandler::new(Arc::new(mock));
        let (cmd, _) = make_cmd(None, Some(Status::Pending), Status::Pending);

        assert!(matches!(
            handler.handle(cmd).await.unwrap_err(),
            AppError::Conflict(_)
        ));
    }
}
