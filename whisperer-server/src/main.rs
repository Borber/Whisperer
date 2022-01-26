use poem::{handler, listener::TcpListener, post, Route, Server, web::Json};
use tracing::{debug, info};

use whisperer::{decode, encode};
use whisperer::config::Conf;
use whisperer_server_lib::*;

use crate::config::ServerConf;

mod config;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    Conf::init_conf();
    let server_config = ServerConf::init_config("server.toml");
    let addr = server_config.get_address();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();
    let app = Route::new()
        .at(server_config.path.e, post(encode_api))
        .at( server_config.path.d, post(decode_api));
    info!("listening on {}", addr);
    Server::new(TcpListener::bind(addr))
        .run(app)
        .await
}


#[handler]
fn encode_api(req: Json<RequestBody>) -> Json<serde_json::Value> {
    match req.s.len() {
        l if l > 0 => {
            debug!("加密->{}", req.s);
            let re = format!("{}{}", Conf::global().flag, encode(req.s.to_string()));
            debug!("结果<-{}", re);
            success_s(re)
        }
        _ => {
            fail("请输入文字")
        }
    }
}

#[handler]
fn decode_api(req: Json<RequestBody>) -> Json<serde_json::Value> {
    match req.s.len() {
        l if l > 0 => {
            debug!("解密->{}", req.s);
            if req.s.starts_with(&Conf::global().flag) {
                let re = decode(req.s.replace(&Conf::global().flag, ""));
                debug!("结果->{}", re);
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