use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CreatorRecord {
    pub id: u64,
    pub platform: String,
    pub platform_creator_id: String,
    pub display_name: String,
    pub handle: Option<String>,
    pub avatar_url: Option<String>,
    pub subscriber_count: u64,
    pub video_count: u64,
    pub creator_score: f64,
}
