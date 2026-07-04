use super::get_todo_output::GetTodoOutput;

/// Output of list-todos: a collection of flat todo views.
pub struct ListTodosOutput {
    pub todos: Vec<GetTodoOutput>,
}
