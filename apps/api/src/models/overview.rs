use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct KeywordOverviewResponse {
    pub keyword: String,
    pub total_contents: u64,
    pub total_creators: u64,
    pub total_views: u64,
    pub last_collected_at: Option<String>,
    pub trend_delta: f64,
}

#[derive(Debug, Serialize)]
pub struct TimelinePoint {
    pub date: String,
    pub new_content_count: u64,
    pub total_views: u64,
    pub active_creator_count: u64,
}

#[derive(Debug, Serialize)]
pub struct TopCreatorSummary {
    pub creator_id: u64,
    pub display_name: String,
    pub subscriber_count: u64,
    pub content_count: u64,
    pub total_views: u64,
    pub creator_score: f64,
}

#[derive(Debug, Serialize)]
pub struct ContentCreatorSummary {
    pub creator_id: u64,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct LatestContentItem {
    pub content_id: u64,
    pub title: String,
    pub thumbnail_url: String,
    pub published_at: String,
    pub view_count: u64,
    pub creator: ContentCreatorSummary,
}
