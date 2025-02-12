use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, FromRow, Clone)]  
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub external_id: String,
    pub description: String,
}
