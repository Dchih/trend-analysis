use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use async_trait::async_trait;
use tokio_postgres::Client;

use crate::models::{
    keyword::KeywordRecord,
    overview::{ContentCreatorSummary, KeywordOverviewResponse, LatestContentItem, TimelinePoint, TopCreatorSummary},
    task::CollectionTaskRecord,
};

#[async_trait]
pub trait KeywordRepository: Send + Sync {
    async fn create_or_get_keyword(&self, keyword: &str) -> Result<KeywordRecord, String>;

    async fn create_collection_task(
        &self,
        keyword_id: u64,
        platform: &str,
        trigger_type: &str,
    ) -> Result<CollectionTaskRecord, String>;

    async fn fetch_overview(
        &self,
        keyword_id: u64,
        range: &str,
    ) -> Result<KeywordOverviewResponse, String>;

    async fn fetch_timeline(
        &self,
        keyword_id: u64,
        range: &str,
    ) -> Result<Vec<TimelinePoint>, String>;

    async fn fetch_top_creators(
        &self,
        keyword_id: u64,
        range: &str,
        limit: u64,
    ) -> Result<Vec<TopCreatorSummary>, String>;

    async fn fetch_latest_contents(
        &self,
        keyword_id: u64,
        range: &str,
        limit: u64,
    ) -> Result<Vec<LatestContentItem>, String>;
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

    async fn fetch_overview(
        &self,
        _keyword_id: u64,
        _range: &str,
    ) -> Result<KeywordOverviewResponse, String> {
        Ok(KeywordOverviewResponse {
            keyword: "ninja creami".to_string(),
            total_contents: 24,
            total_creators: 9,
            total_views: 215_000,
            last_collected_at: Some("2026-03-12T08:00:00Z".to_string()),
            trend_delta: 12.4,
        })
    }

    async fn fetch_timeline(
        &self,
        _keyword_id: u64,
        _range: &str,
    ) -> Result<Vec<TimelinePoint>, String> {
        Ok(vec![
            TimelinePoint {
                date: "2026-03-10".to_string(),
                new_content_count: 3,
                total_views: 45_000,
                active_creator_count: 2,
            },
            TimelinePoint {
                date: "2026-03-11".to_string(),
                new_content_count: 5,
                total_views: 88_000,
                active_creator_count: 4,
            },
        ])
    }

    async fn fetch_top_creators(
        &self,
        _keyword_id: u64,
        _range: &str,
        _limit: u64,
    ) -> Result<Vec<TopCreatorSummary>, String> {
        Ok(vec![TopCreatorSummary {
            creator_id: 1,
            display_name: "Kitchen Lab".to_string(),
            subscriber_count: 150_000,
            content_count: 4,
            total_views: 120_000,
            creator_score: 81.5,
        }])
    }

    async fn fetch_latest_contents(
        &self,
        _keyword_id: u64,
        _range: &str,
        _limit: u64,
    ) -> Result<Vec<LatestContentItem>, String> {
        Ok(vec![LatestContentItem {
            content_id: 1,
            title: "Ninja Creami recipes worth trying".to_string(),
            thumbnail_url: "https://img.youtube.com/vi/abc123/maxresdefault.jpg".to_string(),
            published_at: "2026-03-11T15:00:00Z".to_string(),
            view_count: 32_000,
            creator: ContentCreatorSummary {
                creator_id: 1,
                display_name: "Kitchen Lab".to_string(),
            },
        }])
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

    async fn fetch_overview(
        &self,
        keyword_id: u64,
        _range: &str,
    ) -> Result<KeywordOverviewResponse, String> {
        let row = self.client.query_one(
            "
            SELECT
                k.keyword,
                COUNT(DISTINCT c.id)::BIGINT AS total_contents,
                COUNT(DISTINCT c.creator_id)::BIGINT AS total_creators,
                COALESCE(SUM(c.view_count), 0)::BIGINT AS total_views,
                k.last_collected_at::TEXT
            FROM keywords k
            LEFT JOIN content_items c ON c.keyword_id = k.id
            WHERE k.id = $1
            GROUP BY k.id, k.keyword, k.last_collected_at
            ",
            &[&(keyword_id as i64)],
        ).await.map_err(|error| error.to_string())?;

        Ok(KeywordOverviewResponse {
            keyword: row.get(0),
            total_contents: row.get::<_, i64>(1) as u64,
            total_creators: row.get::<_, i64>(2) as u64,
            total_views: row.get::<_, i64>(3) as u64,
            last_collected_at: row.get(4),
            trend_delta: 0.0,
        })
    }

    async fn fetch_timeline(
        &self,
        keyword_id: u64,
        _range: &str,
    ) -> Result<Vec<TimelinePoint>, String> {
        let rows = self.client.query(
            "
            SELECT date::TEXT, new_content_count, total_views, active_creator_count
            FROM keyword_daily_stats
            WHERE keyword_id = $1
            ORDER BY date ASC
            ",
            &[&(keyword_id as i64)],
        ).await.map_err(|error| error.to_string())?;

        Ok(rows
            .into_iter()
            .map(|row| TimelinePoint {
                date: row.get(0),
                new_content_count: row.get::<_, i64>(1) as u64,
                total_views: row.get::<_, i64>(2) as u64,
                active_creator_count: row.get::<_, i64>(3) as u64,
            })
            .collect())
    }

    async fn fetch_top_creators(
        &self,
        keyword_id: u64,
        _range: &str,
        limit: u64,
    ) -> Result<Vec<TopCreatorSummary>, String> {
        let rows = self.client.query(
            "
            SELECT
                cr.id,
                cr.display_name,
                cr.subscriber_count::BIGINT,
                COUNT(c.id)::BIGINT AS content_count,
                COALESCE(SUM(c.view_count), 0)::BIGINT AS total_views,
                cr.creator_score
            FROM content_items c
            JOIN creators cr ON cr.id = c.creator_id
            WHERE c.keyword_id = $1
            GROUP BY cr.id, cr.display_name, cr.subscriber_count, cr.creator_score
            ORDER BY total_views DESC
            LIMIT $2
            ",
            &[&(keyword_id as i64), &(limit as i64)],
        ).await.map_err(|error| error.to_string())?;

        Ok(rows
            .into_iter()
            .map(|row| TopCreatorSummary {
                creator_id: row.get::<_, i64>(0) as u64,
                display_name: row.get(1),
                subscriber_count: row.get::<_, i64>(2) as u64,
                content_count: row.get::<_, i64>(3) as u64,
                total_views: row.get::<_, i64>(4) as u64,
                creator_score: row.get::<_, f64>(5),
            })
            .collect())
    }

    async fn fetch_latest_contents(
        &self,
        keyword_id: u64,
        _range: &str,
        limit: u64,
    ) -> Result<Vec<LatestContentItem>, String> {
        let rows = self.client.query(
            "
            SELECT
                c.id,
                c.title,
                COALESCE(c.thumbnail_url, ''),
                c.published_at::TEXT,
                c.view_count,
                cr.id,
                cr.display_name
            FROM content_items c
            JOIN creators cr ON cr.id = c.creator_id
            WHERE c.keyword_id = $1
            ORDER BY c.published_at DESC
            LIMIT $2
            ",
            &[&(keyword_id as i64), &(limit as i64)],
        ).await.map_err(|error| error.to_string())?;

        Ok(rows
            .into_iter()
            .map(|row| LatestContentItem {
                content_id: row.get::<_, i64>(0) as u64,
                title: row.get(1),
                thumbnail_url: row.get(2),
                published_at: row.get(3),
                view_count: row.get::<_, i64>(4) as u64,
                creator: ContentCreatorSummary {
                    creator_id: row.get::<_, i64>(5) as u64,
                    display_name: row.get(6),
                },
            })
            .collect())
    }
}
