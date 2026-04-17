mod config;
mod db;
mod error;
mod handlers;
mod routes;

use crate::config::Config;
use crate::db::create_pool;
use crate::routes::create_router;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // --- 初始化日志 (新增) ---
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "blog_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    // 加载配置
    let config = Config::from_env();

    // 创建数据库连接池
    let pool = create_pool(&config).await?;

    // 运行数据库迁移
    sqlx::migrate!().run(&pool).await?;

    // 构建路由
    let app = create_router(pool);

    // ()(),启动！
    let listener = TcpListener::bind("0.0.0.0:3001").await?;
    info!("Server listening on http://{}", listener.local_addr()?); // 使用 info! 记录日志

    axum::serve(listener, app).await?;

    Ok(())
}