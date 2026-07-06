use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ts_rs::TS, utoipa::ToSchema)]
#[ts(export, export_to = "../../web/src/lib/dto.ts")]
pub struct SignUpRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(max = 100))]
    #[ts(optional)]
    pub name: Option<String>,
}
