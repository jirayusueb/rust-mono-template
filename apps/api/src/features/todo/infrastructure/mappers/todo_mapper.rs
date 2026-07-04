use crate::features::todo::domain::Todo;
use crate::features::todo::domain::{Status, Title};
use crate::features::todo::infrastructure::error::TodoInfraError;
use crate::features::todo::infrastructure::schema::todo::Model;

/// Maps a database row into a domain `Todo`.
pub fn to_domain(row: Model) -> Result<Todo, TodoInfraError> {
    Ok(Todo {
        id: row.id.into(),
        user_id: row.user_id.into(),
        title: Title::new(row.title).map_err(|_| TodoInfraError::Decode("invalid title".into()))?,
        status: match row.status.as_str() {
            "pending" => Status::Pending,
            "completed" => Status::Completed,
            _ => {
                return Err(TodoInfraError::Decode(format!(
                    "invalid status: {}",
                    row.status
                )))
            }
        },
        created_at: row.created_at,
        updated_at: row.updated_at,
        deleted_at: row.deleted_at,
    })
}
