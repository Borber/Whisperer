use poem::{handler, web::Json};
use tokio::time;
use whisperer::config::Conf;
use whisperer::encode;

use poem_vercel_lib::{
    api::Api,
    Error,
    response::*,
};

#[handler]
async fn index(req: Json<RequestBody>) -> Json<serde_json::Value> {
    let api = Api::default();
    time::timeout(time::Duration::from_millis(50), async {
        api.add_one().await;
    }).await;
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