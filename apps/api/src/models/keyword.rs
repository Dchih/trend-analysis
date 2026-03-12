use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct KeywordSearchRequest {
    pub keyword: String,
}

#[derive(Debug, Serialize)]
pub struct KeywordSearchResponse {
    pub id: u64,
    pub keyword: String,
    pub task_status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct KeywordHistoryItem {
    pub id: u64,
    pub keyword: &'static str,
}

#[derive(Debug, Serialize)]
pub struct KeywordStatusResponse {
    pub keyword_id: u64,
    pub status: &'static str,
    pub last_collected_at: Option<&'static str>,
}
