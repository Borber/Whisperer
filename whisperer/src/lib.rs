use rand::Rng;

use crate::config::Conf;
use crate::crypt::{decrypt, encrypt};

mod crypt;
mod hash;
pub mod config;

pub fn decode(s: String) -> String {
    let reduction: Vec<u8> = s.trim().chars().into_iter().map(|c| Conf::global().dict.binary_search(&c).expect("解密失败, 请检查你的输入") as u8).collect();
    let mut clear: Vec<u8> = Vec::new();
    for (index, e) in reduction.iter().enumerate() {
        if index == 0 { continue; }
        clear.push(decrypt(index, &reduction[index - 1], reduction.len(), e));
    }
    let buf = zstd::stream::decode_all(clear.as_slice()).unwrap_or(clear);
    let mut result = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(..) => return "解密失败, 请检查你的输入".to_string()
    };
    for key in &Conf::global().key_words {
        result = result.replace(key[1].as_str(), key[0].as_str());
    }
    result
}

pub fn encode(s: String) -> String {
    let mut buf = s.trim().to_string();
    for key in &Conf::global().key_words {
        buf = buf.replace(key[0].as_str(), key[1].as_str());
    }
    let compressed = zstd::stream::encode_all(buf.as_bytes(), Conf::global().zstd_level).expect("压缩失败"); //zstd最高级别压缩
    let mut short = if buf.as_bytes().len() > compressed.len() { compressed } else { buf.as_bytes().to_vec() };
    let mut rng = rand::thread_rng();
    let random: u8 = rng.gen();
    short.insert(0, random);
    let mut cipher: Vec<u8> = vec![random];
    for (index, d) in short.iter().enumerate() {
        if index == 0 { continue; }
        cipher.push(encrypt(index, cipher.last().unwrap(), short.len(), d));
    }
    cipher.iter().map(|c| Conf::global().dict[*c as usize]).into_iter().collect() //映射u8为中文字符
}