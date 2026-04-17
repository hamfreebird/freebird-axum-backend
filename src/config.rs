use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); // 加载 .env 文件，失败也不 panic
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        Self { database_url }
    }
}