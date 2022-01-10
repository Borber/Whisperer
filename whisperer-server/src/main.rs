use poem::{handler, listener::TcpListener, post, Route, Server, web::Json};
use tracing::{debug, info};

use whisperer::{decode, encode};
use whisperer::config::Conf;

use crate::response::JsonBody;

mod response;

const ADDR: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();
    Conf::init_conf("config.toml");

    let app = Route::new()
        .at("/v1/api/e", post(encode_api))
        .at("/v1/api/d", post(decode_api));
    info!("listening on {}", ADDR);
    Server::new(TcpListener::bind(ADDR))
        .run(app)
        .await
}


#[handler]
fn encode_api(req: Json<JsonBody>) -> Json<serde_json::Value> {
    match req.s.len() {
        l if l > 0 => {
            debug!("加密->{}", req.s);
            let re = format!("{}{}", Conf::global().flag, encode(req.s.to_string()));
            debug!("结果<-{}", re);
            response::success_s(re)
        }
        _ => {
            response::fail("请输入文字")
        }
    }
}

#[handler]
fn decode_api(req: Json<JsonBody>) -> Json<serde_json::Value> {
    match req.s.len() {
        l if l > 0 => {
            debug!("解密->{}", req.s);
            if req.s.starts_with(&Conf::global().flag) {
                let re = format!("{}", decode(req.s.replace(&Conf::global().flag, "")));
                debug!("结果->{}", re);
                response::success_s(re)
            } else {
                response::fail(format!("请检查你的输入格式, 形如 -> {}XXX", &Conf::global().flag).as_str())
            }
        }
        _ => {
            response::fail("请输入文字")
        }
    }
}