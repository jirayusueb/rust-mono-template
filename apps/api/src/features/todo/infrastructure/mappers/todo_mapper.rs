use crate::features::todo::domain::Todo;
use crate::features::todo::domain::{Status, Title};
use crate::features::todo::infrastructure::error::TodoInfraError;
use crate::features::todo::infrastructure::schema::todo::Model;

/// Maps a database row into a domain `Todo`.
pub fn to_domain(row: Model) -> Result<Todo, TodoInfraError> {
    let status = match row.status.as_str() {
        "pending" => Status::Pending,
        "completed" => Status::Completed,
        _ => {
            return Err(TodoInfraError::Decode(format!(
                "invalid status: {}",
                row.status
            )))
        }
    };
    Ok(Todo::restore(
        row.id.into(),
        row.user_id.into(),
        Title::restore(row.title),
        status,
        row.created_at,
        row.updated_at,
        row.deleted_at,
    ))
}
