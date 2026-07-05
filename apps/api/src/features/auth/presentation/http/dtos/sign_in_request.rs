use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ts_rs::TS)]
#[ts(export, export_to = "../../web/src/lib/contract.ts")]
pub struct SignInRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}
