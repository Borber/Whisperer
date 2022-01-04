use clap::Parser;

use crate::config::Conf;
use crate::crypt::{decrypt, encrypt};

mod crypt;
mod hash;
mod config;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// 待加密文本
    value: Option<String>,

    /// 待解密文本
    #[clap(short, long, value_name = "VALUE")]
    decrypt: Option<String>,

    /// 字典校验及排序
    #[clap(long)]
    check_dict: bool,
}

fn main() {
    Conf::init_conf();
    let cli: Cli = Cli::parse();
    match cli.value {
        Some(encrypt) => {
            println!("{}{}", Conf::global().flag, encode(encrypt.to_string()));
        },
        None => {}
    }
    match cli.decrypt {
        Some(decrypt) => {
            if decrypt.starts_with(&Conf::global().flag) {
                println!("{}", decode(decrypt.replace(&Conf::global().flag, "")));
            } else {
                println!("请检查你的输入格式, 形如 -> {}XXX", &Conf::global().flag)
            }
        },
        None => {}
    }
    match cli.check_dict {
        true => {
            let mut pass = true;
            let mut dict_tmp = Conf::global().dict.clone();
            if dict_tmp.len() != 256 {
                println!("长度错误, 应长256");
                pass = false;
            }
            dict_tmp.sort();
            if Conf::global().dict != dict_tmp {
                println!("顺序错误, 应为:");
                println!("{:?}", dict_tmp);
                pass = false;
            }
            for i in 0..dict_tmp.len() - 1 {
                if dict_tmp[i] == dict_tmp[i + 1] {
                    println!("第{},{}相同, 值为:{}", i, i + 1, dict_tmp[i]);
                    pass = false;
                }
            }
            if pass {
                println!("验证通过!");
            }
        }
        false => {}
    }
}

pub fn decode(s: String) -> String {
    let reduction: Vec<u8> = s.chars().into_iter().map(|c| Conf::global().dict.binary_search(&c).expect("解密失败, 请检查你的输入") as u8).collect();
    let clear: Vec<u8> = reduction.iter().enumerate().map(|(index, e)| decrypt(index, reduction.len(), e)).collect();
    let buf = zstd::stream::decode_all(clear.as_slice()).unwrap_or(clear);
    let mut result = String::from_utf8(buf).expect("解密失败, 请检查你的输入");
    for key in &Conf::global().key_words {
        result = result.replace(key[1].as_str(), key[0].as_str());
    }
    result
}

pub fn encode(s: String) -> String {
    let mut buf = String::from(s);
    for key in &Conf::global().key_words {
        buf = buf.replace(key[0].as_str(), key[1].as_str());
    }
    let compressed = zstd::stream::encode_all(buf.as_bytes(), 20).expect("压缩失败"); //zstd最高级别压缩
    let short = if buf.as_bytes().len() > compressed.len() { compressed } else { buf.as_bytes().to_vec() };
    let cipher: Vec<u8> = short.iter().enumerate().map(|(index, d)| encrypt(index, short.len(), d)).collect();
    cipher.iter().map(|c| Conf::global().dict[*c as usize]).into_iter().collect() //映射u8为中文字符
}