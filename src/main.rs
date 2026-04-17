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
use axum::{middleware, Router};
use axum::routing::{get, post};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use axum::{
    extract::Request,      // 使用这个具体类型
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use crate::handlers::admin::create_post;

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

    let admin_routes = Router::new()
        .route("/posts", post(create_post))
        .layer(middleware::from_fn(auth));

    // 构建路由
    let app = Router::new()
        .nest("/api/admin", admin_routes)
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

async fn auth(
    req: Request,           // 直接使用 axum::extract::Request
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let expected_token = std::env::var("ADMIN_TOKEN")
        .expect("ADMIN_TOKEN must be set");

    if auth_header != format!("Bearer {}", expected_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}