use axum::{
    routing::get,
    Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::handlers::health;
use crate::handlers::posts;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/health", get(health::health_check))
        .route("/api/posts", get(posts::list_posts))
        .route("/api/posts/:slug", get(posts::get_post))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}