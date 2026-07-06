use serde::Deserialize;
use validator::Validate;

use crate::features::todo::domain::Status;

#[derive(Debug, Deserialize, Validate, ts_rs::TS, utoipa::ToSchema)]
#[ts(export, export_to = "../../web/src/lib/dto.ts")]
pub struct UpdateTodoRequest {
    #[validate(length(min = 1, max = 200))]
    #[ts(optional)]
    pub title: Option<String>,
    // status is an enum — serde rejects bad values before validator runs
    #[ts(optional)]
    pub status: Option<Status>,
}
