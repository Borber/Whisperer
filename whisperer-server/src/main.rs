use std::collections::HashMap;
use std::net::SocketAddr;

use axum::extract::Path;
use axum::Router;
use axum::routing::get;

use whisperer::{decode, encode};
use whisperer::config::Conf;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    Conf::init_conf();
    let app = Router::new()
        .route("/v1/api/e/:e", get(encode_api))
        .route("/v1/api/d/:d", get(decode_api));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn encode_api(Path(params): Path<HashMap<String, String>>) -> String {
    match params.get("e") {
        Some(e) => {
            format!("{}{}", Conf::global().flag, encode(e.to_string()))
        }
        None => { String::from("请输入文字") }
    }
}

async fn decode_api(Path(params): Path<HashMap<String, String>>) -> String {
    match params.get("d") {
        Some(d) => {
            if d.starts_with(&Conf::global().flag) {
                format!("{}", decode(d.replace(&Conf::global().flag, "")))
            } else {
                format!("请检查你的输入格式, 形如 -> {}XXX", &Conf::global().flag)
            }
        }
        None => { String::from("请输入文字") }
    }
}