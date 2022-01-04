use clap::Parser;

use whisperer::{decode, encode};
use whisperer::config::Conf;

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
