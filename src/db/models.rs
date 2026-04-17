use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// 文章摘要（用于列表）
#[derive(Debug, Serialize, FromRow)]
pub struct PostSummary {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub published_at: DateTime<Utc>,
    #[serde(skip)]
    #[sqlx(skip)]   // 添加这一行
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PostDetail {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub published_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip)]
    #[sqlx(skip)]   // 添加这一行
    pub tags: Option<Vec<String>>,
}