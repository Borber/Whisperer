use clap::Parser;

use whisperer::{decode, encode};
use whisperer::config::{Conf, pure_config};

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
    Conf::init_conf("config.toml");
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
            let dict_tmp = pure_config("config.toml").dict;
            if dict_tmp.len() != 256 {
                println!("长度错误, 应长256");
                pass = false;
            }
            for i in 0..dict_tmp.len() {
                for j in i + 1..dict_tmp.len() {
                    if dict_tmp[i] == dict_tmp[j] && i != j {
                        println!("第{},{}个相同, 值为{}", i, j, dict_tmp[i]);
                        pass = false;
                    }
                }
            }
            if pass {
                println!("验证通过!");
            }
        }
        false => {}
    }
}
