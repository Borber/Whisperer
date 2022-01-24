use poem::{handler, web::Json};
use whisperer::{decode, encode};
use whisperer::config::Conf;

use poem_vercel_lib::{
    Error,
    response::*,
};

#[handler]
fn index(req: Json<RequestBody>) -> Json<serde_json::Value> {
    match req.s.len() {
        l if l > 0 => {
            let re = format!("{}{}", Conf::global().flag, encode(req.s.to_string()));
            success_s(re)
        }
        _ => {
            fail("请输入文字")
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    Conf::init_conf();
    poem_vercel_lib::run(index).await
}