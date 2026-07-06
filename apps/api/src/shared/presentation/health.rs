use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize, ts_rs::TS, utoipa::ToSchema)]
#[ts(export, export_to = "../../web/src/lib/dto.ts")]
pub struct Health {
    pub status: String,
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    operation_id = "check",
    responses((status = 200, body = Health, description = "service is healthy")),
)]
pub async fn health() -> Json<Health> {
    Json(Health {
        status: "ok".into(),
    })
}
