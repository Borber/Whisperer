use poem::{handler, web::Json};
use tokio::time;
use whisperer::config::Conf;
use whisperer::decode;

use poem_vercel_lib::{
    api::Api,
    Error,
    response::*,
};

#[handler]
async fn index(req: Json<RequestBody>) -> Json<serde_json::Value> {
    let api = Api::default();
    time::timeout(time::Duration::from_millis(20), async {
        api.add_one().await;
    }).await;
    match req.s.len() {
        l if l > 0 => {
            if req.s.starts_with(&Conf::global().flag) {
                let re = decode(req.s.replace(&Conf::global().flag, ""));
                success_s(re)
            } else {
                fail(format!("请检查你的输入格式, 形如 -> {}XXX", &Conf::global().flag).as_str())
            }
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