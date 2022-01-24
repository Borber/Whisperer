use poem::web::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ResponseBody {
    code: usize,
    result: String,
}

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    pub s: String,
}

pub fn fail(msg: &str) -> Json<serde_json::Value> {
    Json(serde_json::json!(
        ResponseBody{
            code: 1,
            result: msg.to_string()
        }))
}

pub fn success_s(msg: String) -> Json<serde_json::Value> {
    Json(serde_json::json!(
        ResponseBody{
            code: 0,
            result: msg
        }))
}