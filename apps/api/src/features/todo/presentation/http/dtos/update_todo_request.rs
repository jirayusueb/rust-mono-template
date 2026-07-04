use serde::Deserialize;
use validator::Validate;

use crate::features::todo::domain::Status;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodoRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    // status is an enum — serde rejects bad values before validator runs
    pub status: Option<Status>,
}
