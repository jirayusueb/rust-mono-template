use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ts_rs::TS, utoipa::ToSchema)]
#[ts(export, export_to = "../../web/src/lib/dto.ts")]
pub struct CreateTodoRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
}
