mod crypt;
mod dict;
mod hash;

use crate::crypt::{decrypt, encrypt};

const KEY: &[u8; 32] =  b"XDXDtudou@KeyFansClub^_^Encode!!";

fn main() {
    let s = "BORBER";
    let re = encode(s);
    println!("低语：{}",re);
    let er = decode(&re);
    println!("低语：{}",er);
}

pub fn decode(s: &str) -> String {
    let reduction:Vec<u8> = s.chars().into_iter().map(|c| dict::DICT.binary_search(&c).expect("解密失败, 请检查你的输入 / decode failure check the input") as u8).collect();
    let clear:Vec<u8> = reduction.iter().enumerate().map(|(index, e)| decrypt(index,e)).collect();
    let buf = zstd::stream::decode_all(clear.as_slice()).unwrap_or(clear);
    let mut  result =  String::from_utf8(buf).expect("解密失败, 请检查你的输入 / decode failure check the input");
    for key in dict::KEY_WORD {
        result = result.replace(key[1], key[0]);
    }
    result
}

pub fn encode(s: &str) -> String {
    let mut buf = String::from(s);
    for key in dict::KEY_WORD {
        buf = buf.replace(key[0], key[1]);
    }
    let compressed = zstd::stream::encode_all(buf.as_bytes(), 20).expect("压缩失败 / compress failure!"); //zstd最高级别压缩
    let short = if buf.as_bytes().len() > compressed.len() { compressed } else { buf.as_bytes().to_vec() } ;
    let cipher:Vec<u8> = short.iter().enumerate().map(|(index,d)| encrypt(index,d)).collect();
    cipher.iter().map(|c| dict::DICT[*c as usize]).into_iter().collect() //映射u8为中文字符
}