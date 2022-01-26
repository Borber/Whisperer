use std::borrow::Borrow;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Api {
    pub uri: String,
    pub database: String,
    pub collection: String,
    pub auth_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataBaseConfig {
    pub uri: String,
    pub database: String,
    pub collection: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddVO {
    pub database: DataBaseConfig,
    pub auth_code: String,
    pub add: isize,
}

impl Api {
    pub fn default() -> Self {
        let s = env!("API_CONFIG");
        serde_json::from_str(s).expect("解析失败, 请查看你的配置文件结构")
    }
    pub async fn add_one(&self) {
        reqwest::Client::new().post("https://vercel-mongodb-count.vercel.app/api/v1/add")
            .json(AddVO {
                database: DataBaseConfig {
                    uri: self.uri.clone(),
                    database: self.database.clone(),
                    collection: self.collection.clone(),
                },
                auth_code: self.auth_code.clone(),
                add: 1,
            }.borrow())
            .send()
            .await;
    }
}
