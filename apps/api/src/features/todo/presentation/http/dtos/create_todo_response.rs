use serde::Serialize;

#[derive(Debug, Serialize, ts_rs::TS, utoipa::ToSchema)]
#[ts(export, export_to = "../../web/src/lib/dto.ts")]
pub struct CreateTodoResponse {
    pub id: String,
}
