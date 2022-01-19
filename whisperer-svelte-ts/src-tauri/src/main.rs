#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use serde::Deserialize;
use tauri::command;

use whisperer::{config::Conf, decode, encode};

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    s: String,
}

#[command]
fn encode_api(endpoint: String, body: RequestBody) -> String {
    println!("{} {:?}", endpoint, body);
    format!("{}{}", Conf::global().flag, encode(body.s))
}

#[command]
fn decode_api(endpoint: String, body: RequestBody) -> String {
    println!("{} {:?}", endpoint, body);
    if body.s.starts_with(&Conf::global().flag) {
        decode(body.s.replace(&Conf::global().flag, ""))
    } else {
        format!("请检查你的输入格式, 形如 -> {}XXX", &Conf::global().flag)
    }
}

fn main() {
    Conf::init_conf("config.toml");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            encode_api,
            decode_api
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
