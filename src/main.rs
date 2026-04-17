mod config;
mod db;
mod error;
mod handlers;
mod routes;

use crate::config::Config;
use crate::db::create_pool;
use crate::routes::create_router;
use std::net::SocketAddr;
use axum::http::HeaderValue;
use axum::Router;
use axum::routing::get;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

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

    let cors = CorsLayer::new()
        .allow_origin("https://freebirdflyinthesky.netlify.app/".parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    // 构建路由
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(cors)
        .with_state(pool);

    // 1. 准备监听地址和端口
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // 2. 创建 TcpListener
    let listener = TcpListener::bind(addr).await?;

    // 3. 使用 axum::serve 启动服务
    info!("🚀 Server listening on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}