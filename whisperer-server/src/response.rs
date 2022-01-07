use poem::web::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonBody {
    pub s: String,
}

pub fn fail(msg: &str) -> Json<serde_json::Value> {
    Json(serde_json::json!(
                JsonBody{
                    s: msg.to_string()
                }))
}

pub fn success_s(msg: String) -> Json<serde_json::Value> {
    Json(serde_json::json!(
                JsonBody{
                    s: msg
                }))
}