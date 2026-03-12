use std::sync::Arc;

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
use routes::overview::{keyword_overview, keyword_timeline, latest_contents, top_creators};
use services::task_queue::NoopQueue;

#[derive(Clone)]
struct OverviewRepoStub;

#[async_trait]
impl KeywordRepository for OverviewRepoStub {
    async fn create_or_get_keyword(&self, _keyword: &str) -> Result<KeywordRecord, String> {
        Ok(KeywordRecord {
            id: 42,
            keyword: "cold brew maker".to_string(),
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
            id: 1,
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
        keyword_id: u64,
        _range: &str,
    ) -> Result<KeywordOverviewResponse, String> {
        Ok(KeywordOverviewResponse {
            keyword: format!("keyword-{keyword_id}"),
            total_contents: 3,
            total_creators: 2,
            total_views: 4500,
            last_collected_at: Some("2026-03-12T09:00:00Z".to_string()),
            trend_delta: 5.5,
        })
    }

    async fn fetch_timeline(
        &self,
        _keyword_id: u64,
        _range: &str,
    ) -> Result<Vec<TimelinePoint>, String> {
        Ok(vec![TimelinePoint {
            date: "2026-03-11".to_string(),
            new_content_count: 1,
            total_views: 4500,
            active_creator_count: 1,
        }])
    }

    async fn fetch_top_creators(
        &self,
        _keyword_id: u64,
        _range: &str,
        _limit: u64,
    ) -> Result<Vec<TopCreatorSummary>, String> {
        Ok(vec![TopCreatorSummary {
            creator_id: 8,
            display_name: "Brew Lab".to_string(),
            subscriber_count: 12000,
            content_count: 2,
            total_views: 4500,
            creator_score: 77.7,
        }])
    }

    async fn fetch_latest_contents(
        &self,
        _keyword_id: u64,
        _range: &str,
        _limit: u64,
    ) -> Result<Vec<LatestContentItem>, String> {
        Ok(vec![LatestContentItem {
            content_id: 9,
            title: "Cold brew maker review".to_string(),
            thumbnail_url: "https://img.youtube.com/vi/demo/maxresdefault.jpg".to_string(),
            published_at: "2026-03-11T15:00:00Z".to_string(),
            view_count: 4500,
            creator: ContentCreatorSummary {
                creator_id: 8,
                display_name: "Brew Lab".to_string(),
            },
        }])
    }
}

#[actix_web::test]
async fn overview_routes_use_repository_data() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::new(
                Arc::new(OverviewRepoStub),
                Arc::new(NoopQueue),
            )))
            .service(keyword_overview)
            .service(keyword_timeline)
            .service(top_creators)
            .service(latest_contents),
    )
    .await;

    let overview_response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/v1/keywords/42/overview?range=30d")
            .to_request(),
    )
    .await;
    assert_eq!(overview_response.status(), StatusCode::OK);
    let overview_body: serde_json::Value = test::read_body_json(overview_response).await;
    assert_eq!(overview_body["keyword"], "keyword-42");
    assert_eq!(overview_body["total_views"], 4500);

    let timeline_response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/v1/keywords/42/timeline?range=30d")
            .to_request(),
    )
    .await;
    assert_eq!(timeline_response.status(), StatusCode::OK);

    let creators_response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/v1/keywords/42/creators/top?range=30d&limit=10")
            .to_request(),
    )
    .await;
    assert_eq!(creators_response.status(), StatusCode::OK);

    let latest_response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/v1/keywords/42/contents/latest?range=30d&limit=20")
            .to_request(),
    )
    .await;
    assert_eq!(latest_response.status(), StatusCode::OK);
}
