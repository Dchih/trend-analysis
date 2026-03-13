use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CollectionTaskRecord {
    pub id: u64,
    pub keyword_id: u64,
    pub platform: String,
    pub trigger_type: String,
    pub status: String,
    pub requested_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub error_message: Option<String>,
}
