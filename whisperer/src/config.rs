use std::fs::File;
use std::io::Read;

use once_cell::sync::OnceCell;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub flag: String,
    pub key: String,
    pub derive_key: String,
    pub dict: Vec<char>,
    pub key_words: Vec<[String; 2]>,
    pub zstd_level: i32
}

static G_CONF: OnceCell<Conf> = OnceCell::new();

impl Conf {
    pub fn global() -> &'static Conf {
        G_CONF.get().expect("配置未初始化")
    }

    pub fn init_conf() {
        let mut str_val = String::new();
        let config_str = match File::open("config.toml") {
            Ok(mut file) => {
                file.read_to_string(&mut str_val).expect("请确认文件编码格式");
                str_val.as_str()
            }
            _ => {
                include_str!("config.toml")
            }
        };
        let mut conf: Conf = toml::from_str(config_str).expect("解析失败, 请查看你的配置文件结构");
        conf.dict.sort_unstable();
        G_CONF.set(conf).expect("设置全局配置失败");
    }
}

pub fn pure_config(path: &str) -> Conf {
    let mut file = File::open(path).expect("请确认配置文件是否存在");
    let mut str_val = String::new();
    file.read_to_string(&mut str_val).expect("请确认文件编码格式");
    toml::from_str(&str_val).expect("解析失败, 请查看你的配置文件结构")
}