use std::fs::File;
use std::io::Read;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct ServerConf {
    pub host: String,
    pub port: usize
}

impl ServerConf {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn init_config(path: &str) -> Self {
        let mut file = File::open(path).expect("请确认配置文件是否存在");
        let mut str_val = String::new();
        file.read_to_string(&mut str_val).expect("请确认文件编码格式");
        toml::from_str(&str_val).expect("解析失败, 请查看你的配置文件结构")
    }
}