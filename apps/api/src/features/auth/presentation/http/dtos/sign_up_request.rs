use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ts_rs::TS)]
#[ts(export, export_to = "../../web/src/lib/contract.ts")]
pub struct SignUpRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(max = 100))]
    #[ts(optional)]
    pub name: Option<String>,
}
