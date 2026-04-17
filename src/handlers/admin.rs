use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
}

pub async fn create_post(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<Uuid>), (StatusCode, String)> {
    let id = Uuid::new_v4();

    // 使用 sqlx::query 代替 query! 宏
    sqlx::query(
        r#"
        INSERT INTO posts (id, slug, title, content, excerpt, published_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        "#,
    )
        .bind(id)
        .bind(&payload.slug)
        .bind(&payload.title)
        .bind(&payload.content)
        .bind(&payload.excerpt)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(id)))
}