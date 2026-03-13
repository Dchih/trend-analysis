use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ContentItemRecord {
    pub id: u64,
    pub platform: String,
    pub platform_content_id: String,
    pub keyword_id: u64,
    pub creator_id: u64,
    pub title: String,
    pub description: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub published_at: String,
    pub view_count: u64,
    pub like_count: u64,
    pub comment_count: u64,
    pub engagement_score: f64,
    pub collected_at: String,
}
