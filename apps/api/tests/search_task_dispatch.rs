use std::sync::{Arc, Mutex};

use actix_web::{App, http::StatusCode, test, web};

#[path = "../src/app_state.rs"]
mod app_state;
#[path = "../src/models/mod.rs"]
mod models;
#[path = "../src/routes/mod.rs"]
mod routes;
#[path = "../src/services/mod.rs"]
mod services;

use app_state::AppState;
use routes::keywords::search_keyword;
use services::task_queue::{CollectTaskMessage, TaskQueue};

#[derive(Clone, Default)]
struct RecordingQueue {
    published: Arc<Mutex<Vec<CollectTaskMessage>>>,
}

impl RecordingQueue {
    fn published_len(&self) -> usize {
        self.published.lock().expect("poisoned mutex").len()
    }
}

impl TaskQueue for RecordingQueue {
    fn publish(&self, task: CollectTaskMessage) -> Result<(), String> {
        self.published
            .lock()
            .expect("poisoned mutex")
            .push(task);
        Ok(())
    }
}

#[actix_web::test]
async fn search_creates_a_pending_task_and_publishes_it() {
    let queue = RecordingQueue::default();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::new(Arc::new(queue.clone()))))
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
    assert_eq!(queue.published_len(), 1);
}
