use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateTodoResponse {
    pub id: String,
}
