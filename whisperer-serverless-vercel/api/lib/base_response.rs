use http::StatusCode;
use serde::{Deserialize, Serialize};
use vercel_lambda::{error::VercelError, IntoResponse, lambda, Request, Response};

#[derive(Serialize, Debug)]
pub struct ResponseBody {
    code: usize,
    result: String,
}

impl ResponseBody {
    pub fn new(code: usize, result: String) -> Self {
        ResponseBody {
            code,
            result,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    pub s: String,
}

pub fn re(body: ResponseBody) -> Response<String> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string::<ResponseBody>(&body).unwrap())
        .expect("Internal Server Error");
    response
}