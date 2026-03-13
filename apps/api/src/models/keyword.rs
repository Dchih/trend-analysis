use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct KeywordRecord {
    pub id: u64,
    pub keyword: String,
    pub status: String,
    pub created_at: String,
    pub last_collected_at: Option<String>,
}

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
    pub status: String,
    pub last_collected_at: Option<String>,
}
