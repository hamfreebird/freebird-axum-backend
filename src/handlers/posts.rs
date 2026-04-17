use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::{PostDetail, PostSummary};
use crate::error::{AppError, Result};

// 列表查询参数
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

// 获取文章列表
pub async fn list_posts(
    State(pool): State<PgPool>,
    Query(params): Query<ListQuery>,
) -> Result<Json<serde_json::Value>> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);

    // 查询总数
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts")
        .fetch_one(&pool)
        .await?;

    // 查询列表
    let posts = sqlx::query_as::<_, PostSummary>(
        r#"
        SELECT id, slug, title, excerpt, published_at
        FROM posts
        ORDER BY published_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
        .bind(limit)
        .bind(offset)
        .fetch_all(&pool)
        .await?;

    Ok(Json(serde_json::json!({
        "posts": posts,
        "total": total
    })))
}

// 获取单篇文章
pub async fn get_post(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<Json<PostDetail>> {
    let post = sqlx::query_as::<_, PostDetail>(
        r#"
        SELECT id, slug, title, content, excerpt, published_at, updated_at
        FROM posts
        WHERE slug = $1
        "#,
    )
        .bind(&slug)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with slug '{}' not found", slug)))?;

    // TODO: 添加相邻文章查询逻辑 (next/prev)
    // 可单独实现一个函数填充

    Ok(Json(post))
}