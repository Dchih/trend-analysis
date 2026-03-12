use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use async_trait::async_trait;
use tokio_postgres::Client;

use crate::models::{keyword::KeywordRecord, task::CollectionTaskRecord};

#[async_trait]
pub trait KeywordRepository: Send + Sync {
    async fn create_or_get_keyword(&self, keyword: &str) -> Result<KeywordRecord, String>;

    async fn create_collection_task(
        &self,
        keyword_id: u64,
        platform: &str,
        trigger_type: &str,
    ) -> Result<CollectionTaskRecord, String>;
}

#[derive(Clone)]
pub struct InMemoryKeywordRepository {
    next_keyword_id: Arc<AtomicU64>,
    next_task_id: Arc<AtomicU64>,
}

impl Default for InMemoryKeywordRepository {
    fn default() -> Self {
        Self {
            next_keyword_id: Arc::new(AtomicU64::new(1)),
            next_task_id: Arc::new(AtomicU64::new(1)),
        }
    }
}

#[async_trait]
impl KeywordRepository for InMemoryKeywordRepository {
    async fn create_or_get_keyword(&self, keyword: &str) -> Result<KeywordRecord, String> {
        Ok(KeywordRecord {
            id: self.next_keyword_id.fetch_add(1, Ordering::SeqCst),
            keyword: keyword.to_string(),
            status: "active".to_string(),
            created_at: "2026-03-12T10:00:00Z".to_string(),
            last_collected_at: None,
        })
    }

    async fn create_collection_task(
        &self,
        keyword_id: u64,
        platform: &str,
        trigger_type: &str,
    ) -> Result<CollectionTaskRecord, String> {
        Ok(CollectionTaskRecord {
            id: self.next_task_id.fetch_add(1, Ordering::SeqCst),
            keyword_id,
            platform: platform.to_string(),
            trigger_type: trigger_type.to_string(),
            status: "pending".to_string(),
            requested_at: "2026-03-12T10:00:00Z".to_string(),
            started_at: None,
            finished_at: None,
            error_message: None,
        })
    }
}

pub struct PgKeywordRepository {
    client: Client,
}

impl PgKeywordRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl KeywordRepository for PgKeywordRepository {
    async fn create_or_get_keyword(&self, keyword: &str) -> Result<KeywordRecord, String> {
        let row = self
            .client
            .query_one(
                "
                INSERT INTO keywords (keyword)
                VALUES ($1)
                ON CONFLICT (keyword)
                DO UPDATE SET keyword = EXCLUDED.keyword
                RETURNING id, keyword, status, created_at::TEXT, last_collected_at::TEXT
                ",
                &[&keyword],
            )
            .await
            .map_err(|error| error.to_string())?;

        Ok(KeywordRecord {
            id: row.get::<_, i64>(0) as u64,
            keyword: row.get(1),
            status: row.get(2),
            created_at: row.get(3),
            last_collected_at: row.get(4),
        })
    }

    async fn create_collection_task(
        &self,
        keyword_id: u64,
        platform: &str,
        trigger_type: &str,
    ) -> Result<CollectionTaskRecord, String> {
        let row = self
            .client
            .query_one(
                "
                INSERT INTO collection_tasks (keyword_id, platform, trigger_type, status)
                VALUES ($1, $2, $3, 'pending')
                RETURNING
                    id,
                    keyword_id,
                    platform,
                    trigger_type,
                    status,
                    requested_at::TEXT,
                    started_at::TEXT,
                    finished_at::TEXT,
                    error_message
                ",
                &[&(keyword_id as i64), &platform, &trigger_type],
            )
            .await
            .map_err(|error| error.to_string())?;

        Ok(CollectionTaskRecord {
            id: row.get::<_, i64>(0) as u64,
            keyword_id: row.get::<_, i64>(1) as u64,
            platform: row.get(2),
            trigger_type: row.get(3),
            status: row.get(4),
            requested_at: row.get(5),
            started_at: row.get(6),
            finished_at: row.get(7),
            error_message: row.get(8),
        })
    }
}
