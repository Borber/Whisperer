use std::error::Error;
use std::fmt::{format, Pointer};

use http::StatusCode;
use vercel_lambda::{error::VercelError, IntoResponse, lambda, Request, Response};
use whisperer::{decode, encode};
use whisperer::config::Conf;

use base_response::{re, RequestBody, ResponseBody};

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    let (parts, body) = request.into_parts();
    let response = match serde_json::from_slice::<RequestBody>(&body) {
        Ok(body) if body.s.len() > 0 => {
            re(ResponseBody::new(0, decode(body.s.replace(&Conf::global().flag, ""))))
        }
        _ => {
            re(ResponseBody::new(1, "请检查你的输入".to_string()))
        }
    };
    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Conf::init_conf();
    Ok(lambda!(handler))
}