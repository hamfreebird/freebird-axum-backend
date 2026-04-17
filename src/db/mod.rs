pub mod models;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use crate::config::Config;

pub type Pool = PgPool;

pub async fn create_pool(config: &Config) -> Result<Pool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
}