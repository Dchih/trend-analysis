use std::sync::{Arc, Mutex};

use actix_web::{App, http::StatusCode, test, web};
use async_trait::async_trait;

#[path = "../src/app_state.rs"]
mod app_state;
#[path = "../src/models/mod.rs"]
mod models;
#[path = "../src/repositories/mod.rs"]
mod repositories;
#[path = "../src/routes/mod.rs"]
mod routes;
#[path = "../src/services/mod.rs"]
mod services;

use app_state::AppState;
use models::{
    keyword::KeywordRecord,
    overview::{
        ContentCreatorSummary, KeywordOverviewResponse, LatestContentItem, TimelinePoint,
        TopCreatorSummary,
    },
    task::CollectionTaskRecord,
};
use repositories::keywords::KeywordRepository;
use routes::keywords::search_keyword;
use services::task_queue::{CollectTaskMessage, TaskQueue};

#[derive(Clone, Default)]
struct RecordingRepository {
    saved_keywords: Arc<Mutex<Vec<String>>>,
    saved_tasks: Arc<Mutex<Vec<(u64, String, String)>>>,
}

impl RecordingRepository {
    fn keywords_len(&self) -> usize {
        self.saved_keywords.lock().expect("poisoned mutex").len()
    }

    fn tasks_len(&self) -> usize {
        self.saved_tasks.lock().expect("poisoned mutex").len()
    }
}

#[derive(Clone, Default)]
struct RecordingQueue {
    published: Arc<Mutex<Vec<CollectTaskMessage>>>,
}

impl RecordingQueue {
    fn published_len(&self) -> usize {
        self.published.lock().expect("poisoned mutex").len()
    }
}

#[async_trait]
impl KeywordRepository for RecordingRepository {
    async fn create_or_get_keyword(&self, keyword: &str) -> Result<KeywordRecord, String> {
        self.saved_keywords
            .lock()
            .expect("poisoned mutex")
            .push(keyword.to_string());

        Ok(KeywordRecord {
            id: 1,
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
        self.saved_tasks
            .lock()
            .expect("poisoned mutex")
            .push((keyword_id, platform.to_string(), trigger_type.to_string()));

        Ok(CollectionTaskRecord {
            id: 99,
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
            total_contents: 0,
            total_creators: 0,
            total_views: 0,
            last_collected_at: None,
            trend_delta: 0.0,
        })
    }

    async fn fetch_timeline(
        &self,
        _keyword_id: u64,
        _range: &str,
    ) -> Result<Vec<TimelinePoint>, String> {
        Ok(vec![])
    }

    async fn fetch_top_creators(
        &self,
        _keyword_id: u64,
        _range: &str,
        _limit: u64,
    ) -> Result<Vec<TopCreatorSummary>, String> {
        Ok(vec![])
    }

    async fn fetch_latest_contents(
        &self,
        _keyword_id: u64,
        _range: &str,
        _limit: u64,
    ) -> Result<Vec<LatestContentItem>, String> {
        Ok(vec![LatestContentItem {
            content_id: 0,
            title: String::new(),
            thumbnail_url: String::new(),
            published_at: String::new(),
            view_count: 0,
            creator: ContentCreatorSummary {
                creator_id: 0,
                display_name: String::new(),
            },
        }])
    }
}

#[async_trait]
impl TaskQueue for RecordingQueue {
    async fn publish(&self, task: CollectTaskMessage) -> Result<(), String> {
        self.published
            .lock()
            .expect("poisoned mutex")
            .push(task);
        Ok(())
    }
}

#[actix_web::test]
async fn search_creates_a_pending_task_and_publishes_it() {
    let repository = RecordingRepository::default();
    let queue = RecordingQueue::default();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::new(
                Arc::new(repository.clone()),
                Arc::new(queue.clone()),
            )))
            .service(search_keyword),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/api/v1/keywords/search")
        .set_json(serde_json::json!({
            "keyword": "ninja creami"
        }))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(response).await;
    assert_eq!(body["keyword"], "ninja creami");
    assert_eq!(body["task_status"], "pending");
    assert_eq!(repository.keywords_len(), 1);
    assert_eq!(repository.tasks_len(), 1);
    assert_eq!(queue.published_len(), 1);
}
