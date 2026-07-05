use serde::Serialize;

#[derive(Debug, Serialize, ts_rs::TS)]
#[ts(export, export_to = "../../web/src/lib/contract.ts")]
pub struct CreateTodoResponse {
    pub id: String,
}
